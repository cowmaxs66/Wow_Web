use super::{AgentConfig, ConfigError, default_config_path, ensure_config_exists};
use shared_types::{ClientConfigPatch, ClientScriptSecurityConfigPatch};
use std::collections::BTreeSet;
use std::path::PathBuf;
use thiserror::Error;

const PERMISSION_HOST_LOG: &str = "host.log";
const PERMISSION_CONFIG_READ: &str = "config.read";
const PERMISSION_DM_ACCESS: &str = "dm.access";

#[derive(Debug, Error)]
pub enum ConfigPatchError {
    #[error("{0}")]
    Config(#[from] ConfigError),
    #[error("解析 config.apply payload 失败：{0}")]
    Payload(String),
    #[error("config.apply payload 没有可应用的配置项")]
    Empty,
    #[error("config.apply payload 包含不支持的脚本权限：{0}")]
    UnsupportedPermission(String),
}

pub fn apply_remote_patch(payload: &serde_json::Value) -> Result<String, ConfigPatchError> {
    let patch = parse_patch(payload)?;
    let config_path = default_config_path();
    ensure_config_exists(&config_path)?;

    let mut config = AgentConfig::load_file_from_path(&config_path)?;
    let mut changes = Vec::new();
    apply_patch(&mut config, patch, &mut changes)?;
    config.save_to_path(&config_path)?;

    // 远程设置只写入本机 TOML，当前命令回执仍使用旧连接，下一轮 monitor 会重新读取配置。
    // 输入：Server 下发的 config.apply payload。
    // 输出：写回后的配置摘要。
    // 边界：不允许修改 client.id，避免 Client 换身份后 Server 回执和历史记录断裂。
    Ok(format!(
        "Client 配置已应用：{}；配置文件={}",
        changes.join(", "),
        config_path.display()
    ))
}

fn parse_patch(payload: &serde_json::Value) -> Result<ClientConfigPatch, ConfigPatchError> {
    let patch: ClientConfigPatch = serde_json::from_value(payload.clone()).map_err(|error| {
        ConfigPatchError::Payload(format!("字段格式不符合 ClientConfigPatch：{error}"))
    })?;

    if patch.is_empty() {
        return Err(ConfigPatchError::Empty);
    }

    Ok(patch)
}

fn apply_patch(
    config: &mut AgentConfig,
    patch: ClientConfigPatch,
    changes: &mut Vec<String>,
) -> Result<(), ConfigPatchError> {
    if let Some(value) = patch.lua.bootstrap_name {
        config.lua.bootstrap_name = value.trim().to_string();
        changes.push("lua.bootstrap_name".to_string());
    }

    if let Some(value) = patch.lua.bootstrap_path {
        config.lua.bootstrap_path = PathBuf::from(value.trim());
        changes.push("lua.bootstrap_path".to_string());
    }

    if let Some(value) = patch.lua.instruction_limit {
        config.lua.instruction_limit = value;
        changes.push("lua.instruction_limit".to_string());
    }

    apply_script_security_patch(config, patch.script_security, changes)?;

    if let Some(value) = patch.dm.bridge_path {
        config.dm.bridge_path = PathBuf::from(value.trim());
        changes.push("dm.bridge_path".to_string());
    }

    if let Some(value) = patch.server.enabled {
        config.server.enabled = value;
        changes.push("server.enabled".to_string());
    }

    if let Some(value) = patch.server.host {
        config.server.host = value.trim().to_string();
        changes.push("server.host".to_string());
    }

    if let Some(value) = patch.server.port {
        config.server.port = value;
        changes.push("server.port".to_string());
    }

    if let Some(value) = patch.server.status_path {
        config.server.status_path = value.trim().to_string();
        changes.push("server.status_path".to_string());
    }

    if let Some(value) = patch.server.connect_timeout_ms {
        config.server.connect_timeout_ms = value;
        changes.push("server.connect_timeout_ms".to_string());
    }

    Ok(())
}

fn apply_script_security_patch(
    config: &mut AgentConfig,
    patch: ClientScriptSecurityConfigPatch,
    changes: &mut Vec<String>,
) -> Result<(), ConfigPatchError> {
    if let Some(value) = patch.enabled {
        config.script_security.enabled = value;
        changes.push("script_security.enabled".to_string());
    }

    if let Some(value) = patch.manifest_path {
        config.script_security.manifest_path = PathBuf::from(value.trim());
        changes.push("script_security.manifest_path".to_string());
    }

    if let Some(value) = patch.trusted_signer_public_key {
        config.script_security.trusted_signer_public_key = value.trim().to_string();
        changes.push("script_security.trusted_signer_public_key".to_string());
    }

    if let Some(value) = patch.allowed_permissions {
        config.script_security.allowed_permissions = normalize_permissions(value)?;
        changes.push("script_security.allowed_permissions".to_string());
    }

    Ok(())
}

fn normalize_permissions(permissions: Vec<String>) -> Result<Vec<String>, ConfigPatchError> {
    let mut normalized = BTreeSet::new();

    for permission in permissions {
        let permission = permission.trim().to_string();
        if permission.is_empty() {
            continue;
        }

        if !is_supported_permission(&permission) {
            return Err(ConfigPatchError::UnsupportedPermission(permission));
        }

        normalized.insert(permission);
    }

    Ok(normalized.into_iter().collect())
}

fn is_supported_permission(permission: &str) -> bool {
    matches!(
        permission,
        PERMISSION_HOST_LOG | PERMISSION_CONFIG_READ | PERMISSION_DM_ACCESS
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn normalizes_known_permissions() {
        let permissions = normalize_permissions(vec![
            "dm.access".to_string(),
            " host.log ".to_string(),
            "dm.access".to_string(),
        ])
        .expect("permissions must normalize");

        assert_eq!(
            permissions,
            vec!["dm.access".to_string(), "host.log".to_string()]
        );
    }

    #[test]
    fn rejects_unknown_permission() {
        let error = normalize_permissions(vec!["shell.exec".to_string()])
            .expect_err("unknown permission must fail");

        assert!(matches!(
            error,
            ConfigPatchError::UnsupportedPermission(permission) if permission == "shell.exec"
        ));
    }

    #[test]
    fn applies_patch_to_config_file() {
        let dir = unique_temp_dir("config-patch");
        let path = dir.join("client-agent.toml");
        fs::create_dir_all(&dir).expect("test dir must exist");
        fs::write(&path, include_str!("../../config/client-agent.toml"))
            .expect("test config must write");

        let mut config = AgentConfig::load_file_from_path(&path).expect("config must load");
        let patch = serde_json::json!({
            "lua": {
                "instruction_limit": 200000
            },
            "script_security": {
                "allowed_permissions": ["host.log", "config.read", "dm.access"]
            },
            "dm": {
                "bridge_path": "dm-bridge/DmBridge.dll"
            },
            "server": {
                "enabled": true,
                "host": "127.0.0.1",
                "port": 18180
            }
        });
        let patch = parse_patch(&patch).expect("patch must parse");
        let mut changes = Vec::new();

        apply_patch(&mut config, patch, &mut changes).expect("patch must apply");
        config.save_to_path(&path).expect("config must save");
        let saved = AgentConfig::load_file_from_path(&path).expect("saved config must load");

        assert_eq!(saved.lua.instruction_limit, 200000);
        assert_eq!(
            saved.script_security.allowed_permissions,
            vec![
                "config.read".to_string(),
                "dm.access".to_string(),
                "host.log".to_string()
            ]
        );
        assert_eq!(
            saved.dm.bridge_path,
            PathBuf::from("dm-bridge/DmBridge.dll")
        );
        assert_eq!(saved.server.port, 18180);
        assert!(changes.contains(&"server.enabled".to_string()));

        let _ = fs::remove_dir_all(dir);
    }

    fn unique_temp_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock must be valid")
            .as_nanos();

        std::env::temp_dir().join(format!("wow-{name}-{}-{nanos}", std::process::id()))
    }
}
