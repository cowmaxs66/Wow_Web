use crate::ps_script;
use std::fs;
use std::io;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub fn run_tray() -> io::Result<()> {
    let exe = std::env::current_exe()?;
    let work_dir = std::env::current_dir()?;
    let icon_path = work_dir.join("assets").join("icons").join("server.ico");
    let script_path = write_tray_script(
        &exe.display().to_string(),
        &work_dir.display().to_string(),
        &icon_path.display().to_string(),
        &server_url_from_env(),
    )?;
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

fn write_tray_script(
    exe_path: &str,
    work_dir: &str,
    icon_path: &str,
    server_url: &str,
) -> io::Result<PathBuf> {
    let script_dir = std::env::temp_dir().join("wow-management-server");
    fs::create_dir_all(&script_dir)?;
    let script_path = script_dir.join("tray.ps1");
    let script = r#"
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
$exe = '__EXE__'
$workDir = '__WORK_DIR__'
$iconPath = '__ICON_PATH__'
$serverUrl = '__SERVER_URL__'
$server = $null
$logDir = Join-Path $workDir 'logs'
$stdoutLog = Join-Path $logDir 'management-server.log'
$stderrLog = Join-Path $logDir 'management-server-error.log'

function Show-Balloon($title, $text) {
  $notify.BalloonTipTitle = $title
  $notify.BalloonTipText = $text
  $notify.ShowBalloonTip(4000)
}

function Find-ServerProcesses {
  Get-CimInstance Win32_Process | Where-Object {
    ($null -ne $_.ExecutablePath) -and
    [string]::Equals($_.ExecutablePath, $exe, [System.StringComparison]::OrdinalIgnoreCase) -and
    ($_.CommandLine -like '*--no-open-browser*')
  }
}

function Start-Server {
  $running = @(Find-ServerProcesses)
  if ($running.Count -gt 0) {
    Show-Balloon 'WoW Server' 'Server 已在运行'
    return
  }

  New-Item -ItemType Directory -Force -Path $logDir | Out-Null
  $script:server = Start-Process -FilePath $exe -ArgumentList '--no-open-browser' -WorkingDirectory $workDir -WindowStyle Hidden -RedirectStandardOutput $stdoutLog -RedirectStandardError $stderrLog -PassThru
  Start-Sleep -Milliseconds 900
  if ($script:server.HasExited) {
    Show-Balloon 'WoW Server' ('Server 启动失败，退出码：' + $script:server.ExitCode)
  } else {
    Show-Balloon 'WoW Server' 'Server 已启动'
  }
}

function Stop-Server {
  $running = @(Find-ServerProcesses)
  if ($running.Count -eq 0) {
    Show-Balloon 'WoW Server' 'Server 未运行'
    return
  }

  foreach ($process in $running) {
    Stop-Process -Id $process.ProcessId -Force -ErrorAction SilentlyContinue
  }
  Show-Balloon 'WoW Server' 'Server 已关闭'
}

function Restart-Server {
  Stop-Server
  Start-Sleep -Milliseconds 700
  Start-Server
}

function Open-DesktopConsole {
  $programFilesX86 = [Environment]::GetFolderPath([Environment+SpecialFolder]::ProgramFilesX86)
  $programFiles = [Environment]::GetFolderPath([Environment+SpecialFolder]::ProgramFiles)
  $edgeCandidates = @(
    (Join-Path $programFilesX86 'Microsoft\Edge\Application\msedge.exe'),
    (Join-Path $programFiles 'Microsoft\Edge\Application\msedge.exe')
  )
  $edge = $edgeCandidates | Where-Object { Test-Path -LiteralPath $_ } | Select-Object -First 1
  if ($edge) {
    Start-Process -FilePath $edge -ArgumentList @("--app=$serverUrl", '--window-size=1280,860')
    return
  }

  Start-Process -FilePath $serverUrl
}

function Open-Web {
  Start-Process -FilePath $serverUrl
}

function Open-LogDir {
  New-Item -ItemType Directory -Force -Path $logDir | Out-Null
  Start-Process -FilePath $logDir
}

function Dispose-Tray {
  $notify.Visible = $false
  $notify.Dispose()
  [System.Windows.Forms.Application]::Exit()
}

$notify = New-Object System.Windows.Forms.NotifyIcon
$notify.Text = 'WoW Management Server __VERSION__'
if (Test-Path -LiteralPath $iconPath) {
  $notify.Icon = New-Object System.Drawing.Icon($iconPath)
} else {
  $notify.Icon = [System.Drawing.SystemIcons]::Application
}
$notify.Visible = $true
$menu = New-Object System.Windows.Forms.ContextMenuStrip

function Add-Item($text, $action) {
  $item = New-Object System.Windows.Forms.ToolStripMenuItem
  $item.Text = $text
  $item.add_Click($action)
  [void]$menu.Items.Add($item)
}

Add-Item '启动 Server' { Start-Server }
Add-Item '关闭 Server' { Stop-Server }
Add-Item '重启 Server' { Restart-Server }
[void]$menu.Items.Add((New-Object System.Windows.Forms.ToolStripSeparator))
Add-Item '打开桌面控制台' { Open-DesktopConsole }
Add-Item '浏览器打开 Web 管理页' { Open-Web }
Add-Item '打开日志目录' { Open-LogDir }
[void]$menu.Items.Add((New-Object System.Windows.Forms.ToolStripSeparator))
Add-Item '仅退出托盘' { Dispose-Tray }
Add-Item '退出托盘并关闭 Server' {
  Stop-Server
  Dispose-Tray
}

$notify.ContextMenuStrip = $menu
$notify.add_DoubleClick({ Open-DesktopConsole })
Start-Server
Open-DesktopConsole
[System.Windows.Forms.Application]::Run()
"#
    .replace("__EXE__", &escape_ps_single(exe_path))
    .replace("__WORK_DIR__", &escape_ps_single(work_dir))
    .replace("__ICON_PATH__", &escape_ps_single(icon_path))
    .replace("__SERVER_URL__", &escape_ps_single(server_url))
    .replace("__VERSION__", framework_release_version());
    ps_script::write_utf8_bom(&script_path, &script)?;
    Ok(script_path)
}

fn tray_error_log() -> io::Result<fs::File> {
    let log_dir = PathBuf::from("logs");
    fs::create_dir_all(&log_dir)?;
    fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_dir.join("server-tray-error.log"))
}

