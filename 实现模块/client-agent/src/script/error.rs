use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ScriptError {
    ReadFailed { path: PathBuf, message: String },
}

impl ScriptError {
    pub fn read(path: &Path, error: std::io::Error) -> Self {
        Self::ReadFailed {
            path: path.to_path_buf(),
            message: error.to_string(),
        }
    }
}

impl Display for ScriptError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadFailed { path, message } => {
                write!(
                    formatter,
                    "读取 Lua 脚本失败：{}，原因：{message}",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for ScriptError {}
