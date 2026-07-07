use std::io;
use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;
#[cfg(windows)]
const DETACHED_PROCESS: u32 = 0x0000_0008;

pub fn notify(title: &str, body: &str) -> io::Result<()> {
    #[cfg(windows)]
    {
        notify_windows(title, body)
    }

    #[cfg(not(windows))]
    {
        let _ = (title, body);
        Ok(())
    }
}

#[cfg(windows)]
fn notify_windows(title: &str, body: &str) -> io::Result<()> {
    let mut probe = Command::new("pwsh");
    probe
        .arg("-NoProfile")
        .arg("-WindowStyle")
        .arg("Hidden")
        .arg("-Command")
        .arg("$PSVersionTable.PSVersion | Out-Null");
    let shell = if status_hidden(probe)
        .map(|status| status.success())
        .unwrap_or(false)
    {
        "pwsh"
    } else {
        "powershell"
    };
    let title = quote_ps(title);
    let body = quote_ps(body);
    let script = format!(
        "Add-Type -AssemblyName System.Windows.Forms; \
         Add-Type -AssemblyName System.Drawing; \
         $n = New-Object System.Windows.Forms.NotifyIcon; \
         $n.Icon = [System.Drawing.SystemIcons]::Information; \
         $n.Visible = $true; \
         $n.BalloonTipTitle = {title}; \
         $n.BalloonTipText = {body}; \
         $n.ShowBalloonTip(5000); \
         Start-Sleep -Seconds 6; \
         $n.Dispose();"
    );

    // 使用系统托盘气泡作为 P11 最小通知能力。
    // 输入：Server 消息或 Client 状态摘要。
    // 输出：Windows 右下角通知气泡。
    // 边界：持久托盘图标、右键菜单和设置窗口后续用专门 UI 阶段实现。
    let mut command = Command::new(shell);
    command.args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &script]);
    spawn_hidden(command)
}

#[cfg(windows)]
fn quote_ps(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

#[cfg(windows)]
fn spawn_hidden(mut command: Command) -> io::Result<()> {
    command.creation_flags(CREATE_NO_WINDOW | DETACHED_PROCESS);
    command.spawn().map(|_| ())
}

#[cfg(windows)]
fn status_hidden(mut command: Command) -> io::Result<std::process::ExitStatus> {
    command.creation_flags(CREATE_NO_WINDOW | DETACHED_PROCESS);
    command.status()
}
