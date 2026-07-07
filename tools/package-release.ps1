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
    if (Test-Path -LiteralPath $packageRoot) {
        Assert-InWorkspace $packageRoot
        Remove-Item -LiteralPath $packageRoot -Recurse -Force
    }
    if (Test-Path -LiteralPath $zipPath) {
        Assert-InWorkspace $zipPath
        Remove-Item -LiteralPath $zipPath -Force
    }
    New-Item -ItemType Directory -Force -Path $packageRoot | Out-Null
    New-Item -ItemType Directory -Force -Path (Join-Path $packageRoot 'bin') | Out-Null
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
    Copy-RequiredFile (Join-Path $root '实现模块\client-agent\config\client-agent.toml') (Join-Path $packageRoot 'config\client-agent.toml')
    Copy-RequiredDirectory (Join-Path $root '实现模块\client-agent\scripts') (Join-Path $packageRoot 'scripts')
    Copy-RequiredDirectory (Join-Path $root 'tools') (Join-Path $packageRoot 'tools')
    Copy-RequiredDirectory (Join-Path $root 'tools\installer') (Join-Path $packageRoot 'installer')
    Copy-RequiredFile (Join-Path $root 'README.md') (Join-Path $packageRoot 'README.md')
    Copy-RequiredFile (Join-Path $root 'VERSION') (Join-Path $packageRoot 'VERSION')
}

function Write-PackageReadme {
    $content = @(
        "# WoW Framework $Version",
        '',
        '## Main entry points',
        '- Double-click `management-server.exe` to start Server and open Web Admin.',
        '- Double-click `client-agent.exe` to start Client tray UI.',
        '- Double-click `WoW-Manager.exe` to install for current user and create shortcuts.',
        '- Double-click `WoW-Remove.exe` to remove current-user program files and shortcuts.',
        '',
        '## Maintenance entry points',
        '- `bin/management-server-core.exe --no-open-browser`',
        '- `bin/client-agent-core.exe --run-once`',
        '- `bin/client-agent-core.exe --startup-status`',
        '- `bin/client-agent-core.exe --update-apply`',
        '',
        '## Safety boundary',
        'The package does not include dm.dll, RegDll.dll, license files, private scripts, account data, or JSONL runtime logs.'
    ) -join [Environment]::NewLine
    Set-Content -LiteralPath (Join-Path $packageRoot 'RUNNING.md') -Value $content -Encoding UTF8
}

function Test-PackageSafety {
    $blocked = @('dm.dll', 'RegDll.dll', '*.chm', '*.chw', '*.jsonl', '*.pdb', '*.dcu', '*.map', '*.tds', '.env')
    foreach ($pattern in $blocked) {
        $found = Get-ChildItem -LiteralPath $packageRoot -Recurse -Force -Filter $pattern -ErrorAction SilentlyContinue
        if ($found) {
            throw "Blocked file found in release package: $($found[0].FullName)"
        }
    }
}

Reset-PackageDirectory
if (-not $SkipBuild) {
    Invoke-Build
}
Copy-PackagePayload
Write-PackageReadme
Test-PackageSafety

Compress-Archive -Path (Join-Path $packageRoot '*') -DestinationPath $zipPath -Force
$hash = (Get-FileHash -Algorithm SHA256 -Path $zipPath).Hash.ToLowerInvariant()
[pscustomobject]@{
    Version = $Version
    PackageRoot = $packageRoot
    ZipPath = $zipPath
    Sha256 = $hash
} | ConvertTo-Json -Compress
