use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ConfigError {
    ReadFailed { path: PathBuf, message: String },
    ParseFailed { path: PathBuf, message: String },
}

impl ConfigError {
    pub fn read(path: &Path, error: std::io::Error) -> Self {
        Self::ReadFailed {
            path: path.to_path_buf(),
            message: error.to_string(),
        }
    }

    pub fn parse(path: &Path, error: toml::de::Error) -> Self {
        Self::ParseFailed {
            path: path.to_path_buf(),
            message: error.to_string(),
        }
    }
}

impl Display for ConfigError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadFailed { path, message } => {
                write!(
                    formatter,
                    "读取配置失败：{}，原因：{message}",
                    path.display()
                )
            }
            Self::ParseFailed { path, message } => {
                write!(
                    formatter,
                    "解析配置失败：{}，原因：{message}",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for ConfigError {}
