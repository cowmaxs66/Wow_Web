use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

const RELEASE_API: &str = "https://api.github.com/repos/cowmaxs66/Wow_Web/releases/latest";
const ASSET_NAME: &str = "WoW_Framework";
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;
#[cfg(windows)]
const DETACHED_PROCESS: u32 = 0x0000_0008;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
struct UpdateMetadata {
    current: String,
    latest: String,
    release_url: String,
    asset_name: String,
    downloaded_to: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct ApplyUpdateResult {
    current: String,
    latest: String,
    release_url: String,
    asset_name: String,
    downloaded_to: String,
    apply_script: String,
    status: String,
    message: String,
}

pub fn check_update() -> io::Result<String> {
    let metadata = query_update(false)?;
    json_string(&metadata)
}

pub fn download_update() -> io::Result<String> {
    let metadata = query_update(true)?;
    json_string(&metadata)
}

pub fn apply_update() -> io::Result<String> {
    let metadata = query_update(false)?;
    if metadata.asset_name.trim().is_empty() {
        return json_string(&ApplyUpdateResult {
            current: metadata.current.clone(),
            latest: metadata.latest.clone(),
            release_url: metadata.release_url.clone(),
            asset_name: metadata.asset_name.clone(),
            downloaded_to: String::new(),
            apply_script: String::new(),
            status: "no_asset".to_string(),
            message: "latest release has no matching WoW_Framework zip asset".to_string(),
        });
    }

    if !is_newer_version(&metadata.latest, &metadata.current) {
        return json_string(&ApplyUpdateResult {
            current: metadata.current.clone(),
            latest: metadata.latest.clone(),
            release_url: metadata.release_url.clone(),
            asset_name: metadata.asset_name.clone(),
            downloaded_to: String::new(),
            apply_script: String::new(),
            status: "up_to_date".to_string(),
            message: "current version is not older than latest release".to_string(),
        });
    }

    let downloaded = query_update(true)?;
    if downloaded.downloaded_to.trim().is_empty() {
        return Err(io::Error::other("更新包下载路径为空，无法安排自替换"));
    }

    let install_root = detect_install_root()?;
    let script_dir = update_dir()?.join(&downloaded.latest);
    fs::create_dir_all(&script_dir)?;
    let apply_script = script_dir.join("apply-update.ps1");
    fs::write(&apply_script, apply_script_content())?;
    let backup_root = update_dir()?.join("backups");
    fs::create_dir_all(&backup_root)?;
    spawn_apply_script(
        &apply_script,
        Path::new(&downloaded.downloaded_to),
        &install_root,
        &backup_root,
    )?;

    json_string(&ApplyUpdateResult {
        current: downloaded.current,
        latest: downloaded.latest,
        release_url: downloaded.release_url,
        asset_name: downloaded.asset_name,
        downloaded_to: downloaded.downloaded_to,
        apply_script: apply_script.display().to_string(),
        status: "scheduled".to_string(),
        message: "update apply script has been started".to_string(),
    })
}

fn query_update(download: bool) -> io::Result<UpdateMetadata> {
    let json = run_update_script(download)?;
    serde_json::from_str(&json).map_err(|error| {
        io::Error::other(format!(
            "更新元数据 JSON 解析失败：{error}；原始输出：{json}"
        ))
    })
}

fn json_string<T: Serialize>(value: &T) -> io::Result<String> {
    serde_json::to_string(value).map_err(|error| io::Error::other(error.to_string()))
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

fn detect_install_root() -> io::Result<PathBuf> {
    let exe = std::env::current_exe()?;
    let root = detect_install_root_from_exe(&exe)
        .ok_or_else(|| io::Error::other("当前程序不在正式发布包目录中，拒绝执行自替换更新"))?;

    if !root.join("VERSION").exists()
        || !root.join("client-agent.exe").exists()
        || !root.join("bin").exists()
    {
        return Err(io::Error::other(
            "当前目录不是完整发布包安装目录，拒绝执行自替换更新",
        ));
    }

    Ok(root)
}

fn detect_install_root_from_exe(exe: &Path) -> Option<PathBuf> {
    let exe_dir = exe.parent()?;
    if exe_dir
        .file_name()
        .is_some_and(|name| name.eq_ignore_ascii_case("bin"))
    {
        return exe_dir.parent().map(PathBuf::from);
    }

    Some(exe_dir.to_path_buf())
}

fn spawn_apply_script(
    script_path: &Path,
    zip_path: &Path,
    install_root: &Path,
    backup_root: &Path,
) -> io::Result<()> {
    let mut command = Command::new(shell_executable());
    command.args([
        "-NoProfile",
        "-ExecutionPolicy",
        "Bypass",
        "-File",
        &script_path.display().to_string(),
        "-ZipPath",
        &zip_path.display().to_string(),
        "-InstallRoot",
        &install_root.display().to_string(),
        "-BackupRoot",
        &backup_root.display().to_string(),
        "-RestartClient",
    ]);

    #[cfg(windows)]
    {
        command.creation_flags(CREATE_NO_WINDOW | DETACHED_PROCESS);
    }

    command.spawn().map(|_| ())
}

fn apply_script_content() -> &'static str {
    r#"
param(
    [Parameter(Mandatory = $true)][string]$ZipPath,
    [Parameter(Mandatory = $true)][string]$InstallRoot,
    [Parameter(Mandatory = $true)][string]$BackupRoot,
    [switch]$RestartClient
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$logDir = Join-Path $InstallRoot 'logs'
New-Item -ItemType Directory -Force -Path $logDir | Out-Null
$logPath = Join-Path $logDir 'update-apply.log'

function Write-UpdateLog {
    param([string]$Message)
    $line = ('{0:u} {1}' -f (Get-Date), $Message)
    Add-Content -LiteralPath $logPath -Value $line
}

function Get-NormalizedRoot {
    param([string]$Path)
    $full = [System.IO.Path]::GetFullPath($Path)
    if (-not $full.EndsWith([System.IO.Path]::DirectorySeparatorChar)) {
        $full += [System.IO.Path]::DirectorySeparatorChar
    }
    $full
}

try {
    Write-UpdateLog 'update apply started'
    Start-Sleep -Seconds 2

    $installFull = Get-NormalizedRoot $InstallRoot
    foreach ($process in Get-Process) {
        try {
            $processPath = $process.MainModule.FileName
        } catch {
            continue
        }
        if ([string]::IsNullOrWhiteSpace($processPath)) {
            continue
        }
        $processFull = [System.IO.Path]::GetFullPath($processPath)
        if ($process.Id -ne $PID -and $processFull.StartsWith($installFull, [System.StringComparison]::OrdinalIgnoreCase)) {
            Write-UpdateLog ('stopping process ' + $process.Id + ' ' + $processFull)
            Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
        }
    }

    Start-Sleep -Seconds 1
    $stage = Join-Path $env:TEMP ('wow-update-stage-' + [guid]::NewGuid().ToString('N'))
    New-Item -ItemType Directory -Force -Path $stage | Out-Null
    Expand-Archive -LiteralPath $ZipPath -DestinationPath $stage -Force

    if (Test-Path -LiteralPath (Join-Path $stage 'VERSION')) {
        $sourceRoot = $stage
    } else {
        $children = @(Get-ChildItem -LiteralPath $stage -Force)
        if ($children.Count -eq 1 -and $children[0].PSIsContainer -and (Test-Path -LiteralPath (Join-Path $children[0].FullName 'VERSION'))) {
            $sourceRoot = $children[0].FullName
        } else {
            throw 'extracted package does not contain VERSION at expected location'
        }
    }

    New-Item -ItemType Directory -Force -Path $BackupRoot | Out-Null
    $backup = Join-Path $BackupRoot (Get-Date -Format 'yyyyMMdd-HHmmss')
    New-Item -ItemType Directory -Force -Path $backup | Out-Null

    $preserve = @('data', 'logs', 'updates')
    $currentItems = Get-ChildItem -LiteralPath $InstallRoot -Force | Where-Object { $_.Name -notin $preserve }
    foreach ($item in $currentItems) {
        Copy-Item -LiteralPath $item.FullName -Destination (Join-Path $backup $item.Name) -Recurse -Force
        Remove-Item -LiteralPath $item.FullName -Recurse -Force
    }

    foreach ($item in Get-ChildItem -LiteralPath $sourceRoot -Force) {
        Copy-Item -LiteralPath $item.FullName -Destination (Join-Path $InstallRoot $item.Name) -Recurse -Force
    }

    Remove-Item -LiteralPath $stage -Recurse -Force
    Write-UpdateLog ('update apply completed; backup=' + $backup)

    if ($RestartClient) {
        $client = Join-Path $InstallRoot 'client-agent.exe'
        if (Test-Path -LiteralPath $client) {
            Start-Process -FilePath $client -WorkingDirectory $InstallRoot
        }
    }
} catch {
    Write-UpdateLog ('update apply failed: ' + $_.Exception.Message)
    throw
}
"#
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

fn is_newer_version(latest: &str, current: &str) -> bool {
    parse_version_segments(latest) > parse_version_segments(current)
}

fn parse_version_segments(version: &str) -> Vec<u64> {
    version
        .trim()
        .trim_start_matches('v')
        .split(['.', '-', '+'])
        .map(|segment| {
            segment
                .chars()
                .take_while(|ch| ch.is_ascii_digit())
                .collect::<String>()
        })
        .take_while(|segment| !segment.is_empty())
        .map(|segment| segment.parse::<u64>().unwrap_or(0))
        .collect()
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

    #[test]
    fn version_compare_handles_minor_rollover() {
        assert!(is_newer_version("v1.10.0", "v1.9.0"));
        assert!(!is_newer_version("v1.9.0", "v1.10.0"));
        assert!(!is_newer_version("v1.10.0", "v1.10.0"));
    }

    #[test]
    fn install_root_for_core_is_package_root() {
        let root = PathBuf::from(r"C:\WoWFramework");
        let core = root.join("bin").join("client-agent-core.exe");

        assert_eq!(detect_install_root_from_exe(&core), Some(root));
    }

    #[test]
    fn apply_script_preserves_runtime_data() {
        let script = apply_script_content();

        assert!(script.contains("'data', 'logs', 'updates'"));
        assert!(script.contains("update-apply.log"));
        assert!(script.contains("Expand-Archive"));
    }
}
