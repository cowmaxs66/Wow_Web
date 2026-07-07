param(
    [string]$Version = '',
    [switch]$SkipBuild
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
if ([string]::IsNullOrWhiteSpace($Version)) {
    $Version = (Get-Content -LiteralPath (Join-Path $root 'VERSION') -Raw).Trim()
}

$packageBase = Join-Path $root 'target\release-package'
$packageRoot = Join-Path $packageBase "WoW_Framework_${Version}_windows"
$zipPath = Join-Path $packageBase "WoW_Framework_${Version}_windows.zip"
$serverPackageRoot = Join-Path $packageBase "WoW_Server_${Version}_windows"
$serverZipPath = Join-Path $packageBase "WoW_Server_${Version}_windows.zip"
$clientPackageRoot = Join-Path $packageBase "WoW_Client_${Version}_windows"
$clientZipPath = Join-Path $packageBase "WoW_Client_${Version}_windows.zip"

function Assert-InWorkspace {
    param([string]$Path)

    $full = [System.IO.Path]::GetFullPath($Path)
    $workspace = [System.IO.Path]::GetFullPath($root)
    if (-not $full.StartsWith($workspace, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "Refuse to operate outside workspace: $full"
    }
}

function Reset-PackageDirectory {
    Assert-InWorkspace $packageBase
    $paths = @(
        $packageRoot,
        $zipPath,
        $serverPackageRoot,
        $serverZipPath,
        $clientPackageRoot,
        $clientZipPath
    )
    foreach ($path in $paths) {
        if (Test-Path -LiteralPath $path) {
            Assert-InWorkspace $path
            Remove-Item -LiteralPath $path -Recurse -Force
        }
    }
    New-Item -ItemType Directory -Force -Path $packageRoot | Out-Null
    New-Item -ItemType Directory -Force -Path (Join-Path $packageRoot 'bin') | Out-Null
    New-Item -ItemType Directory -Force -Path $serverPackageRoot | Out-Null
    New-Item -ItemType Directory -Force -Path (Join-Path $serverPackageRoot 'bin') | Out-Null
    New-Item -ItemType Directory -Force -Path $clientPackageRoot | Out-Null
    New-Item -ItemType Directory -Force -Path (Join-Path $clientPackageRoot 'bin') | Out-Null
}

function Copy-RequiredFile {
    param(
        [string]$Source,
        [string]$Destination
    )

    if (-not (Test-Path -LiteralPath $Source)) {
        throw "Missing required file: $Source"
    }
    New-Item -ItemType Directory -Force -Path (Split-Path -Parent $Destination) | Out-Null
    Copy-Item -LiteralPath $Source -Destination $Destination -Force
}

function Copy-RequiredDirectory {
    param(
        [string]$Source,
        [string]$Destination
    )

    if (-not (Test-Path -LiteralPath $Source)) {
        throw "Missing required directory: $Source"
    }
    if (Test-Path -LiteralPath $Destination) {
        Remove-Item -LiteralPath $Destination -Recurse -Force
    }
    Copy-Item -LiteralPath $Source -Destination $Destination -Recurse -Force
}

function Invoke-Build {
    Push-Location $root
    try {
        Push-Location (Join-Path $root '实现模块\web-admin')
        try {
            npm run build
        } finally {
            Pop-Location
        }

        cargo build --workspace --release
        rustup target add i686-pc-windows-msvc
        cargo build -p client-agent --release --target i686-pc-windows-msvc
        & (Join-Path $root '实现模块\dm-bridge\build.ps1')
    } finally {
        Pop-Location
    }
}

function Copy-PackagePayload {
    $releaseDir = Join-Path $root 'target\release'
    $x86Dir = Join-Path $root 'target\i686-pc-windows-msvc\release'
    $binDir = Join-Path $packageRoot 'bin'

    # Root keeps GUI launchers only. Maintenance console binaries stay in bin.
    Copy-RequiredFile (Join-Path $releaseDir 'wow-server-launcher.exe') (Join-Path $packageRoot 'management-server.exe')
    Copy-RequiredFile (Join-Path $releaseDir 'wow-client-launcher.exe') (Join-Path $packageRoot 'client-agent.exe')
    Copy-RequiredFile (Join-Path $releaseDir 'wow-user-provision.exe') (Join-Path $packageRoot 'WoW-Manager.exe')
    Copy-RequiredFile (Join-Path $releaseDir 'wow-user-remove.exe') (Join-Path $packageRoot 'WoW-Remove.exe')

    Copy-RequiredFile (Join-Path $releaseDir 'management-server.exe') (Join-Path $binDir 'management-server-core.exe')
    Copy-RequiredFile (Join-Path $x86Dir 'client-agent.exe') (Join-Path $binDir 'client-agent-core.exe')
    Copy-RequiredFile (Join-Path $releaseDir 'client-agent.exe') (Join-Path $binDir 'client-agent-x64-core.exe')

    Copy-RequiredFile (Join-Path $root 'target\dm-bridge\Win32\DmBridge.dll') (Join-Path $packageRoot 'dm-bridge\Win32\DmBridge.dll')
    Write-PackagedClientConfig $packageRoot
    Copy-RequiredDirectory (Join-Path $root '实现模块\client-agent\scripts') (Join-Path $packageRoot 'scripts')
    Copy-RequiredDirectory (Join-Path $root 'tools') (Join-Path $packageRoot 'tools')
    Copy-RequiredDirectory (Join-Path $root 'tools\installer') (Join-Path $packageRoot 'installer')
    Copy-IconAssets $packageRoot @('server.ico', 'client.ico', 'lua_ai_server_icon.svg', 'lua_ai_client_icon.svg')
    Copy-RequiredFile (Join-Path $root 'README.md') (Join-Path $packageRoot 'README.md')
    Copy-RequiredFile (Join-Path $root 'VERSION') (Join-Path $packageRoot 'VERSION')
}

function Copy-ToolFiles {
    param(
        [string]$Destination,
        [string[]]$Names
    )

    New-Item -ItemType Directory -Force -Path $Destination | Out-Null
    foreach ($name in $Names) {
        Copy-RequiredFile (Join-Path $root "tools\$name") (Join-Path $Destination $name)
    }
}

function Copy-IconAssets {
    param(
        [string]$DestinationRoot,
        [string[]]$Names
    )

    foreach ($name in $Names) {
        Copy-RequiredFile (Join-Path $root "assets\icons\$name") (Join-Path $DestinationRoot "assets\icons\$name")
    }
}

function Write-PackagedClientConfig {
    param([string]$TargetRoot)

    $sourcePath = Join-Path $root '实现模块\client-agent\config\client-agent.toml'
    if (-not (Test-Path -LiteralPath $sourcePath)) {
        throw "Missing required file: $sourcePath"
    }

    $destination = Join-Path $TargetRoot 'config\client-agent.toml'
    New-Item -ItemType Directory -Force -Path (Split-Path -Parent $destination) | Out-Null
    $content = Get-Content -LiteralPath $sourcePath -Raw
    $content = $content.Replace('bridge_path = "../../target/dm-bridge/Win32/DmBridge.dll"', 'bridge_path = "dm-bridge/Win32/DmBridge.dll"')
    $content = $content.Replace("enabled = false`r`nhost = `"127.0.0.1`"", "enabled = true`r`nhost = `"127.0.0.1`"")
    $content = $content.Replace("enabled = false`nhost = `"127.0.0.1`"", "enabled = true`nhost = `"127.0.0.1`"")
    Set-Content -LiteralPath $destination -Value $content -Encoding UTF8
}

function Copy-SplitPackagePayload {
    $releaseDir = Join-Path $root 'target\release'
    $x86Dir = Join-Path $root 'target\i686-pc-windows-msvc\release'

    # Server 分包只包含服务端运行所需文件，避免把 Client 配置和脚本带入服务端目录。
    Copy-RequiredFile (Join-Path $releaseDir 'wow-server-launcher.exe') (Join-Path $serverPackageRoot 'management-server.exe')
    Copy-RequiredFile (Join-Path $releaseDir 'management-server.exe') (Join-Path $serverPackageRoot 'bin\management-server-core.exe')
    Copy-ToolFiles (Join-Path $serverPackageRoot 'tools') @('common.ps1', 'start-server.ps1')
    Copy-IconAssets $serverPackageRoot @('server.ico', 'lua_ai_server_icon.svg')
    Copy-RequiredFile (Join-Path $root 'README.md') (Join-Path $serverPackageRoot 'README.md')
    Copy-RequiredFile (Join-Path $root 'VERSION') (Join-Path $serverPackageRoot 'VERSION')

    # Client 分包保留 x86 默认入口、x64 core、DmBridge、配置、脚本和 Client 工具脚本。
    Copy-RequiredFile (Join-Path $releaseDir 'wow-client-launcher.exe') (Join-Path $clientPackageRoot 'client-agent.exe')
    Copy-RequiredFile (Join-Path $x86Dir 'client-agent.exe') (Join-Path $clientPackageRoot 'bin\client-agent-core.exe')
    Copy-RequiredFile (Join-Path $releaseDir 'client-agent.exe') (Join-Path $clientPackageRoot 'bin\client-agent-x64-core.exe')
    Copy-RequiredFile (Join-Path $root 'target\dm-bridge\Win32\DmBridge.dll') (Join-Path $clientPackageRoot 'dm-bridge\Win32\DmBridge.dll')
    Write-PackagedClientConfig $clientPackageRoot
    Copy-RequiredDirectory (Join-Path $root '实现模块\client-agent\scripts') (Join-Path $clientPackageRoot 'scripts')
    Copy-ToolFiles (Join-Path $clientPackageRoot 'tools') @('common.ps1', 'start-client.ps1')
    Copy-IconAssets $clientPackageRoot @('client.ico', 'lua_ai_client_icon.svg')
    Copy-RequiredFile (Join-Path $root 'README.md') (Join-Path $clientPackageRoot 'README.md')
    Copy-RequiredFile (Join-Path $root 'VERSION') (Join-Path $clientPackageRoot 'VERSION')
}

function Write-PackageReadme {
    param(
        [string]$TargetRoot,
        [ValidateSet('full', 'server', 'client')]
        [string]$PackageKind
    )

    if ($PackageKind -eq 'server') {
        $content = @(
            "# WoW Server $Version",
            '',
            '## Main entry point',
            '- Double-click `management-server.exe` to start the Server tray UI. The tray starts Server and opens Web Admin.',
            '',
            '## Maintenance entry point',
            '- `bin/management-server-core.exe --no-open-browser`',
            '- `bin/management-server-core.exe --tray`',
            '',
            '## Package boundary',
            'This server package does not include Client config, scripts, DmBridge, dm.dll, RegDll.dll, private data, or runtime logs.'
        ) -join [Environment]::NewLine
        Set-Content -LiteralPath (Join-Path $TargetRoot 'RUNNING.md') -Value $content -Encoding UTF8
        return
    }

    if ($PackageKind -eq 'client') {
        $content = @(
            "# WoW Client $Version",
            '',
            '## Main entry point',
            '- Double-click `client-agent.exe` to start Client tray UI.',
            '',
            '## Maintenance entry points',
            '- `bin/client-agent-core.exe --run-once`',
            '- `bin/client-agent-core.exe --monitor`',
            '- `bin/client-agent-core.exe --service-status`',
            '- `bin/client-agent-core.exe --update-apply`',
            '',
            '## Package boundary',
            'This client package does not include Management Server binaries, dm.dll, RegDll.dll, license files, private scripts, account data, or JSONL runtime logs.'
        ) -join [Environment]::NewLine
        Set-Content -LiteralPath (Join-Path $TargetRoot 'RUNNING.md') -Value $content -Encoding UTF8
        return
    }

    $content = @(
        "# WoW Framework $Version",
        '',
        '## Main entry points',
        '- Double-click `management-server.exe` to start the Server tray UI. The tray starts Server and opens Web Admin.',
        '- Double-click `client-agent.exe` to start Client tray UI.',
        '- Double-click `WoW-Manager.exe` to install for current user and create shortcuts.',
        '- Double-click `WoW-Remove.exe` to remove current-user program files and shortcuts.',
        '',
        '## Maintenance entry points',
        '- `bin/management-server-core.exe --no-open-browser`',
        '- `bin/management-server-core.exe --tray`',
        '- `bin/client-agent-core.exe --run-once`',
        '- `bin/client-agent-core.exe --startup-status`',
        '- `bin/client-agent-core.exe --update-apply`',
        '',
        '## Safety boundary',
        'The package does not include dm.dll, RegDll.dll, license files, private scripts, account data, or JSONL runtime logs.'
    ) -join [Environment]::NewLine
    Set-Content -LiteralPath (Join-Path $TargetRoot 'RUNNING.md') -Value $content -Encoding UTF8
}

function Test-PackageSafety {
    param([string]$TargetRoot)

    $blocked = @('dm.dll', 'RegDll.dll', '*.chm', '*.chw', '*.jsonl', '*.pdb', '*.dcu', '*.map', '*.tds', '.env')
    foreach ($pattern in $blocked) {
        $found = Get-ChildItem -LiteralPath $TargetRoot -Recurse -Force -Filter $pattern -ErrorAction SilentlyContinue
        if ($found) {
            throw "Blocked file found in release package: $($found[0].FullName)"
        }
    }
}

function New-ZipPackage {
    param(
        [string]$TargetRoot,
        [string]$TargetZip
    )

    Compress-Archive -Path (Join-Path $TargetRoot '*') -DestinationPath $TargetZip -Force
    (Get-FileHash -Algorithm SHA256 -Path $TargetZip).Hash.ToLowerInvariant()
}

Reset-PackageDirectory
if (-not $SkipBuild) {
    Invoke-Build
}
Copy-PackagePayload
Copy-SplitPackagePayload
Write-PackageReadme $packageRoot 'full'
Write-PackageReadme $serverPackageRoot 'server'
Write-PackageReadme $clientPackageRoot 'client'
Test-PackageSafety $packageRoot
Test-PackageSafety $serverPackageRoot
Test-PackageSafety $clientPackageRoot

$hash = New-ZipPackage $packageRoot $zipPath
$serverHash = New-ZipPackage $serverPackageRoot $serverZipPath
$clientHash = New-ZipPackage $clientPackageRoot $clientZipPath
[pscustomobject]@{
    Version = $Version
    PackageRoot = $packageRoot
    ZipPath = $zipPath
    Sha256 = $hash
    ServerPackageRoot = $serverPackageRoot
    ServerZipPath = $serverZipPath
    ServerSha256 = $serverHash
    ClientPackageRoot = $clientPackageRoot
    ClientZipPath = $clientZipPath
    ClientSha256 = $clientHash
} | ConvertTo-Json -Compress
