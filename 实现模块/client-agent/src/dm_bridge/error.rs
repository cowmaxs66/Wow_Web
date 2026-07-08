use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum DmBridgeError {
    LoadFailed {
        path: PathBuf,
        message: String,
    },
    UnsupportedArchitecture {
        path: PathBuf,
        dll_arch: String,
        process_arch: String,
    },
    SymbolMissing {
        name: String,
        message: String,
    },
    InvalidAbi {
        expected: i32,
        actual: i32,
    },
    InvalidInput {
        context: &'static str,
        message: String,
    },
    BridgeFailed {
        context: &'static str,
        status: i32,
        message: String,
    },
}

impl DmBridgeError {
    pub fn load_failed(path: &Path, message: impl Into<String>) -> Self {
        Self::LoadFailed {
            path: path.to_path_buf(),
            message: message.into(),
        }
    }

    pub fn symbol_missing(name: String, message: impl Into<String>) -> Self {
        Self::SymbolMissing {
            name,
            message: message.into(),
        }
    }
}

impl Display for DmBridgeError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoadFailed { path, message } => {
                write!(
                    formatter,
                    "加载 DmBridge 失败：{}，原因：{message}",
                    path.display()
                )
            }
            Self::UnsupportedArchitecture {
                path,
                dll_arch,
                process_arch,
            } => {
                write!(
                    formatter,
                    "DmBridge DLL 架构不匹配：{} 是 {dll_arch}，当前进程是 {process_arch}",
                    path.display()
                )
            }
            Self::SymbolMissing { name, message } => {
                write!(
                    formatter,
                    "加载 DmBridge 导出函数失败：{name}，原因：{message}"
                )
            }
            Self::InvalidAbi { expected, actual } => {
                write!(
                    formatter,
                    "DmBridge ABI 版本不匹配：期望 {expected}，实际 {actual}"
                )
            }
            Self::InvalidInput { context, message } => {
                write!(formatter, "DmBridge 参数无效：{context}，原因：{message}")
            }
            Self::BridgeFailed {
                context,
                status,
                message,
            } => {
                write!(
                    formatter,
                    "DmBridge 调用失败：{context}，状态码 {status}，消息：{message}"
                )
            }
        }
    }
}

impl std::error::Error for DmBridgeError {}
