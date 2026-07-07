use crate::config::{default_config_path, ensure_config_exists};
use crate::ps_script;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub fn open_settings_window() -> io::Result<()> {
    let config_path = default_config_path();
    ensure_settings_config_exists(&config_path)?;
    let script_path = write_settings_script(&config_path)?;

    let mut command = Command::new(shell_executable());
    command.args([
        "-STA",
        "-NoProfile",
        "-ExecutionPolicy",
        "Bypass",
        "-File",
        &script_path.display().to_string(),
    ]);

    #[cfg(windows)]
    {
        command.creation_flags(CREATE_NO_WINDOW);
    }

    command.spawn().map(|_| ())
}

fn ensure_settings_config_exists(config_path: &Path) -> io::Result<()> {
    ensure_config_exists(config_path).map_err(io::Error::other)
}

fn write_settings_script(config_path: &Path) -> io::Result<PathBuf> {
    let script_dir = std::env::temp_dir().join("wow-client-agent");
    fs::create_dir_all(&script_dir)?;
    let script_path = script_dir.join("settings-window.ps1");
    let config = escape_ps_single(&config_path.display().to_string());
    let script = settings_script_template(&config);
    ps_script::write_utf8_bom(&script_path, &script)?;
    Ok(script_path)
}

fn settings_script_template(escaped_config_path: &str) -> String {
    SETTINGS_SCRIPT_TEMPLATE.replace("__CONFIG_PATH__", escaped_config_path)
}

const SETTINGS_SCRIPT_TEMPLATE: &str = include_str!("settings_window_script.ps1");

fn shell_executable() -> &'static str {
    "powershell"
}

fn escape_ps_single(value: &str) -> String {
    value.replace('\'', "''")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn powershell_single_quotes_are_escaped() {
        assert_eq!(escape_ps_single("C:\\a'b"), "C:\\a''b");
    }

    #[test]
    fn settings_script_uses_structured_form() {
        let script = settings_script_template("C:\\config\\client-agent.toml");

        assert!(script.contains("Server 上报"));
        assert!(script.contains("Lua 脚本"));
        assert!(script.contains("脚本安全门"));
        assert!(script.contains("保存设置"));
        assert!(script.contains("[System.IO.Directory]::CreateDirectory"));
        assert!(!script.contains("$editor.Multiline = $true"));
    }
}
