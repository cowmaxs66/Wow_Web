use crate::config::AgentConfig;
use mlua::Lua;

/// Lua 宿主只负责注册白名单 API 并执行脚本，不负责读取配置或网络通信。
pub struct LuaHost {
    config: AgentConfig,
}

impl LuaHost {
    pub fn new(config: AgentConfig) -> Self {
        Self { config }
    }

    pub fn run_bootstrap(&self) -> mlua::Result<String> {
        let lua = Lua::new();
        let globals = lua.globals();

        // 注册日志函数，让 Lua 可以输出可追踪日志，但不能直接操作文件或系统命令。
        let client_id_for_log = self.config.client.id.clone();
        let log = lua.create_function(move |_, message: String| {
            eprintln!("[lua:{client_id_for_log}] {message}");
            Ok(())
        })?;
        globals.set("log", log)?;

        // 注册只读配置函数，只允许读取明确白名单内的键，避免脚本窥探完整配置。
        let config_for_lua = self.config.clone();
        let get_config =
            lua.create_function(move |_, key: String| Ok(config_for_lua.get_value(&key)))?;
        globals.set("get_config", get_config)?;

        // 当前只注册最小白名单 API，避免 Lua 脚本直接接触系统能力。
        // 输入：配置中的 bootstrap_script。
        // 输出：Lua 返回的字符串结果，用于确认宿主执行成功。
        // 边界：脚本返回非字符串会报错，后续脚本管理阶段再扩展结果协议。
        lua.load(&self.config.lua.bootstrap_script).eval()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AgentConfig, ClientConfig, LuaConfig};

    #[test]
    fn lua_bootstrap_can_read_whitelisted_config() {
        let config = AgentConfig {
            client: ClientConfig {
                id: "lua-test-client".to_string(),
            },
            lua: LuaConfig {
                bootstrap_name: "test-bootstrap".to_string(),
                bootstrap_script: r#"
                    log("bootstrap started")
                    return get_config("client.id")
                "#
                .to_string(),
            },
        };

        let result = LuaHost::new(config).run_bootstrap().expect("lua must run");

        assert_eq!(result, "lua-test-client");
    }
}
