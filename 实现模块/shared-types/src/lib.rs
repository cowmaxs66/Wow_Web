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
}

impl ClientStatus {
    pub fn new(client_id: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            online: true,
            current_script: None,
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
    }
}
