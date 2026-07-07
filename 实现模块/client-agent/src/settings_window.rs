use crate::config::default_config_path;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn open_settings_window() -> io::Result<()> {
    let config_path = default_config_path();
    ensure_config_exists(&config_path)?;
    let script_path = write_settings_script(&config_path)?;

    Command::new(shell_executable())
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            &script_path.display().to_string(),
        ])
        .spawn()
        .map(|_| ())
}

fn ensure_config_exists(config_path: &Path) -> io::Result<()> {
    if config_path.exists() {
        return Ok(());
    }

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(config_path, include_str!("../config/client-agent.toml"))
}

fn write_settings_script(config_path: &Path) -> io::Result<PathBuf> {
    let script_dir = std::env::temp_dir().join("wow-client-agent");
    fs::create_dir_all(&script_dir)?;
    let script_path = script_dir.join("settings-window.ps1");
    let config = escape_ps_single(&config_path.display().to_string());
    let script = format!(
        r#"
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
$configPath = '{config}'
$form = New-Object System.Windows.Forms.Form
$form.Text = 'WoW Client 本机设置'
$form.Width = 760
$form.Height = 620
$form.StartPosition = 'CenterScreen'

$label = New-Object System.Windows.Forms.Label
$label.Text = '编辑 client-agent.toml。保存后 monitor / service 下次刷新会读取新配置。'
$label.Left = 12
$label.Top = 12
$label.Width = 700
$label.Height = 24
$form.Controls.Add($label)

$editor = New-Object System.Windows.Forms.TextBox
$editor.Multiline = $true
$editor.ScrollBars = 'Both'
$editor.WordWrap = $false
$editor.Font = New-Object System.Drawing.Font('Consolas', 10)
$editor.Left = 12
$editor.Top = 44
$editor.Width = 720
$editor.Height = 460
$editor.Text = [System.IO.File]::ReadAllText($configPath, [System.Text.Encoding]::UTF8)
$form.Controls.Add($editor)

$status = New-Object System.Windows.Forms.Label
$status.Left = 12
$status.Top = 516
$status.Width = 520
$status.Height = 24
$form.Controls.Add($status)

$save = New-Object System.Windows.Forms.Button
$save.Text = '保存'
$save.Left = 536
$save.Top = 512
$save.Width = 88
$save.Add_Click({{
  [System.IO.File]::WriteAllText($configPath, $editor.Text, [System.Text.Encoding]::UTF8)
  $status.Text = '已保存：' + (Get-Date).ToString('HH:mm:ss')
}})
$form.Controls.Add($save)

$reload = New-Object System.Windows.Forms.Button
$reload.Text = '重新读取'
$reload.Left = 632
$reload.Top = 512
$reload.Width = 100
$reload.Add_Click({{
  $editor.Text = [System.IO.File]::ReadAllText($configPath, [System.Text.Encoding]::UTF8)
  $status.Text = '已重新读取'
}})
$form.Controls.Add($reload)

[void]$form.ShowDialog()
"#
    );
    fs::write(&script_path, script)?;
    Ok(script_path)
}

fn shell_executable() -> &'static str {
    if Command::new("pwsh")
        .arg("-NoProfile")
        .arg("-Command")
        .arg("$PSVersionTable.PSVersion.ToString()")
        .output()
        .is_ok_and(|output| output.status.success())
    {
        "pwsh"
    } else {
        "powershell"
    }
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
