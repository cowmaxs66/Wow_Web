use crate::local_log::LocalLog;
use crate::ps_script;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub fn open_log_window() -> io::Result<()> {
    let log = LocalLog::default();
    let script_path = write_log_script(
        &log.event_path().display().to_string(),
        &log.event_path()
            .parent()
            .unwrap_or_else(|| std::path::Path::new("logs"))
            .display()
            .to_string(),
    )?;

    let mut command = Command::new("powershell.exe");
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

fn write_log_script(log_path: &str, log_dir: &str) -> io::Result<PathBuf> {
    let script_dir = std::env::temp_dir().join("wow-client-agent");
    fs::create_dir_all(&script_dir)?;
    let script_path = script_dir.join("log-window.ps1");
    let script = LOG_WINDOW_TEMPLATE
        .replace("__LOG_PATH__", &escape_ps_single(log_path))
        .replace("__LOG_DIR__", &escape_ps_single(log_dir));
    ps_script::write_utf8_bom(&script_path, &script)?;
    Ok(script_path)
}

fn escape_ps_single(value: &str) -> String {
    value.replace('\'', "''")
}

const LOG_WINDOW_TEMPLATE: &str = r#"
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing

$logPath = '__LOG_PATH__'
$logDir = '__LOG_DIR__'

function Read-LogTail {
  if (-not (Test-Path -LiteralPath $logPath)) {
    return @('日志文件尚未创建：' + $logPath)
  }

  try {
    return @(Get-Content -LiteralPath $logPath -Tail 800 -ErrorAction Stop)
  } catch {
    return @('读取日志失败：' + $_.Exception.Message)
  }
}

function Refresh-Log {
  $lines = Read-LogTail
  $text.Lines = $lines
  $text.SelectionStart = $text.TextLength
  $text.ScrollToCaret()
  $status.Text = '最后刷新：' + (Get-Date -Format 'HH:mm:ss') + '  行数：' + $lines.Count
}

$form = New-Object System.Windows.Forms.Form
$form.Text = 'WoW Client 日志'
$form.StartPosition = 'CenterScreen'
$form.Size = New-Object System.Drawing.Size(980, 720)
$form.MinimumSize = New-Object System.Drawing.Size(720, 480)
$form.AutoScaleMode = [System.Windows.Forms.AutoScaleMode]::Dpi
$form.Font = New-Object System.Drawing.Font('Microsoft YaHei UI', 9)

$top = New-Object System.Windows.Forms.Panel
$top.Dock = 'Top'
$top.Height = 44
$top.Padding = New-Object System.Windows.Forms.Padding(10, 8, 10, 6)

$refresh = New-Object System.Windows.Forms.Button
$refresh.Text = '刷新'
$refresh.Width = 84
$refresh.Height = 28
$refresh.Left = 10
$refresh.Top = 8
$refresh.Add_Click({ Refresh-Log })

$openDir = New-Object System.Windows.Forms.Button
$openDir.Text = '打开目录'
$openDir.Width = 96
$openDir.Height = 28
$openDir.Left = 102
$openDir.Top = 8
$openDir.Add_Click({
  New-Item -ItemType Directory -Force -Path $logDir | Out-Null
  Start-Process -FilePath $logDir
})

$status = New-Object System.Windows.Forms.Label
$status.AutoSize = $true
$status.Left = 214
$status.Top = 14
$status.Text = '准备读取日志'

$top.Controls.Add($refresh)
$top.Controls.Add($openDir)
$top.Controls.Add($status)

$text = New-Object System.Windows.Forms.TextBox
$text.Multiline = $true
$text.ReadOnly = $true
$text.ScrollBars = 'Both'
$text.WordWrap = $false
$text.Dock = 'Fill'
$text.Font = New-Object System.Drawing.Font('Consolas', 10)

$timer = New-Object System.Windows.Forms.Timer
$timer.Interval = 3000
$timer.Add_Tick({ Refresh-Log })
$timer.Start()

$form.Controls.Add($text)
$form.Controls.Add($top)
$form.Add_Shown({ Refresh-Log })
[void]$form.ShowDialog()
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn powershell_single_quotes_are_escaped() {
        assert_eq!(escape_ps_single("C:\\a'b"), "C:\\a''b");
    }

    #[test]
    fn log_window_uses_dpi_scaling() {
        assert!(LOG_WINDOW_TEMPLATE.contains("AutoScaleMode"));
        assert!(LOG_WINDOW_TEMPLATE.contains("Refresh-Log"));
    }
}