fn server_url_from_env() -> String {
    let bind =
        std::env::var("MANAGEMENT_SERVER_BIND").unwrap_or_else(|_| "127.0.0.1:18080".to_string());
    server_url_from_bind(&bind).unwrap_or_else(|| "http://127.0.0.1:18080".to_string())
}

fn server_url_from_bind(bind: &str) -> Option<String> {
    let addr: SocketAddr = bind.parse().ok()?;
    let host = match addr.ip() {
        IpAddr::V4(ip) if ip.is_unspecified() => "127.0.0.1".to_string(),
        IpAddr::V4(ip) => ip.to_string(),
        IpAddr::V6(ip) if ip.is_unspecified() => "127.0.0.1".to_string(),
        IpAddr::V6(ip) => format!("[{ip}]"),
    };
    Some(format!("http://{host}:{}", addr.port()))
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

    #[test]
    fn server_url_uses_loopback_for_unspecified_bind() {
        assert_eq!(
            server_url_from_bind("0.0.0.0:18080"),
            Some("http://127.0.0.1:18080".to_string())
        );
    }

    #[test]
    fn server_url_keeps_explicit_bind_host() {
        assert_eq!(
            server_url_from_bind("127.0.0.1:18100"),
            Some("http://127.0.0.1:18100".to_string())
        );
    }
}
