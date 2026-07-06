use crate::config::AgentConfig;
use crate::lua_host::ScriptRunReport;
use shared_types::ClientStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentStatusSnapshot {
    client_id: String,
    current_script: Option<String>,
}

impl AgentStatusSnapshot {
    pub fn from_script_report(config: &AgentConfig, report: &ScriptRunReport) -> Self {
        // 这里保留 client-agent 内部状态快照，避免 main.rs 直接拼共享协议结构。
        // 输入：Agent 配置和 Lua 执行报告。
        // 输出：可转换为 shared-types::ClientStatus 的状态快照。
        // 边界：后续增加窗口绑定、大漠连接状态时，只扩展本模块映射。
        Self {
            client_id: config.client.id.clone(),
            current_script: Some(report.script_name.clone()),
        }
    }

    pub fn into_client_status(self) -> ClientStatus {
        let mut status = ClientStatus::new(self.client_id);
        status.current_script = self.current_script;
        status
    }
}
