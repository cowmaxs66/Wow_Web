mod error;
mod path;

pub use error::ConfigError;
pub use path::default_config_path;

use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AgentConfig {
    pub client: ClientConfig,
    pub lua: LuaConfig,
    pub dm: DmConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ClientConfig {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct LuaConfig {
    pub bootstrap_name: String,
    pub bootstrap_path: PathBuf,
    pub instruction_limit: u32,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct DmConfig {
    pub bridge_path: PathBuf,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
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
        let content = fs::read_to_string(path).map_err(|error| ConfigError::read(path, error))?;

        // 使用 TOML 只承载开发期必要配置，避免一开始引入复杂配置层级。
        // 输入：client-agent.toml 文本内容。
        // 输出：强类型 AgentConfig。
        // 边界：配置结构不匹配时立即失败，不生成隐式默认值。
        let mut config: Self =
            toml::from_str(&content).map_err(|error| ConfigError::parse(path, error))?;
        config.apply_env_overrides(path)?;
        config.validate(path)?;
        Ok(config)
    }

    pub fn get_value(&self, key: &str) -> Option<String> {
        match key {
            "client.id" => Some(self.client.id.clone()),
            "lua.bootstrap_name" => Some(self.lua.bootstrap_name.clone()),
            "lua.bootstrap_path" => Some(self.lua.bootstrap_path.display().to_string()),
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

        Ok(())
    }

    fn validate(&self, path: &Path) -> Result<(), ConfigError> {
        if self.client.id.trim().is_empty() {
            return Err(ConfigError::validate(path, "client.id 不能为空"));
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
            },
            lua: LuaConfig {
                bootstrap_name: "bootstrap".to_string(),
                bootstrap_path: PathBuf::from("scripts/bootstrap.lua"),
                instruction_limit: 1000,
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
        assert_eq!(config.get_value("unknown.key"), None);
    }
}
