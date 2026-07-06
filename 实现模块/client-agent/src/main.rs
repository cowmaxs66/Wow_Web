mod config;
mod lua_host;

use config::{AgentConfig, default_config_path};
use lua_host::LuaHost;
use shared_types::{ClientStatus, WsEnvelope};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config = AgentConfig::load_from_path(default_config_path())?;
    let lua_result = LuaHost::new(config.clone()).run_bootstrap()?;

    eprintln!("lua_bootstrap_result={lua_result}");

    let mut status = ClientStatus::new(config.client.id.clone());
    status.current_script = Some(config.lua.bootstrap_name.clone());
    let envelope = WsEnvelope::status(config.client.id, status);

    // 当前阶段输出标准 JSON，验证配置读取、Lua 宿主和协议消息可以串成闭环。
    // 输入：本地 TOML 配置和内置 Lua bootstrap 脚本。
    // 输出：包含 client_id 与当前脚本名的状态消息。
    // 边界：P1 后续接入脚本文件加载后，bootstrap_script 不再放在配置内。
    let json = serde_json::to_string_pretty(&envelope).expect("status envelope must serialize");
    println!("{json}");

    Ok(())
}
