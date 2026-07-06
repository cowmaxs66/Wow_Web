mod error;
mod hash;
mod manifest;
mod permissions;
mod signature;

pub use error::ScriptError;
pub use permissions::{
    PERMISSION_CONFIG_READ, PERMISSION_DM_ACCESS, PERMISSION_HOST_LOG, ScriptPermissions,
};

use crate::config::AgentConfig;
use hash::sha256_hex;
use manifest::ScriptManifest;
use signature::verify_manifest_signature;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptSource {
    pub name: String,
    pub path: PathBuf,
    pub content: String,
    pub permissions: ScriptPermissions,
}

impl ScriptSource {
    pub fn load_bootstrap(config: &AgentConfig) -> Result<Self, ScriptError> {
        if config.script_security.enabled {
            return load_secure_bootstrap(config);
        }

        let path = resolve_module_path(&config.lua.bootstrap_path);
        let content = fs::read_to_string(&path).map_err(|error| ScriptError::read(&path, error))?;

        // 脚本加载只负责把受控路径转换成脚本文本，不注册能力、不执行代码。
        // 输入：配置中的 bootstrap_name 与 bootstrap_path。
        // 输出：带名称、绝对路径和文本内容的 ScriptSource。
        // 边界：相对路径固定按 client-agent 模块根目录解析，避免从 workspace 根目录运行时路径漂移。
        Ok(Self {
            name: config.lua.bootstrap_name.clone(),
            path,
            content,
            permissions: ScriptPermissions::allow_all(),
        })
    }
}

fn load_secure_bootstrap(config: &AgentConfig) -> Result<ScriptSource, ScriptError> {
    let manifest_path = resolve_module_path(&config.script_security.manifest_path);
    let manifest = ScriptManifest::load(&manifest_path)?;
    manifest.validate_script_id(&manifest_path, &config.lua.bootstrap_name)?;
    verify_manifest_signature(
        &manifest_path,
        &manifest,
        &config.script_security.trusted_signer_public_key,
    )?;

    let entry_path = manifest.entry_path(&manifest_path)?;
    let configured_path = resolve_module_path(&config.lua.bootstrap_path);
    ensure_manifest_entry_matches_config(&manifest_path, &entry_path, &configured_path)?;

    let content =
        fs::read_to_string(&entry_path).map_err(|error| ScriptError::read(&entry_path, error))?;
    verify_script_hash(&entry_path, &manifest.sha256, content.as_bytes())?;
    verify_permissions(
        &manifest_path,
        &manifest.permissions,
        &config.script_security.allowed_permissions,
    )?;

    // P5 安全加载顺序：manifest -> entry 一致性 -> hash -> 权限白名单 -> ScriptSource。
    // 输入：TOML 安全配置、manifest JSON、Lua 文件。
    // 输出：带明确权限集合的 ScriptSource。
    // 边界：不校验通过就不把脚本文本交给 Lua VM。
    Ok(ScriptSource {
        name: manifest.script_id,
        path: entry_path,
        content,
        permissions: ScriptPermissions::from_list(manifest.permissions),
    })
}

fn resolve_module_path(path: &PathBuf) -> PathBuf {
    if path.is_absolute() {
        return path.clone();
    }

    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path)
}

fn ensure_manifest_entry_matches_config(
    manifest_path: &Path,
    entry_path: &Path,
    configured_path: &Path,
) -> Result<(), ScriptError> {
    let entry_canonical =
        fs::canonicalize(entry_path).map_err(|error| ScriptError::read(entry_path, error))?;
    let configured_canonical = fs::canonicalize(configured_path)
        .map_err(|error| ScriptError::read(configured_path, error))?;

    if entry_canonical != configured_canonical {
        return Err(ScriptError::validate(
            manifest_path,
            "manifest entry 必须与 lua.bootstrap_path 指向同一脚本",
        ));
    }

    Ok(())
}

fn verify_script_hash(
    script_path: &Path,
    expected_hash: &str,
    content: &[u8],
) -> Result<(), ScriptError> {
    let actual_hash = sha256_hex(content);
    if actual_hash != expected_hash.to_ascii_lowercase() {
        return Err(ScriptError::security(
            script_path,
            format!("sha256 不匹配，期望 {expected_hash}，实际 {actual_hash}"),
        ));
    }

    Ok(())
}

fn verify_permissions(
    manifest_path: &Path,
    requested_permissions: &[String],
    allowed_permissions: &[String],
) -> Result<(), ScriptError> {
    let allowed: BTreeSet<_> = allowed_permissions
        .iter()
        .map(|permission| permission.trim().to_string())
        .collect();

    for permission in requested_permissions {
        if !allowed.contains(permission) {
            return Err(ScriptError::security(
                manifest_path,
                format!("权限 {permission} 未在配置白名单中"),
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests;
