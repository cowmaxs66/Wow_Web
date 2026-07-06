mod error;
mod path;

pub use error::ConfigError;
pub use path::default_config_path;

use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AgentConfig {
    pub client: ClientConfig,
    pub lua: LuaConfig,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ClientConfig {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct LuaConfig {
    pub bootstrap_name: String,
    pub bootstrap_script: String,
}

impl AgentConfig {
    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let content = fs::read_to_string(path).map_err(|error| ConfigError::read(path, error))?;

        // 使用 TOML 只承载开发期必要配置，避免一开始引入复杂配置层级。
        // 输入：client-agent.toml 文本内容。
        // 输出：强类型 AgentConfig。
        // 边界：配置结构不匹配时立即失败，不生成隐式默认值。
        toml::from_str(&content).map_err(|error| ConfigError::parse(path, error))
    }

    pub fn get_value(&self, key: &str) -> Option<String> {
        match key {
            "client.id" => Some(self.client.id.clone()),
            "lua.bootstrap_name" => Some(self.lua.bootstrap_name.clone()),
            _ => None,
        }
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
                bootstrap_script: "return 'ok'".to_string(),
            },
        };

        assert_eq!(
            config.get_value("client.id"),
            Some("local-dev-client".to_string())
        );
        assert_eq!(config.get_value("unknown.key"), None);
    }
}
