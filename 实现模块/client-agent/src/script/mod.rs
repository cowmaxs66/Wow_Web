mod error;

pub use error::ScriptError;

use crate::config::AgentConfig;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptSource {
    pub name: String,
    pub path: PathBuf,
    pub content: String,
}

impl ScriptSource {
    pub fn load_bootstrap(config: &AgentConfig) -> Result<Self, ScriptError> {
        let path = resolve_script_path(&config.lua.bootstrap_path);
        let content = fs::read_to_string(&path).map_err(|error| ScriptError::read(&path, error))?;

        // 脚本加载只负责把受控路径转换成脚本文本，不注册能力、不执行代码。
        // 输入：配置中的 bootstrap_name 与 bootstrap_path。
        // 输出：带名称、绝对路径和文本内容的 ScriptSource。
        // 边界：相对路径固定按 client-agent 模块根目录解析，避免从 workspace 根目录运行时路径漂移。
        Ok(Self {
            name: config.lua.bootstrap_name.clone(),
            path,
            content,
        })
    }
}

fn resolve_script_path(path: &PathBuf) -> PathBuf {
    if path.is_absolute() {
        return path.clone();
    }

    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_script_path_is_resolved_from_module_root() {
        let path = resolve_script_path(&PathBuf::from("scripts/bootstrap.lua"));

        assert!(path.ends_with("scripts/bootstrap.lua"));
        assert!(path.is_absolute());
    }
}
