use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ScriptError {
    Read { path: PathBuf, message: String },
    Parse { path: PathBuf, message: String },
    Validate { path: PathBuf, message: String },
    Security { path: PathBuf, message: String },
}

impl ScriptError {
    pub fn read(path: &Path, error: std::io::Error) -> Self {
        Self::Read {
            path: path.to_path_buf(),
            message: error.to_string(),
        }
    }

    pub fn parse(path: &Path, error: serde_json::Error) -> Self {
        Self::Parse {
            path: path.to_path_buf(),
            message: error.to_string(),
        }
    }

    pub fn validate(path: &Path, message: impl Into<String>) -> Self {
        Self::Validate {
            path: path.to_path_buf(),
            message: message.into(),
        }
    }

    pub fn security(path: &Path, message: impl Into<String>) -> Self {
        Self::Security {
            path: path.to_path_buf(),
            message: message.into(),
        }
    }
}

impl Display for ScriptError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read { path, message } => {
                write!(
                    formatter,
                    "读取 Lua 脚本失败：{}，原因：{message}",
                    path.display()
                )
            }
            Self::Parse { path, message } => {
                write!(
                    formatter,
                    "解析脚本 manifest 失败：{}，原因：{message}",
                    path.display()
                )
            }
            Self::Validate { path, message } => {
                write!(
                    formatter,
                    "脚本 manifest 校验失败：{}，原因：{message}",
                    path.display()
                )
            }
            Self::Security { path, message } => {
                write!(
                    formatter,
                    "脚本安全校验失败：{}，原因：{message}",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for ScriptError {}
