use crate::config::AgentConfig;
use crate::lua_host::LuaHost;
use crate::script::ScriptSource;
use crate::server_reporter::StatusReporter;
use crate::status::AgentStatusSnapshot;
use shared_types::{ClientStatus, StatusAck, WsEnvelope};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct AgentRunResult {
    pub envelope: WsEnvelope<ClientStatus>,
    pub ack: Option<StatusAck>,
}

pub fn run_once(config: &AgentConfig) -> Result<AgentRunResult, Box<dyn Error>> {
    run_once_with_report(config, true)
}

pub fn run_once_local(config: &AgentConfig) -> Result<AgentRunResult, Box<dyn Error>> {
    run_once_with_report(config, false)
}

fn run_once_with_report(
    config: &AgentConfig,
    report_to_server: bool,
) -> Result<AgentRunResult, Box<dyn Error>> {
    let script = ScriptSource::load_bootstrap(config)?;
    let report = LuaHost::new(config.clone()).run_script(&script)?;

    tracing::info!(
        script = %report.script_name,
        path = %report.script_path.display(),
        result = %report.result,
        "Lua bootstrap 执行完成"
    );

    let status = AgentStatusSnapshot::from_script_report(config, &report).into_client_status();
    let envelope = WsEnvelope::status(config.client.id.clone(), status);
    let ack = if report_to_server && config.server.enabled {
        let ack = StatusReporter::new(config.server.clone()).report_status(&envelope)?;
        tracing::info!(
            client_id = %ack.client_id,
            message_id = %ack.message_id,
            accepted = ack.accepted,
            "Client 状态已上报 Management Server"
        );
        Some(ack)
    } else {
        None
    };

    Ok(AgentRunResult { envelope, ack })
}
