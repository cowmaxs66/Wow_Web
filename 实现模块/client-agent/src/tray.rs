use crate::ps_script;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub fn run_tray() -> io::Result<()> {
    let exe = std::env::current_exe()?;
    let icon_path = std::env::current_dir()?
        .join("assets")
        .join("icons")
        .join("client.ico");
    let script_path =
        write_tray_script(&exe.display().to_string(), &icon_path.display().to_string())?;
    let stderr = tray_error_log()?;
    let mut command = Command::new(shell_executable());
    command.args([
        "-STA",
        "-NoProfile",
        "-ExecutionPolicy",
        "Bypass",
        "-File",
        &script_path.display().to_string(),
    ]);
    command.stderr(Stdio::from(stderr));
    spawn_hidden(command)
}

fn write_tray_script(exe_path: &str, icon_path: &str) -> io::Result<PathBuf> {
    let script_dir = std::env::temp_dir().join("wow-client-agent");
    fs::create_dir_all(&script_dir)?;
    let script_path = script_dir.join("tray.ps1");
    let exe = escape_ps_single(exe_path);
    let icon_path = escape_ps_single(icon_path);
    let current_version = escape_ps_single(framework_release_version());
    let script = format!(
        r#"
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
$exe = '{exe}'
$iconPath = '{icon_path}'
$currentVersion = '{current_version}'
$monitor = $null

function Show-Balloon($title, $text) {{
  $notify.BalloonTipTitle = $title
  $notify.BalloonTipText = $text
  $notify.ShowBalloonTip(4000)
}}

function Run-Agent($arguments, $title) {{
  try {{
    $process = Start-Process -FilePath $exe -ArgumentList $arguments -WindowStyle Hidden -PassThru -Wait
    Show-Balloon $title ('完成，退出码：' + $process.ExitCode)
  }} catch {{
    Show-Balloon $title $_.Exception.Message
  }}
}}

function Start-Monitor {{
  if ($script:monitor -and -not $script:monitor.HasExited) {{
    Show-Balloon 'WoW Client' 'monitor 已在运行'
    return
  }}
  $script:monitor = Start-Process -FilePath $exe -ArgumentList '--monitor' -WindowStyle Hidden -PassThru
  Show-Balloon 'WoW Client' 'monitor 已启动'
}}

function Report-Offline {{
  try {{
    Start-Process -FilePath $exe -ArgumentList '--report-offline' -WindowStyle Hidden -Wait
  }} catch {{
    Show-Balloon 'WoW Client' ('离线状态上报失败：' + $_.Exception.Message)
  }}
}}

function Stop-Monitor {{
  if ($script:monitor -and -not $script:monitor.HasExited) {{
    Stop-Process -Id $script:monitor.Id -Force
    Report-Offline
    Show-Balloon 'WoW Client' 'monitor 已停止'
  }} else {{
    Report-Offline
    Show-Balloon 'WoW Client' 'monitor 未运行'
  }}
}}

$notify = New-Object System.Windows.Forms.NotifyIcon
$notify.Text = 'WoW Client Agent ' + $currentVersion
if (Test-Path -LiteralPath $iconPath) {{
  $notify.Icon = New-Object System.Drawing.Icon($iconPath)
}} else {{
  $notify.Icon = [System.Drawing.SystemIcons]::Application
}}
$notify.Visible = $true
$menu = New-Object System.Windows.Forms.ContextMenuStrip

function Add-Item($text, $action) {{
  $item = New-Object System.Windows.Forms.ToolStripMenuItem
  $item.Text = $text
  $item.add_Click($action)
  [void]$menu.Items.Add($item)
}}

Add-Item '启动 Monitor' {{ Start-Monitor }}
Add-Item '停止 Monitor' {{ Stop-Monitor }}
[void]$menu.Items.Add((New-Object System.Windows.Forms.ToolStripSeparator))
Add-Item '打开设置窗口' {{ Start-Process -FilePath $exe -ArgumentList '--settings-window' -WindowStyle Hidden }}
Add-Item '查看日志窗口' {{ Start-Process -FilePath $exe -ArgumentList '--log-window' -WindowStyle Hidden }}
Add-Item '用默认程序打开日志' {{ Start-Process -FilePath $exe -ArgumentList '--open-log' -WindowStyle Hidden }}
[void]$menu.Items.Add((New-Object System.Windows.Forms.ToolStripSeparator))
Add-Item '查询开机启动' {{ Run-Agent '--startup-status' '开机启动状态' }}
Add-Item '启用开机启动' {{ Run-Agent '--enable-startup' '开机启动' }}
Add-Item '停用开机启动' {{ Run-Agent '--disable-startup' '开机启动' }}
[void]$menu.Items.Add((New-Object System.Windows.Forms.ToolStripSeparator))
Add-Item 'Service 状态' {{ Run-Agent '--service-status' 'Service 状态' }}
Add-Item '安装 Service' {{ Run-Agent '--service-install' 'Service 安装' }}
Add-Item '启动 Service' {{ Run-Agent '--service-start' 'Service 启动' }}
Add-Item '停止 Service' {{ Run-Agent '--service-stop' 'Service 停止' }}
Add-Item '卸载 Service' {{ Run-Agent '--service-uninstall' 'Service 卸载' }}
[void]$menu.Items.Add((New-Object System.Windows.Forms.ToolStripSeparator))
Add-Item '检查更新' {{ Run-Agent '--update-check' '更新检查' }}
Add-Item '下载更新包' {{ Run-Agent '--update-download' '更新下载' }}
Add-Item '安装更新并退出' {{
  Stop-Monitor
  Run-Agent '--update-apply' '安装更新'
  $notify.Visible = $false
  $notify.Dispose()
  [System.Windows.Forms.Application]::Exit()
}}
[void]$menu.Items.Add((New-Object System.Windows.Forms.ToolStripSeparator))
Add-Item '退出托盘' {{
  Stop-Monitor
  $notify.Visible = $false
  $notify.Dispose()
  [System.Windows.Forms.Application]::Exit()
}}

$notify.ContextMenuStrip = $menu
$timer = New-Object System.Windows.Forms.Timer
$timer.Interval = 21600000
$timer.Add_Tick({{ Run-Agent '--update-check' '自动更新检查' }})
$timer.Start()
Start-Monitor
[System.Windows.Forms.Application]::Run()
"#
    );
    ps_script::write_utf8_bom(&script_path, &script)?;
    Ok(script_path)
}

fn tray_error_log() -> io::Result<fs::File> {
    let log_dir = PathBuf::from("logs");
    fs::create_dir_all(&log_dir)?;
    fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_dir.join("tray-error.log"))
}

fn shell_executable() -> &'static str {
    "powershell"
}

fn spawn_hidden(mut command: Command) -> io::Result<()> {
    #[cfg(windows)]
    {
        command.creation_flags(CREATE_NO_WINDOW);
    }

    command.spawn().map(|_| ())
}

fn framework_release_version() -> &'static str {
    include_str!("../../../VERSION").trim()
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
}
