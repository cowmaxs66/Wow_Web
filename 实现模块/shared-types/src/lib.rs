use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WsEnvelope<T> {
    pub schema_version: u16,
    pub message_id: String,
    pub message_type: MessageType,
    pub client_id: String,
    pub timestamp_ms: u128,
    pub data: T,
}

impl<T> WsEnvelope<T> {
    pub fn status(client_id: impl Into<String>, data: T) -> Self {
        let client_id = client_id.into();
        let timestamp_ms = current_timestamp_ms();

        // 使用本地时间戳生成首版消息 ID。
        // 输入：client_id 与当前毫秒时间。
        // 输出：可追踪但不保证全局强唯一的开发期 message_id。
        // 边界：后续接入 Server 后应替换为 UUID 或雪花 ID。
        let message_id = format!("{client_id}-{timestamp_ms}");

        Self {
            schema_version: 1,
            message_id,
            message_type: MessageType::Status,
            client_id,
            timestamp_ms,
            data,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClientStatus {
    pub client_id: String,
    pub online: bool,
    pub current_script: Option<String>,
    pub runtime: ClientRuntimeInfo,
    pub script: ClientScriptInfo,
    pub server: ClientServerInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClientRuntimeInfo {
    pub release_version: String,
    pub os: String,
    pub arch: String,
    pub process_id: u32,
}

impl ClientRuntimeInfo {
    pub fn unknown() -> Self {
        Self {
            release_version: "unknown".to_string(),
            os: "unknown".to_string(),
            arch: "unknown".to_string(),
            process_id: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClientScriptInfo {
    pub bootstrap_name: String,
    pub instruction_limit: u32,
    pub security_enabled: bool,
    pub allowed_permissions: Vec<String>,
}

impl ClientScriptInfo {
    pub fn unknown() -> Self {
        Self {
            bootstrap_name: "unknown".to_string(),
            instruction_limit: 0,
            security_enabled: false,
            allowed_permissions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClientServerInfo {
    pub report_enabled: bool,
    pub report_target: String,
}

impl ClientServerInfo {
    pub fn disabled() -> Self {
        Self {
            report_enabled: false,
            report_target: "disabled".to_string(),
        }
    }
}

impl ClientStatus {
    pub fn new(client_id: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            online: true,
            current_script: None,
            runtime: ClientRuntimeInfo::unknown(),
            script: ClientScriptInfo::unknown(),
            server: ClientServerInfo::disabled(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HealthResponse {
    pub status: String,
}

impl HealthResponse {
    pub fn ok() -> Self {
        Self {
            status: "ok".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StatusAck {
    pub accepted: bool,
    pub client_id: String,
    pub message_id: String,
}

impl StatusAck {
    pub fn accepted(client_id: impl Into<String>, message_id: impl Into<String>) -> Self {
        Self {
            accepted: true,
            client_id: client_id.into(),
            message_id: message_id.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClientStatusHistory {
    pub client_id: String,
    pub limit: usize,
    pub total: usize,
    pub items: Vec<WsEnvelope<ClientStatus>>,
}

impl ClientStatusHistory {
    pub fn new(
        client_id: impl Into<String>,
        limit: usize,
        items: Vec<WsEnvelope<ClientStatus>>,
    ) -> Self {
        Self {
            client_id: client_id.into(),
            limit,
            total: items.len(),
            items,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClientMessageRequest {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClientMessage {
    pub id: String,
    pub client_id: String,
    pub timestamp_ms: u128,
    pub title: String,
    pub body: String,
}

impl ClientMessage {
    pub fn new(client_id: impl Into<String>, request: ClientMessageRequest) -> Self {
        let client_id = client_id.into();
        let timestamp_ms = current_timestamp_ms();

        // P11 消息 ID 使用 client_id 和毫秒时间组合，保证本机试运行可追踪。
        // 输入：目标 Client ID 与消息正文。
        // 输出：可被 Client 轮询、日志记录和通知展示的消息。
        // 边界：生产环境需要改为 Server 侧强唯一 ID 和持久化队列。
        Self {
            id: format!("{client_id}-message-{timestamp_ms}"),
            client_id,
            timestamp_ms,
            title: request.title,
            body: request.body,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClientMessageList {
    pub client_id: String,
    pub total: usize,
    pub items: Vec<ClientMessage>,
}

impl ClientMessageList {
    pub fn new(client_id: impl Into<String>, items: Vec<ClientMessage>) -> Self {
        Self {
            client_id: client_id.into(),
            total: items.len(),
            items,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClientCommandRequest {
    pub command_type: String,
    #[serde(default)]
    pub payload: serde_json::Value,
}

pub const REMOTE_COMMAND_SCRIPT_RUN_BOOTSTRAP: &str = "script.run_bootstrap";
pub const REMOTE_COMMAND_STARTUP_STATUS: &str = "startup.status";
pub const REMOTE_COMMAND_STARTUP_ENABLE: &str = "startup.enable";
pub const REMOTE_COMMAND_STARTUP_DISABLE: &str = "startup.disable";
pub const REMOTE_COMMAND_SERVICE_STATUS: &str = "service.status";
pub const REMOTE_COMMAND_SERVICE_INSTALL: &str = "service.install";
pub const REMOTE_COMMAND_SERVICE_START: &str = "service.start";
pub const REMOTE_COMMAND_SERVICE_STOP: &str = "service.stop";
pub const REMOTE_COMMAND_UPDATE_CHECK: &str = "update.check";
pub const REMOTE_COMMAND_UPDATE_DOWNLOAD: &str = "update.download";
pub const REMOTE_COMMAND_UPDATE_APPLY: &str = "update.apply";
pub const REMOTE_COMMAND_SETTINGS_OPEN: &str = "settings.open";
pub const REMOTE_COMMAND_LOG_OPEN: &str = "log.open";
pub const REMOTE_COMMAND_TRAY_OPEN: &str = "tray.open";

pub const REMOTE_COMMAND_TYPES: &[&str] = &[
    REMOTE_COMMAND_SCRIPT_RUN_BOOTSTRAP,
    REMOTE_COMMAND_STARTUP_STATUS,
    REMOTE_COMMAND_STARTUP_ENABLE,
    REMOTE_COMMAND_STARTUP_DISABLE,
    REMOTE_COMMAND_SERVICE_STATUS,
    REMOTE_COMMAND_SERVICE_INSTALL,
    REMOTE_COMMAND_SERVICE_START,
    REMOTE_COMMAND_SERVICE_STOP,
    REMOTE_COMMAND_UPDATE_CHECK,
    REMOTE_COMMAND_UPDATE_DOWNLOAD,
    REMOTE_COMMAND_UPDATE_APPLY,
    REMOTE_COMMAND_SETTINGS_OPEN,
    REMOTE_COMMAND_LOG_OPEN,
    REMOTE_COMMAND_TRAY_OPEN,
];

pub fn is_supported_remote_command(command_type: &str) -> bool {
    // P25 将 Server 白名单和 Client 执行分支收敛到同一份共享清单。
    // 输入：协议中的 command_type 字符串。
    // 输出：是否属于当前版本允许的远程命令。
    // 边界：这里仍只定义“允许的命令类型”，不代表该命令一定能在本机执行成功。
    REMOTE_COMMAND_TYPES.contains(&command_type)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClientCommand {
    pub id: String,
    pub client_id: String,
    pub timestamp_ms: u128,
    pub command_type: String,
    pub payload: serde_json::Value,
}

impl ClientCommand {
    pub fn new(client_id: impl Into<String>, request: ClientCommandRequest) -> Self {
        let client_id = client_id.into();
        let timestamp_ms = current_timestamp_ms();

        // P13 远程命令与 P11 文本消息分离，避免把可执行动作塞进普通消息体。
        // 输入：目标 Client ID、命令类型和可选 JSON 参数。
        // 输出：可被 Client monitor 轮询并按白名单执行的命令。
        // 边界：当前仍是本机试运行队列；生产环境必须补鉴权、审计和送达确认。
        Self {
            id: format!("{client_id}-command-{timestamp_ms}"),
            client_id,
            timestamp_ms,
            command_type: request.command_type,
            payload: request.payload,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClientCommandList {
    pub client_id: String,
    pub total: usize,
    pub items: Vec<ClientCommand>,
}

impl ClientCommandList {
    pub fn new(client_id: impl Into<String>, items: Vec<ClientCommand>) -> Self {
        Self {
            client_id: client_id.into(),
            total: items.len(),
            items,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClientCommandReceiptRequest {
    pub command_id: String,
    pub command_type: String,
    pub success: bool,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClientCommandReceipt {
    pub id: String,
    pub client_id: String,
    pub timestamp_ms: u128,
    pub command_id: String,
    pub command_type: String,
    pub success: bool,
    pub summary: String,
}

impl ClientCommandReceipt {
    pub fn new(client_id: impl Into<String>, request: ClientCommandReceiptRequest) -> Self {
        let client_id = client_id.into();
        let timestamp_ms = current_timestamp_ms();

        // P24 命令回执独立于命令队列保存，避免 Client 拉取命令后 Server 无法知道执行结果。
        // 输入：Client 执行后的 command_id、command_type、success 和摘要。
        // 输出：可由 Web Admin 查询的审计记录。
        // 边界：当前仍是内存记录；生产环境需要持久化、操作者身份和更强唯一 ID。
        Self {
            id: format!("{client_id}-receipt-{timestamp_ms}"),
            client_id,
            timestamp_ms,
            command_id: request.command_id,
            command_type: request.command_type,
            success: request.success,
            summary: request.summary,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClientCommandReceiptList {
    pub client_id: String,
    pub total: usize,
    pub items: Vec<ClientCommandReceipt>,
}

impl ClientCommandReceiptList {
    pub fn new(client_id: impl Into<String>, items: Vec<ClientCommandReceipt>) -> Self {
        Self {
            client_id: client_id.into(),
            total: items.len(),
            items,
        }
    }
}

fn current_timestamp_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be later than UNIX_EPOCH")
        .as_millis()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_envelope_keeps_client_identity() {
        let status = ClientStatus::new("local-dev-client");
        let envelope = WsEnvelope::status("local-dev-client", status);

        assert_eq!(envelope.schema_version, 1);
        assert_eq!(envelope.client_id, "local-dev-client");
        assert_eq!(envelope.message_type, MessageType::Status);
        assert!(envelope.data.online);
        assert_eq!(envelope.data.runtime.release_version, "unknown");
        assert!(!envelope.data.server.report_enabled);
    }

    #[test]
    fn status_ack_keeps_message_identity() {
        let ack = StatusAck::accepted("local-dev-client", "message-1");

        assert!(ack.accepted);
        assert_eq!(ack.client_id, "local-dev-client");
        assert_eq!(ack.message_id, "message-1");
    }

    #[test]
    fn status_history_reports_total_from_items() {
        let envelope = WsEnvelope::status("client-a", ClientStatus::new("client-a"));
        let history = ClientStatusHistory::new("client-a", 50, vec![envelope]);

        assert_eq!(history.client_id, "client-a");
        assert_eq!(history.limit, 50);
        assert_eq!(history.total, 1);
    }

    #[test]
    fn client_message_keeps_target_identity() {
        let message = ClientMessage::new(
            "client-a",
            ClientMessageRequest {
                title: "测试消息".to_string(),
                body: "hello".to_string(),
            },
        );
        let list = ClientMessageList::new("client-a", vec![message.clone()]);

        assert_eq!(message.client_id, "client-a");
        assert!(message.id.starts_with("client-a-message-"));
        assert_eq!(message.title, "测试消息");
        assert_eq!(list.total, 1);
    }

    #[test]
    fn client_command_keeps_target_identity() {
        let command = ClientCommand::new(
            "client-a",
            ClientCommandRequest {
                command_type: REMOTE_COMMAND_STARTUP_STATUS.to_string(),
                payload: serde_json::json!({}),
            },
        );
        let list = ClientCommandList::new("client-a", vec![command.clone()]);

        assert_eq!(command.client_id, "client-a");
        assert!(command.id.starts_with("client-a-command-"));
        assert_eq!(command.command_type, REMOTE_COMMAND_STARTUP_STATUS);
        assert_eq!(list.total, 1);
        assert_eq!(list.items[0], command);
    }

    #[test]
    fn remote_command_catalog_accepts_only_known_types() {
        assert!(is_supported_remote_command(REMOTE_COMMAND_STARTUP_STATUS));
        assert!(is_supported_remote_command(REMOTE_COMMAND_UPDATE_APPLY));
        assert!(is_supported_remote_command(
            REMOTE_COMMAND_SCRIPT_RUN_BOOTSTRAP
        ));
        assert!(!is_supported_remote_command("shell.exec"));
    }

    #[test]
    fn client_command_receipt_keeps_command_identity() {
        let receipt = ClientCommandReceipt::new(
            "client-a",
            ClientCommandReceiptRequest {
                command_id: "cmd-1".to_string(),
                command_type: "startup.status".to_string(),
                success: true,
                summary: "ok".to_string(),
            },
        );
        let list = ClientCommandReceiptList::new("client-a", vec![receipt.clone()]);

        assert_eq!(receipt.client_id, "client-a");
        assert!(receipt.id.starts_with("client-a-receipt-"));
        assert_eq!(receipt.command_id, "cmd-1");
        assert_eq!(receipt.command_type, "startup.status");
        assert!(receipt.success);
        assert_eq!(list.total, 1);
    }
}
