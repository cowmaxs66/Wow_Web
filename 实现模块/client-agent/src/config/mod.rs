mod error;
mod patch;
mod path;

pub use error::ConfigError;
pub(crate) use patch::apply_remote_patch;
pub use path::{current_exe_dir, default_config_path};

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_CONFIG_TEMPLATE: &str = include_str!("../../config/client-agent.toml");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentConfig {
    pub client: ClientConfig,
    pub lua: LuaConfig,
    pub script_security: ScriptSecurityConfig,
    pub dm: DmConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClientConfig {
    pub id: String,
    #[serde(default = "default_client_display_name")]
    pub display_name: String,
    #[serde(default = "default_client_group")]
    pub group: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LuaConfig {
    pub bootstrap_name: String,
    pub bootstrap_path: PathBuf,
    pub instruction_limit: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScriptSecurityConfig {
    pub enabled: bool,
    pub manifest_path: PathBuf,
    pub trusted_signer_public_key: String,
    pub allowed_permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DmConfig {
    pub bridge_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServerConfig {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub status_path: String,
    pub connect_timeout_ms: u64,
}

impl AgentConfig {
    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let mut config = Self::load_file_from_path(path)?;
        config.apply_env_overrides(path)?;
        config.validate(path)?;
        Ok(config)
    }

    pub fn load_file_from_path(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let content = fs::read_to_string(path).map_err(|error| ConfigError::read(path, error))?;

        // 使用 TOML 只承载开发期必要配置，避免一开始引入复杂配置层级。
        // 输入：client-agent.toml 文本内容。
        // 输出：强类型 AgentConfig。
        // 边界：配置结构不匹配时立即失败，不生成隐式默认值。
        let config: Self =
            toml::from_str(&content).map_err(|error| ConfigError::parse(path, error))?;
        config.validate(path)?;
        Ok(config)
    }

    pub fn save_to_path(&self, path: impl AsRef<Path>) -> Result<(), ConfigError> {
        let path = path.as_ref();
        self.validate(path)?;
        let content =
            toml::to_string_pretty(self).map_err(|error| ConfigError::serialize(path, error))?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| ConfigError::write(path, error))?;
        }

        fs::write(path, content).map_err(|error| ConfigError::write(path, error))
    }

    pub fn get_value(&self, key: &str) -> Option<String> {
        match key {
            "client.id" => Some(self.client.id.clone()),
            "client.display_name" => Some(self.client.display_name.clone()),
            "client.group" => Some(self.client.group.clone()),
            "client.tags" => Some(self.client.tags.join(",")),
            "lua.bootstrap_name" => Some(self.lua.bootstrap_name.clone()),
            "lua.bootstrap_path" => Some(self.lua.bootstrap_path.display().to_string()),
            "script_security.enabled" => Some(self.script_security.enabled.to_string()),
            "dm.bridge_path" => Some(self.dm.bridge_path.display().to_string()),
            "server.enabled" => Some(self.server.enabled.to_string()),
            "server.host" => Some(self.server.host.clone()),
            "server.port" => Some(self.server.port.to_string()),
            "server.status_path" => Some(self.server.status_path.clone()),
            _ => None,
        }
    }

    fn apply_env_overrides(&mut self, path: &Path) -> Result<(), ConfigError> {
        if let Ok(value) = std::env::var("CLIENT_AGENT_SERVER_ENABLED") {
            self.server.enabled = parse_bool(path, "CLIENT_AGENT_SERVER_ENABLED", &value)?;
        }

        if let Ok(value) = std::env::var("CLIENT_AGENT_DISPLAY_NAME") {
            self.client.display_name = value;
        }

        if let Ok(value) = std::env::var("CLIENT_AGENT_GROUP") {
            self.client.group = value;
        }

        if let Ok(value) = std::env::var("CLIENT_AGENT_TAGS") {
            self.client.tags = parse_tags(&value);
        }

        if let Ok(value) = std::env::var("CLIENT_AGENT_SERVER_HOST") {
            self.server.host = value;
        }

        if let Ok(value) = std::env::var("CLIENT_AGENT_SERVER_PORT") {
            self.server.port = value.parse().map_err(|_| {
                ConfigError::validate(path, "CLIENT_AGENT_SERVER_PORT 必须是 1-65535 的整数")
            })?;
        }

        if let Ok(value) = std::env::var("CLIENT_AGENT_SERVER_STATUS_PATH") {
            self.server.status_path = value;
        }

        if let Ok(value) = std::env::var("CLIENT_AGENT_SCRIPT_SECURITY_ENABLED") {
            self.script_security.enabled =
                parse_bool(path, "CLIENT_AGENT_SCRIPT_SECURITY_ENABLED", &value)?;
        }

        Ok(())
    }

    fn validate(&self, path: &Path) -> Result<(), ConfigError> {
        if self.client.id.trim().is_empty() {
            return Err(ConfigError::validate(path, "client.id 不能为空"));
        }

        if self.client.display_name.trim().is_empty() {
            return Err(ConfigError::validate(path, "client.display_name 不能为空"));
        }

        if self.client.group.trim().is_empty() {
            return Err(ConfigError::validate(path, "client.group 不能为空"));
        }

        if self.client.tags.iter().any(|tag| tag.trim().is_empty()) {
            return Err(ConfigError::validate(path, "client.tags 不能包含空标签"));
        }

        if self.lua.bootstrap_name.trim().is_empty() {
            return Err(ConfigError::validate(path, "lua.bootstrap_name 不能为空"));
        }

        if self.lua.bootstrap_path.as_os_str().is_empty() {
            return Err(ConfigError::validate(path, "lua.bootstrap_path 不能为空"));
        }

        // 指令上限是 Lua 宿主的最小安全边界。
        // 输入：TOML 中的 instruction_limit。
        // 输出：大于 0 的限制值。
        // 边界：0 会让无限循环脚本无法被可靠拦截，因此直接拒绝启动。
        if self.lua.instruction_limit == 0 {
            return Err(ConfigError::validate(
                path,
                "lua.instruction_limit 必须大于 0",
            ));
        }

        if self.dm.bridge_path.as_os_str().is_empty() {
            return Err(ConfigError::validate(path, "dm.bridge_path 不能为空"));
        }

        if self.script_security.enabled {
            if self.script_security.manifest_path.as_os_str().is_empty() {
                return Err(ConfigError::validate(
                    path,
                    "script_security.manifest_path 不能为空",
                ));
            }

            if !is_hex_with_len(&self.script_security.trusted_signer_public_key, 64) {
                return Err(ConfigError::validate(
                    path,
                    "script_security.trusted_signer_public_key 必须是 64 位十六进制 Ed25519 公钥",
                ));
            }

            if self
                .script_security
                .allowed_permissions
                .iter()
                .any(|permission| permission.trim().is_empty())
            {
                return Err(ConfigError::validate(
                    path,
                    "script_security.allowed_permissions 不能包含空权限",
                ));
            }
        }

        if self.server.host.trim().is_empty() {
            return Err(ConfigError::validate(path, "server.host 不能为空"));
        }

        if self.server.port == 0 {
            return Err(ConfigError::validate(path, "server.port 必须大于 0"));
        }

        if !self.server.status_path.starts_with('/') {
            return Err(ConfigError::validate(
                path,
                "server.status_path 必须以 / 开头",
            ));
        }

        if self.server.connect_timeout_ms == 0 {
            return Err(ConfigError::validate(
                path,
                "server.connect_timeout_ms 必须大于 0",
            ));
        }

        Ok(())
    }
}

pub fn ensure_config_exists(path: impl AsRef<Path>) -> Result<(), ConfigError> {
    let path = path.as_ref();
    if path.exists() {
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| ConfigError::write(path, error))?;
    }

    fs::write(path, DEFAULT_CONFIG_TEMPLATE).map_err(|error| ConfigError::write(path, error))
}

fn is_hex_with_len(value: &str, expected_len: usize) -> bool {
    value.len() == expected_len && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn default_client_display_name() -> String {
    "Local Dev Client".to_string()
}

fn default_client_group() -> String {
    "default".to_string()
}

fn parse_tags(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|tag| !tag.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn parse_bool(path: &Path, key: &str, value: &str) -> Result<bool, ConfigError> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Ok(true),
        "0" | "false" | "no" | "off" => Ok(false),
        _ => Err(ConfigError::validate(
            path,
            format!("{key} 必须是 true/false 或 1/0"),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_exposes_whitelisted_values() {
        let config = AgentConfig {
            client: ClientConfig {
                id: "local-dev-client".to_string(),
                display_name: "Local Dev Client".to_string(),
                group: "default".to_string(),
                tags: vec!["local".to_string()],
            },
            lua: LuaConfig {
                bootstrap_name: "bootstrap".to_string(),
                bootstrap_path: PathBuf::from("scripts/bootstrap.lua"),
                instruction_limit: 1000,
            },
            script_security: ScriptSecurityConfig {
                enabled: true,
                manifest_path: PathBuf::from("scripts/bootstrap.manifest.json"),
                trusted_signer_public_key:
                    "1111111111111111111111111111111111111111111111111111111111111111".to_string(),
                allowed_permissions: vec![
                    "host.log".to_string(),
                    "config.read".to_string(),
                    "dm.access".to_string(),
                ],
            },
            dm: DmConfig {
                bridge_path: PathBuf::from("../../target/dm-bridge/Win32/DmBridge.dll"),
            },
            server: ServerConfig {
                enabled: false,
                host: "127.0.0.1".to_string(),
                port: 18080,
                status_path: "/api/client/status".to_string(),
                connect_timeout_ms: 3000,
            },
        };

        assert_eq!(
            config.get_value("client.id"),
            Some("local-dev-client".to_string())
        );
        assert_eq!(
            config.get_value("client.group"),
            Some("default".to_string())
        );
        assert_eq!(config.get_value("client.tags"), Some("local".to_string()));
        assert_eq!(
            config.get_value("lua.bootstrap_path"),
            Some("scripts/bootstrap.lua".to_string())
        );
        assert_eq!(
            config.get_value("dm.bridge_path"),
            Some("../../target/dm-bridge/Win32/DmBridge.dll".to_string())
        );
        assert_eq!(
            config.get_value("server.status_path"),
            Some("/api/client/status".to_string())
        );
        assert_eq!(
            config.get_value("script_security.enabled"),
            Some("true".to_string())
        );
        assert_eq!(config.get_value("unknown.key"), None);
    }

    #[test]
    fn default_config_enables_dm_access() {
        let config: AgentConfig =
            toml::from_str(DEFAULT_CONFIG_TEMPLATE).expect("default config must parse");

        assert!(
            config
                .script_security
                .allowed_permissions
                .contains(&"dm.access".to_string())
        );
    }
}
