use std::io;
use std::process::Command;

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
    let shell = if Command::new("pwsh")
        .arg("-NoProfile")
        .arg("-Command")
        .arg("$PSVersionTable.PSVersion | Out-Null")
        .status()
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
    Command::new(shell)
        .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &script])
        .spawn()
        .map(|_| ())
}

#[cfg(windows)]
fn quote_ps(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}
