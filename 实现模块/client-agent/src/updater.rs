use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;

const RELEASE_API: &str = "https://api.github.com/repos/cowmaxs66/Wow_Web/releases/latest";
const ASSET_NAME: &str = "WoW_Framework";

pub fn check_update() -> io::Result<String> {
    run_update_script(false)
}

pub fn download_update() -> io::Result<String> {
    run_update_script(true)
}

fn run_update_script(download: bool) -> io::Result<String> {
    let download_flag = if download { "$true" } else { "$false" };
    let current_version = escape_ps_single(framework_release_version());
    let download_dir = escape_ps_single(&update_dir()?.display().to_string());
    let script = format!(
        r#"
$ErrorActionPreference = 'Stop'
$current = '{current_version}'
$release = Invoke-RestMethod -Uri '{RELEASE_API}' -Headers @{{ 'User-Agent' = 'wow-client-agent' }}
$asset = $release.assets | Where-Object {{ $_.name -like '*{ASSET_NAME}*.zip' }} | Select-Object -First 1
$result = [ordered]@{{
  current = $current
  latest = $release.tag_name
  release_url = $release.html_url
  asset_name = if ($asset) {{ $asset.name }} else {{ '' }}
  downloaded_to = ''
}}
if ({download_flag} -and $asset) {{
  $dir = Join-Path '{download_dir}' $release.tag_name
  New-Item -ItemType Directory -Force -Path $dir | Out-Null
  $target = Join-Path $dir $asset.name
  Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $target
  $result.downloaded_to = $target
}}
$result | ConvertTo-Json -Compress
"#
    );

    let output = Command::new(shell_executable())
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            &script,
        ])
        .output()?;

    if !output.status.success() {
        let detail = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(io::Error::other(format!(
            "更新检查失败：{}",
            if detail.is_empty() {
                "PowerShell 未返回错误详情"
            } else {
                &detail
            }
        )));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn update_dir() -> io::Result<PathBuf> {
    let base = std::env::var("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir());
    let dir = base.join("WoWFramework").join("updates");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn framework_release_version() -> &'static str {
    include_str!("../../../VERSION").trim()
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
    fn update_metadata_uses_repo_release_api() {
        assert!(RELEASE_API.contains("cowmaxs66/Wow_Web"));
        assert!(ASSET_NAME.contains("WoW_Framework"));
    }

    #[test]
    fn powershell_single_quotes_are_escaped() {
        assert_eq!(escape_ps_single("v'1"), "v''1");
    }
}
