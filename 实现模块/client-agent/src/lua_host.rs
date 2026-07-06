use crate::config::AgentConfig;
use crate::lua_dm;
use crate::script::ScriptSource;
use mlua::{Error as LuaError, HookTriggers, Lua, VmState};
use std::path::PathBuf;
use std::sync::atomic::{AtomicI64, Ordering};

/// Lua 宿主只负责注册白名单 API 并执行脚本，不负责读取配置或网络通信。
pub struct LuaHost {
    config: AgentConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptRunReport {
    pub script_name: String,
    pub script_path: PathBuf,
    pub result: String,
    pub instruction_limit: u32,
}

impl LuaHost {
    pub fn new(config: AgentConfig) -> Self {
        Self { config }
    }

    pub fn run_script(&self, script: &ScriptSource) -> mlua::Result<ScriptRunReport> {
        let lua = Lua::new();
        self.install_instruction_limit(&lua)?;
        self.install_host_api(&lua)?;

        // Lua 文件由 script 模块读取，宿主只执行已确认的脚本文本。
        // 输入：ScriptSource 中的脚本名称、路径和内容。
        // 输出：脚本返回的字符串结果和执行元信息。
        // 边界：脚本返回非字符串、运行时错误或超过指令上限都会失败。
        let result = lua.load(&script.content).set_name(&script.name).eval()?;

        Ok(ScriptRunReport {
            script_name: script.name.clone(),
            script_path: script.path.clone(),
            result,
            instruction_limit: self.config.lua.instruction_limit,
        })
    }

    fn install_host_api(&self, lua: &Lua) -> mlua::Result<()> {
        let globals = lua.globals();

        // 注册日志函数，让 Lua 可以输出可追踪日志，但不能直接操作文件或系统命令。
        let client_id_for_log = self.config.client.id.clone();
        let log = lua.create_function(move |_, message: String| {
            tracing::info!(target: "lua", client_id = %client_id_for_log, message = %message);
            Ok(())
        })?;
        globals.set("log", log)?;

        // 注册只读配置函数，只允许读取明确白名单内的键，避免脚本窥探完整配置。
        let config_for_lua = self.config.clone();
        let get_config =
            lua.create_function(move |_, key: String| Ok(config_for_lua.get_value(&key)))?;
        globals.set("get_config", get_config)?;
        globals.set("dm", lua_dm::create_table(lua, &self.config)?)?;

        Ok(())
    }

    fn install_instruction_limit(&self, lua: &Lua) -> mlua::Result<()> {
        let instruction_limit = self.config.lua.instruction_limit;
        let hook_step = instruction_limit.clamp(1, 1000);
        let remaining = AtomicI64::new(i64::from(instruction_limit));

        // 用 VM hook 做开发期最小防护，避免 Lua 脚本无限循环卡死 Agent。
        // 输入：配置中的 instruction_limit。
        // 输出：正常继续执行，或在超过限制时返回 Lua runtime error。
        // 边界：hook 不是精确计数器，会按 hook_step 粒度拦截，适合作为 P1 最小闭环保护。
        lua.set_hook(
            HookTriggers::new().every_nth_instruction(hook_step),
            move |_, _| {
                if remaining.fetch_sub(i64::from(hook_step), Ordering::Relaxed)
                    <= i64::from(hook_step)
                {
                    Err(LuaError::runtime("Lua 脚本超过指令上限"))
                } else {
                    Ok(VmState::Continue)
                }
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AgentConfig, ClientConfig, DmConfig, LuaConfig, ServerConfig};
    use std::path::PathBuf;

    fn test_config(instruction_limit: u32) -> AgentConfig {
        AgentConfig {
            client: ClientConfig {
                id: "lua-test-client".to_string(),
            },
            lua: LuaConfig {
                bootstrap_name: "test-bootstrap".to_string(),
                bootstrap_path: PathBuf::from("scripts/bootstrap.lua"),
                instruction_limit,
            },
            dm: DmConfig {
                bridge_path: PathBuf::from("missing/DmBridge.dll"),
            },
            server: ServerConfig {
                enabled: false,
                host: "127.0.0.1".to_string(),
                port: 18080,
                status_path: "/api/client/status".to_string(),
                connect_timeout_ms: 3000,
            },
        }
    }

    fn test_script(content: &str) -> ScriptSource {
        ScriptSource {
            name: "test-bootstrap".to_string(),
            path: PathBuf::from("scripts/bootstrap.lua"),
            content: content.to_string(),
        }
    }

    #[test]
    fn lua_bootstrap_can_read_whitelisted_config() {
        let report = LuaHost::new(test_config(10_000))
            .run_script(&test_script(
                r#"
                    log("bootstrap started")
                    return get_config("client.id")
                "#,
            ))
            .expect("lua must run");

        assert_eq!(report.result, "lua-test-client");
        assert_eq!(report.script_name, "test-bootstrap");
    }

    #[test]
    fn lua_bootstrap_reports_instruction_limit_error() {
        let error = LuaHost::new(test_config(100))
            .run_script(&test_script("while true do end"))
            .expect_err("infinite loop must hit instruction limit");

        assert!(error.to_string().contains("Lua 脚本超过指令上限"));
    }

    #[test]
    fn lua_dm_api_reports_bridge_load_error() {
        let error = LuaHost::new(test_config(10_000))
            .run_script(&test_script("return dm.abi_version()"))
            .expect_err("missing bridge must fail clearly");

        assert!(error.to_string().contains("加载 DmBridge 失败"));
    }

    #[test]
    fn lua_dm_api_reads_abi_version_from_env_when_available() {
        let Some(path) = std::env::var_os("DM_BRIDGE_DLL") else {
            return;
        };

        let mut config = test_config(10_000);
        config.dm.bridge_path = PathBuf::from(path);
        let report = LuaHost::new(config)
            .run_script(&test_script("return tostring(dm.abi_version())"))
            .expect("Lua dm API must call DmBridge");

        assert_eq!(report.result, "1");
    }

    #[test]
    fn lua_dm_api_com_ver_and_color_smoke_when_enabled() {
        if std::env::var("DM_BRIDGE_COM_SMOKE").ok().as_deref() != Some("1") {
            return;
        }

        let path = std::env::var_os("DM_BRIDGE_DLL").expect("DM_BRIDGE_DLL must be set");
        let mut config = test_config(10_000);
        config.dm.bridge_path = PathBuf::from(path);
        let report = LuaHost::new(config)
            .run_script(&test_script(
                r#"
                    dm.init("")
                    local version = dm.ver()
                    local color = dm.get_color(0, 0)
                    local move_ret = dm.move_to(0, 0)
                    dm.shutdown()
                    return version .. "|" .. color .. "|" .. tostring(move_ret)
                "#,
            ))
            .expect("Lua dm COM smoke must work");

        assert!(report.result.contains("|"));
    }
}
