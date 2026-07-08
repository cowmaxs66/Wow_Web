use crate::config::{AgentConfig, ConfigError, default_config_path, ensure_config_exists};
use shared_types::ClientScriptDeployBundle;
use std::fs;
use std::path::{Component, Path, PathBuf};
use thiserror::Error;

const MAX_LUA_CONTENT_CHARS: usize = 120_000;
const MAX_MANIFEST_CONTENT_CHARS: usize = 20_000;
const DEFAULT_PERMISSIONS: &[&str] = &["host.log", "config.read", "dm.access"];

#[derive(Debug, Error)]
pub enum ScriptDeployError {
    #[error("解析 script.deploy_bundle payload 失败：{0}")]
    Payload(String),
    #[error("script.deploy_bundle payload 无效：{0}")]
    Validate(String),
    #[error("写入脚本包失败：{0}")]
    Io(String),
    #[error("应用 Client 配置失败：{0}")]
    Config(#[from] ConfigError),
    #[error("脚本安全校验失败：{0}")]
    Script(String),
    #[error("脚本执行失败：{0}")]
    Run(String),
}

pub fn deploy_script_bundle(payload: &serde_json::Value) -> Result<String, ScriptDeployError> {
    let bundle: ClientScriptDeployBundle = serde_json::from_value(payload.clone())
        .map_err(|error| ScriptDeployError::Payload(error.to_string()))?;
    let root = client_root_from_config();
    deploy_script_bundle_to_root(&root, bundle)
}

fn deploy_script_bundle_to_root(
    root: &Path,
    bundle: ClientScriptDeployBundle,
) -> Result<String, ScriptDeployError> {
    validate_bundle(&bundle)?;

    let lua_path = safe_relative_file(root, &bundle.bootstrap_path, "lua")?;
    let manifest_path = match bundle.manifest_path.as_deref() {
        Some(path) if !path.trim().is_empty() => Some(safe_relative_file(root, path, "json")?),
        _ => None,
    };
    write_text_file(&lua_path, &bundle.lua_content)?;
    if let Some(path) = &manifest_path
        && let Some(content) = bundle.manifest_content.as_deref()
    {
        write_text_file(path, content)?;
    }

    let mut summary = format!(
        "脚本包已写入：script={} lua={} manifest={}",
        bundle.bootstrap_name,
        bundle.bootstrap_path,
        bundle.manifest_path.as_deref().unwrap_or("未启用")
    );

    if bundle.activate {
        let next_config = activate_bundle(&bundle)?;
        summary.push_str("；已切换为当前 bootstrap");

        if bundle.run_after_deploy {
            let result = crate::agent::run_once(&next_config)
                .map_err(|error| ScriptDeployError::Run(error.to_string()))?;
            let script = result
                .envelope
                .data
                .current_script
                .as_deref()
                .unwrap_or("无");
            summary.push_str(&format!("；已执行 script={script}"));
            if let Some(report) = result.script_report.as_ref() {
                summary.push('\n');
                summary.push_str(&report.receipt_lines().join("\n"));
            }
        }
    } else if bundle.run_after_deploy {
        return Err(ScriptDeployError::Validate(
            "run_after_deploy=true 时必须同时 activate=true".to_string(),
        ));
    }

    Ok(summary)
}

fn validate_bundle(bundle: &ClientScriptDeployBundle) -> Result<(), ScriptDeployError> {
    if bundle.bootstrap_name.trim().is_empty() {
        return Err(ScriptDeployError::Validate(
            "bootstrap_name 不能为空".to_string(),
        ));
    }

    if bundle.bootstrap_path.trim().is_empty() {
        return Err(ScriptDeployError::Validate(
            "bootstrap_path 不能为空".to_string(),
        ));
    }

    if bundle.lua_content.trim().is_empty() {
        return Err(ScriptDeployError::Validate(
            "lua_content 不能为空".to_string(),
        ));
    }

    if bundle.lua_content.chars().count() > MAX_LUA_CONTENT_CHARS {
        return Err(ScriptDeployError::Validate(format!(
            "lua_content 不能超过 {MAX_LUA_CONTENT_CHARS} 字符"
        )));
    }

    if bundle.security_enabled
        && bundle
            .manifest_path
            .as_deref()
            .is_none_or(|value| value.trim().is_empty())
    {
        return Err(ScriptDeployError::Validate(
            "启用安全门时 manifest_path 不能为空".to_string(),
        ));
    }

    if bundle.security_enabled
        && bundle
            .manifest_content
            .as_deref()
            .is_none_or(|value| value.trim().is_empty())
    {
        return Err(ScriptDeployError::Validate(
            "启用安全门时 manifest_content 不能为空".to_string(),
        ));
    }

    if bundle
        .manifest_content
        .as_deref()
        .unwrap_or_default()
        .chars()
        .count()
        > MAX_MANIFEST_CONTENT_CHARS
    {
        return Err(ScriptDeployError::Validate(format!(
            "manifest_content 不能超过 {MAX_MANIFEST_CONTENT_CHARS} 字符"
        )));
    }

    Ok(())
}

fn activate_bundle(bundle: &ClientScriptDeployBundle) -> Result<AgentConfig, ScriptDeployError> {
    let config_path = default_config_path();
    ensure_config_exists(&config_path)?;
    let mut config = AgentConfig::load_file_from_path(&config_path)?;

    config.lua.enabled = true;
    config.lua.bootstrap_name = bundle.bootstrap_name.trim().to_string();
    config.lua.bootstrap_path = PathBuf::from(bundle.bootstrap_path.trim());
    config.script_security.enabled = bundle.security_enabled;

    if let Some(manifest_path) = bundle.manifest_path.as_deref()
        && !manifest_path.trim().is_empty()
    {
        config.script_security.manifest_path = PathBuf::from(manifest_path.trim());
    }

    if let Some(public_key) = bundle.trusted_signer_public_key.as_deref() {
        let public_key = public_key.trim();
        if !public_key.is_empty() {
            config.script_security.trusted_signer_public_key = public_key.to_string();
        }
    }

    config.script_security.allowed_permissions = bundle
        .allowed_permissions
        .as_ref()
        .filter(|permissions| !permissions.is_empty())
        .map(|permissions| normalize_permissions(permissions))
        .unwrap_or_else(default_permissions);

    // 启用安全门时先做完整验证；内部测试模式下直接保存，便于热推脚本快速试错。
    // 输入：Server 下发的 Lua + manifest + 可选权限/公钥。
    // 输出：配置写回 client-agent.toml。
    // 边界：security_enabled=false 时不校验 hash/签名，适合内网测试，不适合公网生产。
    if config.script_security.enabled {
        crate::script::ScriptSource::load_bootstrap(&config)
            .map_err(|error| ScriptDeployError::Script(error.to_string()))?;
    }
    config.save_to_path(&config_path)?;

    Ok(config)
}

fn normalize_permissions(permissions: &[String]) -> Vec<String> {
    permissions
        .iter()
        .map(|permission| permission.trim().to_string())
        .filter(|permission| !permission.is_empty())
        .collect()
}

fn default_permissions() -> Vec<String> {
    DEFAULT_PERMISSIONS
        .iter()
        .map(|permission| permission.to_string())
        .collect()
}

fn client_root_from_config() -> PathBuf {
    let config_path = default_config_path();
    if let Some(config_dir) = config_path.parent()
        && config_dir.file_name().is_some_and(|name| name == "config")
        && let Some(root) = config_dir.parent()
    {
        return root.to_path_buf();
    }

    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn safe_relative_file(
    root: &Path,
    relative_path: &str,
    extension: &str,
) -> Result<PathBuf, ScriptDeployError> {
    let trimmed = relative_path.trim();
    let raw_path = Path::new(trimmed);
    if raw_path.is_absolute() {
        return Err(ScriptDeployError::Validate(format!(
            "{trimmed} 不能是绝对路径"
        )));
    }

    let mut normalized = PathBuf::new();
    for component in raw_path.components() {
        match component {
            Component::Normal(part) => normalized.push(part),
            _ => {
                return Err(ScriptDeployError::Validate(format!(
                    "{trimmed} 只能使用普通相对路径"
                )));
            }
        }
    }

    if !normalized.starts_with("scripts") {
        return Err(ScriptDeployError::Validate(format!(
            "{trimmed} 必须位于 scripts 目录"
        )));
    }

    let actual_extension = normalized
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    if !actual_extension.eq_ignore_ascii_case(extension) {
        return Err(ScriptDeployError::Validate(format!(
            "{trimmed} 必须是 .{extension} 文件"
        )));
    }

    Ok(root.join(normalized))
}

fn write_text_file(path: &Path, content: &str) -> Result<(), ScriptDeployError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| ScriptDeployError::Io(error.to_string()))?;
    }

    fs::write(path, content).map_err(|error| ScriptDeployError::Io(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn safe_relative_file_rejects_path_traversal() {
        let root = PathBuf::from("C:/wow");
        let error = safe_relative_file(&root, "../bootstrap.lua", "lua")
            .expect_err("path traversal must fail");

        assert!(error.to_string().contains("普通相对路径"));
    }

    #[test]
    fn deploy_script_bundle_writes_files_without_activation() {
        let root = unique_temp_dir("script-deploy");
        let bundle = ClientScriptDeployBundle {
            bootstrap_name: "bootstrap".to_string(),
            bootstrap_path: "scripts/pushed.lua".to_string(),
            lua_content: "return 'ok'".to_string(),
            manifest_path: None,
            manifest_content: None,
            security_enabled: false,
            allowed_permissions: None,
            trusted_signer_public_key: None,
            activate: false,
            run_after_deploy: false,
        };

        let summary =
            deploy_script_bundle_to_root(&root, bundle).expect("bundle must write to scripts");

        assert!(summary.contains("脚本包已写入"));
        assert_eq!(
            fs::read_to_string(root.join("scripts/pushed.lua")).expect("lua must exist"),
            "return 'ok'"
        );
        assert!(!root.join("scripts/pushed.manifest.json").exists());

        let _ = fs::remove_dir_all(root);
    }

    fn unique_temp_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock must be valid")
            .as_nanos();

        std::env::temp_dir().join(format!("wow-{name}-{}-{nanos}", std::process::id()))
    }
}
