use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ServerReportError {
    ResolveFailed(String),
    ConnectFailed(String),
    IoFailed(String),
    SerializeFailed(String),
    InvalidResponse(String),
    ServerRejected { status: u16, body: String },
}

impl Display for ServerReportError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ResolveFailed(message) => write!(formatter, "解析 Server 地址失败：{message}"),
            Self::ConnectFailed(message) => {
                write!(formatter, "连接 Management Server 失败：{message}")
            }
            Self::IoFailed(message) => write!(formatter, "上报状态 IO 失败：{message}"),
            Self::SerializeFailed(message) => write!(formatter, "序列化状态消息失败：{message}"),
            Self::InvalidResponse(message) => write!(formatter, "Server 响应格式错误：{message}"),
            Self::ServerRejected { status, body } => {
                write!(
                    formatter,
                    "Server 拒绝状态上报：HTTP {status}，响应：{body}"
                )
            }
        }
    }
}

impl std::error::Error for ServerReportError {}
