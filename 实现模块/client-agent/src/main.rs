mod config;
mod dm_bridge;
mod logging;
mod lua_dm;
mod lua_host;
mod script;
mod status;

use config::{AgentConfig, default_config_path};
use lua_host::LuaHost;
use script::ScriptSource;
use shared_types::WsEnvelope;
use status::AgentStatusSnapshot;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    logging::init();

    let config = AgentConfig::load_from_path(default_config_path())?;
    let script = ScriptSource::load_bootstrap(&config)?;
    let report = LuaHost::new(config.clone()).run_script(&script)?;

    tracing::info!(
        script = %report.script_name,
        path = %report.script_path.display(),
        result = %report.result,
        "Lua bootstrap 执行完成"
    );

    let status = AgentStatusSnapshot::from_script_report(&config, &report).into_client_status();
    let envelope = WsEnvelope::status(config.client.id.clone(), status);

    // 当前阶段输出标准 JSON，验证配置读取、Lua 宿主和协议消息可以串成闭环。
    // 输入：本地 TOML 配置和 bootstrap Lua 文件。
    // 输出：包含 client_id 与当前脚本名的状态消息。
    // 边界：真实 WebSocket 上报在 P2/P3 接入，这里只输出可验证 JSON。
    let json = serde_json::to_string_pretty(&envelope).expect("status envelope must serialize");
    println!("{json}");

    Ok(())
}
