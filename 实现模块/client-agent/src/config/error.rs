use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ConfigError {
    Read { path: PathBuf, message: String },
    Write { path: PathBuf, message: String },
    Parse { path: PathBuf, message: String },
    Serialize { path: PathBuf, message: String },
    Validate { path: PathBuf, message: String },
}

impl ConfigError {
    pub fn read(path: &Path, error: std::io::Error) -> Self {
        Self::Read {
            path: path.to_path_buf(),
            message: error.to_string(),
        }
    }

    pub fn write(path: &Path, error: std::io::Error) -> Self {
        Self::Write {
            path: path.to_path_buf(),
            message: error.to_string(),
        }
    }

    pub fn parse(path: &Path, error: toml::de::Error) -> Self {
        Self::Parse {
            path: path.to_path_buf(),
            message: error.to_string(),
        }
    }

    pub fn serialize(path: &Path, error: toml::ser::Error) -> Self {
        Self::Serialize {
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
}

impl Display for ConfigError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read { path, message } => {
                write!(
                    formatter,
                    "读取配置失败：{}，原因：{message}",
                    path.display()
                )
            }
            Self::Write { path, message } => {
                write!(
                    formatter,
                    "写入配置失败：{}，原因：{message}",
                    path.display()
                )
            }
            Self::Parse { path, message } => {
                write!(
                    formatter,
                    "解析配置失败：{}，原因：{message}",
                    path.display()
                )
            }
            Self::Serialize { path, message } => {
                write!(
                    formatter,
                    "序列化配置失败：{}，原因：{message}",
                    path.display()
                )
            }
            Self::Validate { path, message } => {
                write!(
                    formatter,
                    "配置校验失败：{}，原因：{message}",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for ConfigError {}
