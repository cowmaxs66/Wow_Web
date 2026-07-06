$ErrorActionPreference = 'Stop'

$moduleRoot = $PSScriptRoot
$repoRoot = Resolve-Path (Join-Path $moduleRoot '..\..')
$dcc32 = $env:DELPHI_DCC32

if ([string]::IsNullOrWhiteSpace($dcc32)) {
    $dcc32 = 'C:\RAD13\bin\dcc32.exe'
}

if (-not (Test-Path -LiteralPath $dcc32)) {
    throw "未找到 dcc32.exe，请设置 DELPHI_DCC32 或确认 C:\RAD13\bin\dcc32.exe 存在。"
}

$outputDir = Join-Path $repoRoot 'target\dm-bridge\Win32'
$dcuDir = Join-Path $outputDir 'dcu'
New-Item -ItemType Directory -Force -Path $outputDir, $dcuDir | Out-Null

# 只编译 Win32。当前大漠 dm.dll 按 32 位 COM 插件处理，64 位桥接需后续单独验证。
& $dcc32 -Q -B "-E$outputDir" "-NU$dcuDir" (Join-Path $moduleRoot 'src\DmBridge.dpr')

if ($LASTEXITCODE -ne 0) {
    throw "DmBridge Win32 编译失败，退出码：$LASTEXITCODE"
}

Write-Host "DmBridge Win32 编译完成：$(Join-Path $outputDir 'DmBridge.dll')"
