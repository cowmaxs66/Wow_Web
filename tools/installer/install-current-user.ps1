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
    $sourceFull = [System.IO.Path]::GetFullPath($Item.FullName)
    $destinationFull = [System.IO.Path]::GetFullPath($destination)

    # 控制中心允许从已安装目录再次执行“安装 / 修复”。
    # 输入：当前发布包条目。
    # 输出：复制到当前用户安装目录。
    # 边界：源路径和目标路径相同时必须跳过，否则会先删除自身再复制。
    if ([string]::Equals($sourceFull, $destinationFull, [System.StringComparison]::OrdinalIgnoreCase)) {
        return
    }

    if (Test-Path -LiteralPath $destination) {
        Remove-Item -LiteralPath $destination -Recurse -Force
    }
    Copy-Item -LiteralPath $Item.FullName -Destination $destination -Recurse -Force
}

function New-AppShortcut {
    param(
        [string]$ShortcutPath,
        [string]$TargetPath,
        [string]$Description,
        [string]$IconPath = ''
    )

    $shell = New-Object -ComObject WScript.Shell
    $shortcut = $shell.CreateShortcut($ShortcutPath)
    $shortcut.TargetPath = $TargetPath
    $shortcut.WorkingDirectory = $installRoot
    $shortcut.Description = $Description
    if (-not [string]::IsNullOrWhiteSpace($IconPath) -and (Test-Path -LiteralPath $IconPath)) {
        $shortcut.IconLocation = $IconPath
    }
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
$managerExe = Join-Path $installRoot 'WoW-Manager.exe'
$uninstallExe = Join-Path $installRoot 'WoW-Remove.exe'
$serverIcon = Join-Path $installRoot 'assets\icons\server.ico'
$clientIcon = Join-Path $installRoot 'assets\icons\client.ico'

New-AppShortcut (Join-Path $desktopDir 'WoW Framework.lnk') $managerExe 'Open WoW Framework Control Center' $serverIcon
New-AppShortcut (Join-Path $desktopDir 'WoW Server.lnk') $serverExe 'Start WoW Management Server' $serverIcon
New-AppShortcut (Join-Path $desktopDir 'WoW Client.lnk') $clientExe 'Start WoW Client Agent' $clientIcon
New-AppShortcut (Join-Path $programsDir 'WoW Framework.lnk') $managerExe 'Open WoW Framework Control Center' $serverIcon
New-AppShortcut (Join-Path $programsDir 'WoW Server.lnk') $serverExe 'Start WoW Management Server' $serverIcon
New-AppShortcut (Join-Path $programsDir 'WoW Client.lnk') $clientExe 'Start WoW Client Agent' $clientIcon
New-AppShortcut (Join-Path $programsDir 'Remove WoW Framework.lnk') $uninstallExe 'Remove WoW Framework'

Show-Info "Install complete.`n`nInstall directory: $installRoot`nDesktop and Start Menu shortcuts were created."
