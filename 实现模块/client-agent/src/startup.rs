use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const RUN_KEY: &str = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run";
const VALUE_NAME: &str = "WoW Client Agent";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartupStatus {
    pub value_name: String,
    pub enabled: bool,
    pub expected_command: String,
    pub registered_command: Option<String>,
}

impl StartupStatus {
    pub fn summary(&self) -> String {
        let registered = self
            .registered_command
            .as_deref()
            .unwrap_or("未注册开机启动项");

        format!(
            "Client 开机启动状态：{}\n注册项：{}\n期望命令：{}\n当前命令：{}",
            if self.enabled {
                "已启用"
            } else {
                "未启用"
            },
            self.value_name,
            self.expected_command,
            registered
        )
    }
}

pub fn startup_status() -> io::Result<StartupStatus> {
    let expected_command = expected_tray_command()?;
    let registered_command = read_registered_command()?;
    let enabled = registered_command
        .as_deref()
        .is_some_and(|command| command == expected_command);

    Ok(StartupStatus {
        value_name: VALUE_NAME.to_string(),
        enabled,
        expected_command,
        registered_command,
    })
}

pub fn enable_startup() -> io::Result<StartupStatus> {
    let command = expected_tray_command()?;
    let output = run_reg([
        "add", RUN_KEY, "/v", VALUE_NAME, "/t", "REG_SZ", "/d", &command, "/f",
    ])?;
    ensure_reg_success(output, "写入开机启动项")?;
    startup_status()
}

pub fn disable_startup() -> io::Result<StartupStatus> {
    if read_registered_command()?.is_none() {
        return startup_status();
    }

    let output = run_reg(["delete", RUN_KEY, "/v", VALUE_NAME, "/f"])?;
    ensure_reg_success(output, "删除开机启动项")?;
    startup_status()
}

fn expected_tray_command() -> io::Result<String> {
    let exe = env::current_exe()?;
    let startup_exe = resolve_formal_startup_exe(&exe);
    build_tray_command(&startup_exe)
}

fn read_registered_command() -> io::Result<Option<String>> {
    let output = run_reg(["query", RUN_KEY, "/v", VALUE_NAME])?;
    if !output.status.success() {
        return Ok(None);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(parse_reg_query_output(&stdout, VALUE_NAME))
}

fn run_reg<const N: usize>(args: [&str; N]) -> io::Result<Output> {
    Command::new("reg").args(args).output()
}

fn ensure_reg_success(output: Output, action: &str) -> io::Result<()> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let detail = if !stderr.is_empty() { stderr } else { stdout };

    Err(io::Error::other(format!(
        "{action}失败：{}",
        if detail.is_empty() {
            "reg.exe 未返回错误详情"
        } else {
            &detail
        }
    )))
}

fn build_tray_command(exe_path: &Path) -> io::Result<String> {
    let exe = exe_path.display().to_string();
    if exe.contains('"') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "client-agent.exe 路径不能包含双引号",
        ));
    }

    // P14 起，开机启动使用正式托盘入口，而不是隐藏 monitor 命令。
    // P15 起，发布包内 core exe 位于 bin 下时，优先写入根目录 launcher。
    // 输入：当前运行的 client-agent.exe 路径。
    // 输出：HKCU Run 使用的命令字符串。
    // 边界：Server 地址、脚本路径和大漠路径仍由本机配置文件/env 管理，避免把私有配置写进注册表。
    Ok(format!("\"{exe}\""))
}

fn resolve_formal_startup_exe(current_exe: &Path) -> PathBuf {
    if !current_exe
        .file_name()
        .is_some_and(|name| name.eq_ignore_ascii_case("client-agent-core.exe"))
    {
        return current_exe.to_path_buf();
    }

    let Some(bin_dir) = current_exe.parent() else {
        return current_exe.to_path_buf();
    };
    if !bin_dir
        .file_name()
        .is_some_and(|name| name.eq_ignore_ascii_case("bin"))
    {
        return current_exe.to_path_buf();
    }

    let Some(package_root) = bin_dir.parent() else {
        return current_exe.to_path_buf();
    };
    let launcher = package_root.join("client-agent.exe");
    if launcher.exists() {
        launcher
    } else {
        current_exe.to_path_buf()
    }
}

fn parse_reg_query_output(output: &str, value_name: &str) -> Option<String> {
    for line in output.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with(value_name) {
            continue;
        }

        let rest = trimmed[value_name.len()..].trim_start();
        if let Some(command) = rest.strip_prefix("REG_SZ") {
            return Some(command.trim_start().to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn build_tray_command_quotes_exe_path() {
        let command = build_tray_command(&PathBuf::from(r"C:\Program Files\WoW\client-agent.exe"))
            .expect("path without quotes must format");

        assert_eq!(command, r#""C:\Program Files\WoW\client-agent.exe""#);
    }

    #[test]
    fn core_exe_startup_prefers_package_launcher() {
        let root =
            std::env::temp_dir().join(format!("wow-startup-launcher-{}", std::process::id()));
        let bin = root.join("bin");
        std::fs::create_dir_all(&bin).expect("bin dir must be created");
        let launcher = root.join("client-agent.exe");
        std::fs::write(&launcher, b"").expect("launcher placeholder must be created");
        let core = bin.join("client-agent-core.exe");

        assert_eq!(resolve_formal_startup_exe(&core), launcher);
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn parse_reg_query_output_reads_reg_sz_value() {
        let output = r#"
HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run
    WoW Client Agent    REG_SZ    "C:\WoW\client-agent.exe"
"#;

        assert_eq!(
            parse_reg_query_output(output, VALUE_NAME),
            Some(r#""C:\WoW\client-agent.exe""#.to_string())
        );
    }

    #[test]
    fn parse_reg_query_output_ignores_missing_value() {
        let output = r#"
HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run
    Other App    REG_SZ    app.exe
"#;

        assert_eq!(parse_reg_query_output(output, VALUE_NAME), None);
    }
}
