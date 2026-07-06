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

impl AgentConfig {
    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
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

    pub fn get_value(&self, key: &str) -> Option<String> {
        match key {
            "client.id" => Some(self.client.id.clone()),
            "lua.bootstrap_name" => Some(self.lua.bootstrap_name.clone()),
            "lua.bootstrap_path" => Some(self.lua.bootstrap_path.display().to_string()),
            _ => None,
        }
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

        Ok(())
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
        };

        assert_eq!(
            config.get_value("client.id"),
            Some("local-dev-client".to_string())
        );
        assert_eq!(
            config.get_value("lua.bootstrap_path"),
            Some("scripts/bootstrap.lua".to_string())
        );
        assert_eq!(config.get_value("unknown.key"), None);
    }
}
