param(
    [switch]$ShowMessage
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$installRoot = Join-Path $env:LOCALAPPDATA 'WoWFramework'
$programsDir = Join-Path ([Environment]::GetFolderPath('Programs')) 'WoW Framework'
$desktopDir = [Environment]::GetFolderPath('DesktopDirectory')

function Show-Info {
    param([string]$Message)

    if (-not $ShowMessage) {
        return
    }
    Add-Type -AssemblyName PresentationFramework
    [System.Windows.MessageBox]::Show($Message, 'WoW Framework Remover', 'OK', 'Information') | Out-Null
}

function Remove-IfExists {
    param([string]$Path)

    if (Test-Path -LiteralPath $Path) {
        Remove-Item -LiteralPath $Path -Recurse -Force
    }
}

Remove-IfExists (Join-Path $desktopDir 'WoW Server.lnk')
Remove-IfExists (Join-Path $desktopDir 'WoW Client.lnk')
Remove-IfExists (Join-Path $desktopDir 'WoW Framework.lnk')
Remove-IfExists $programsDir

if (Test-Path -LiteralPath $installRoot) {
    $items = Get-ChildItem -LiteralPath $installRoot -Force |
        Where-Object { $_.Name -notin @('data', 'logs') }

    foreach ($item in $items) {
        Remove-IfExists $item.FullName
    }

    $remaining = Get-ChildItem -LiteralPath $installRoot -Force -ErrorAction SilentlyContinue
    if (-not $remaining) {
        Remove-Item -LiteralPath $installRoot -Force
    }
}

Show-Info "Remove complete.`n`nProgram files and shortcuts were removed. Existing data/logs directories were preserved."
