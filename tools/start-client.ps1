param(
    [string]$ServerHost = '127.0.0.1',
    [int]$ServerPort = 18080,
    [ValidateSet('x64', 'x86')]
    [string]$ClientArch = 'x86',
    [string]$DmBridgePath = 'dm-bridge/Win32/DmBridge.dll',
    [switch]$DisableReport,
    [switch]$Monitor,
    [switch]$Notify,
    [switch]$OpenLog,
    [switch]$Setup,
    [switch]$StartupStatus,
    [switch]$EnableStartup,
    [switch]$DisableStartup,
    [switch]$Tray,
    [switch]$SettingsWindow,
    [switch]$ServiceInstall,
    [switch]$ServiceUninstall,
    [switch]$ServiceStart,
    [switch]$ServiceStop,
    [switch]$ServiceStatus,
    [switch]$UpdateCheck,
    [switch]$UpdateDownload
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
. "$PSScriptRoot\common.ps1"

$root = Get-WowRoot
$clientExe = Resolve-ClientExe -Root $root -Arch $ClientArch

Push-Location $root
try {
    $env:CLIENT_AGENT_SERVER_ENABLED = if ($DisableReport) { '0' } else { '1' }
    $env:CLIENT_AGENT_SERVER_HOST = $ServerHost
    $env:CLIENT_AGENT_SERVER_PORT = [string]$ServerPort

    if ($ClientArch -eq 'x86') {
        $env:DM_BRIDGE_DLL = $DmBridgePath
        Write-Host 'Client 模式：x86 DM。请确认本机已准备 32 位 dm.dll 注册与授权。'
    } else {
        Write-Host 'Client 模式：x64 核心。不会直接加载 32 位大漠 DLL。'
    }

    $arguments = @()
    if ($Monitor) {
        $arguments += '--monitor'
    } elseif ($Setup) {
        $arguments += '--setup'
    } elseif ($OpenLog) {
        $arguments += '--open-log'
    } elseif ($StartupStatus) {
        $arguments += '--startup-status'
    } elseif ($EnableStartup) {
        $arguments += '--enable-startup'
    } elseif ($DisableStartup) {
        $arguments += '--disable-startup'
    } elseif ($Tray) {
        $arguments += '--tray'
    } elseif ($SettingsWindow) {
        $arguments += '--settings-window'
    } elseif ($ServiceInstall) {
        $arguments += '--service-install'
    } elseif ($ServiceUninstall) {
        $arguments += '--service-uninstall'
    } elseif ($ServiceStart) {
        $arguments += '--service-start'
    } elseif ($ServiceStop) {
        $arguments += '--service-stop'
    } elseif ($ServiceStatus) {
        $arguments += '--service-status'
    } elseif ($UpdateCheck) {
        $arguments += '--update-check'
    } elseif ($UpdateDownload) {
        $arguments += '--update-download'
    } elseif ($Notify) {
        $arguments += '--notify'
    }

    & $clientExe @arguments
} finally {
    Remove-Item Env:\CLIENT_AGENT_SERVER_ENABLED -ErrorAction SilentlyContinue
    Remove-Item Env:\CLIENT_AGENT_SERVER_HOST -ErrorAction SilentlyContinue
    Remove-Item Env:\CLIENT_AGENT_SERVER_PORT -ErrorAction SilentlyContinue
    Remove-Item Env:\DM_BRIDGE_DLL -ErrorAction SilentlyContinue
    Pop-Location
}
