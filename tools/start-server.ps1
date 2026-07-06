param(
    [string]$HostAddress = '127.0.0.1',
    [int]$Port = 18080,
    [string]$HistoryPath = 'data/status-history.jsonl',
    [switch]$OpenBrowser
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
. "$PSScriptRoot\common.ps1"

$root = Get-WowRoot
$serverExe = Resolve-ServerExe $root
$webDir = Resolve-WebDir $root
$url = "http://${HostAddress}:$Port"

Push-Location $root
try {
    $env:MANAGEMENT_SERVER_BIND = "${HostAddress}:$Port"
    $env:MANAGEMENT_SERVER_HISTORY_PATH = $HistoryPath
    if ($webDir) {
        $env:MANAGEMENT_SERVER_WEB_DIR = $webDir
    }

    Write-Host "Management Server: $url"
    Write-Host "History: $HistoryPath"
    if ($webDir) {
        Write-Host "Web Admin: $url"
        if ($OpenBrowser) {
            Start-Process $url
        }
    } else {
        Write-Host 'Web Admin dist 未找到，只启动 API。'
    }

    & $serverExe
} finally {
    Remove-Item Env:\MANAGEMENT_SERVER_BIND -ErrorAction SilentlyContinue
    Remove-Item Env:\MANAGEMENT_SERVER_HISTORY_PATH -ErrorAction SilentlyContinue
    Remove-Item Env:\MANAGEMENT_SERVER_WEB_DIR -ErrorAction SilentlyContinue
    Pop-Location
}
