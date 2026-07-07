param(
    [switch]$ShowMessage
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$packageRoot = Split-Path -Parent $PSScriptRoot
$installRoot = Join-Path $env:LOCALAPPDATA 'WoWFramework'
$programsDir = Join-Path ([Environment]::GetFolderPath('Programs')) 'WoW Framework'
$desktopDir = [Environment]::GetFolderPath('DesktopDirectory')

function Show-Info {
    param([string]$Message)

    if (-not $ShowMessage) {
        return
    }
    Add-Type -AssemblyName PresentationFramework
    [System.Windows.MessageBox]::Show($Message, 'WoW Framework Manager', 'OK', 'Information') | Out-Null
}

function Copy-InstallItem {
    param([System.IO.FileSystemInfo]$Item)

    $destination = Join-Path $installRoot $Item.Name
    if (Test-Path -LiteralPath $destination) {
        Remove-Item -LiteralPath $destination -Recurse -Force
    }
    Copy-Item -LiteralPath $Item.FullName -Destination $destination -Recurse -Force
}

function New-AppShortcut {
    param(
        [string]$ShortcutPath,
        [string]$TargetPath,
        [string]$Description
    )

    $shell = New-Object -ComObject WScript.Shell
    $shortcut = $shell.CreateShortcut($ShortcutPath)
    $shortcut.TargetPath = $TargetPath
    $shortcut.WorkingDirectory = $installRoot
    $shortcut.Description = $Description
    $shortcut.Save()
}

New-Item -ItemType Directory -Force -Path $installRoot | Out-Null
New-Item -ItemType Directory -Force -Path $programsDir | Out-Null

$packageItems = Get-ChildItem -LiteralPath $packageRoot -Force |
    Where-Object { $_.Name -notin @('data', 'logs') }

foreach ($item in $packageItems) {
    Copy-InstallItem $item
}

$serverExe = Join-Path $installRoot 'management-server.exe'
$clientExe = Join-Path $installRoot 'client-agent.exe'
$uninstallExe = Join-Path $installRoot 'WoW-Remove.exe'

New-AppShortcut (Join-Path $desktopDir 'WoW Server.lnk') $serverExe 'Start WoW Management Server'
New-AppShortcut (Join-Path $desktopDir 'WoW Client.lnk') $clientExe 'Start WoW Client Agent'
New-AppShortcut (Join-Path $programsDir 'WoW Server.lnk') $serverExe 'Start WoW Management Server'
New-AppShortcut (Join-Path $programsDir 'WoW Client.lnk') $clientExe 'Start WoW Client Agent'
New-AppShortcut (Join-Path $programsDir 'Remove WoW Framework.lnk') $uninstallExe 'Remove WoW Framework'

Show-Info "Install complete.`n`nInstall directory: $installRoot`nDesktop and Start Menu shortcuts were created."
