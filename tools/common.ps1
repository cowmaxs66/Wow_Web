Set-StrictMode -Version Latest

function Get-WowRoot {
    Split-Path -Parent $PSScriptRoot
}

function Test-SourceTree {
    param([string]$Root)
    Test-Path -LiteralPath (Join-Path $Root 'Cargo.toml')
}

function Resolve-ServerExe {
    param([string]$Root)

    if (Test-SourceTree $Root) {
        Push-Location $Root
        try {
            # 源码目录下优先重新构建，避免一键运行误用旧 target/debug 产物。
            cargo build -p management-server
        } finally {
            Pop-Location
        }
        return Join-Path $Root 'target\debug\management-server.exe'
    }

    $candidates = @(
        (Join-Path $Root 'bin\management-server.exe'),
        (Join-Path $Root 'target\debug\management-server.exe'),
        (Join-Path $Root 'target\release\management-server.exe')
    )

    foreach ($candidate in $candidates) {
        if (Test-Path -LiteralPath $candidate) {
            return $candidate
        }
    }

    throw '找不到 management-server.exe。请确认发布包完整，或先在源码目录执行 cargo build -p management-server。'
}

function Resolve-ClientExe {
    param(
        [string]$Root,
        [ValidateSet('x64', 'x86')]
        [string]$Arch
    )

    $packageName = if ($Arch -eq 'x86') { 'client-agent-x86.exe' } else { 'client-agent.exe' }
    $targetPath = if ($Arch -eq 'x86') {
        Join-Path $Root 'target\i686-pc-windows-msvc\debug\client-agent.exe'
    } else {
        Join-Path $Root 'target\debug\client-agent.exe'
    }
    $releasePath = if ($Arch -eq 'x86') {
        Join-Path $Root 'target\i686-pc-windows-msvc\release\client-agent.exe'
    } else {
        Join-Path $Root 'target\release\client-agent.exe'
    }

    if (Test-SourceTree $Root) {
        Push-Location $Root
        try {
            # x86 Client 需要单独目标；源码目录下强制构建，保证版本号与当前 VERSION 对齐。
            if ($Arch -eq 'x86') {
                rustup target add i686-pc-windows-msvc
                cargo build -p client-agent --target i686-pc-windows-msvc
            } else {
                cargo build -p client-agent
            }
        } finally {
            Pop-Location
        }
        return $targetPath
    }

    $candidates = @(
        (Join-Path $Root "bin\$packageName"),
        $targetPath,
        $releasePath
    )

    foreach ($candidate in $candidates) {
        if (Test-Path -LiteralPath $candidate) {
            return $candidate
        }
    }

    throw "找不到 $packageName。请确认发布包完整，或先在源码目录执行 Client 构建。"
}

function Resolve-WebDir {
    param([string]$Root)

    $candidates = @(
        (Join-Path $Root 'web-admin\dist'),
        (Join-Path $Root '实现模块\web-admin\dist')
    )

    foreach ($candidate in $candidates) {
        if (Test-Path -LiteralPath (Join-Path $candidate 'index.html')) {
            return $candidate
        }
    }

    return ''
}

function Wait-ServerHealth {
    param(
        [string]$BaseUrl,
        [int]$TimeoutSeconds = 20
    )

    $deadline = (Get-Date).AddSeconds($TimeoutSeconds)
    while ((Get-Date) -lt $deadline) {
        try {
            $health = Invoke-RestMethod -Uri "$BaseUrl/health" -TimeoutSec 2
            if ($health.status -eq 'ok') {
                return
            }
        } catch {
            Start-Sleep -Milliseconds 300
        }
    }

    throw "Server 健康检查超时：$BaseUrl"
}
