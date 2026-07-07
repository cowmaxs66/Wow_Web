param(
    [switch]$ShowMessage
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing

$packageRoot = Split-Path -Parent $PSScriptRoot
$installRoot = Join-Path $env:LOCALAPPDATA 'WoWFramework'
$serverUrl = 'http://127.0.0.1:18080'
$iconPath = Join-Path $packageRoot 'assets\icons\server.ico'

function Resolve-RunRoot {
    if (Test-Path -LiteralPath (Join-Path $installRoot 'management-server.exe')) {
        return $installRoot
    }
    return $packageRoot
}

function Show-Notice {
    param(
        [string]$Title,
        [string]$Message,
        [System.Windows.Forms.MessageBoxIcon]$Icon = [System.Windows.Forms.MessageBoxIcon]::Information
    )

    [System.Windows.Forms.MessageBox]::Show($Message, $Title, 'OK', $Icon) | Out-Null
}

function Invoke-InstallerScript {
    param([string]$ScriptName)

    $script = Join-Path $packageRoot "installer\$ScriptName"
    if (-not (Test-Path -LiteralPath $script)) {
        throw "找不到脚本：$script"
    }

    # 控制中心只负责调用经过打包的安装/卸载脚本。
    # 输入：脚本名。
    # 输出：等待脚本执行完成，并让脚本自己显示结果。
    # 边界：不拼接用户输入，不执行外部下载内容。
    $arguments = @('-STA', '-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', $script, '-ShowMessage')
    $process = Start-Process -FilePath 'powershell.exe' -ArgumentList $arguments -Wait -PassThru -WindowStyle Hidden
    if ($process.ExitCode -ne 0) {
        throw "脚本执行失败，退出码：$($process.ExitCode)"
    }
}

function Start-WowExe {
    param([string]$ExeName)

    $root = Resolve-RunRoot
    $exe = Join-Path $root $ExeName
    if (-not (Test-Path -LiteralPath $exe)) {
        throw "找不到程序：$exe"
    }
    Start-Process -FilePath $exe -WorkingDirectory $root | Out-Null
}

function Open-Folder {
    param([string]$Path)

    if (-not (Test-Path -LiteralPath $Path)) {
        New-Item -ItemType Directory -Force -Path $Path | Out-Null
    }
    Start-Process -FilePath $Path | Out-Null
}

function Get-ProcessCountByName {
    param([string]$Name)

    @(Get-Process -Name $Name -ErrorAction SilentlyContinue).Count
}

function Refresh-Status {
    $runRoot = Resolve-RunRoot
    $installed = Test-Path -LiteralPath (Join-Path $installRoot 'management-server.exe')
    $serverCount = Get-ProcessCountByName 'management-server-core'
    $clientCount = Get-ProcessCountByName 'client-agent-core'

    $installState.Text = if ($installed) { '已安装到：' + $installRoot } else { '未安装，当前使用解压目录运行' }
    $serverState.Text = if ($serverCount -gt 0) { "Server 运行中：$serverCount 个进程" } else { 'Server 未运行' }
    $clientState.Text = if ($clientCount -gt 0) { "Client 运行中：$clientCount 个进程" } else { 'Client 未运行' }
    $runRootState.Text = '当前运行目录：' + $runRoot
}

$form = New-Object System.Windows.Forms.Form
$form.Text = 'WoW Framework 控制中心'
$form.Width = 620
$form.Height = 440
$form.StartPosition = 'CenterScreen'
$form.FormBorderStyle = 'FixedDialog'
$form.MaximizeBox = $false
if (Test-Path -LiteralPath $iconPath) {
    $form.Icon = New-Object System.Drawing.Icon($iconPath)
}

$title = New-Object System.Windows.Forms.Label
$title.Text = 'WoW Framework'
$title.Font = New-Object System.Drawing.Font('Microsoft YaHei UI', 18, [System.Drawing.FontStyle]::Bold)
$title.Left = 24
$title.Top = 20
$title.Width = 560
$title.Height = 36
$form.Controls.Add($title)

$subtitle = New-Object System.Windows.Forms.Label
$subtitle.Text = '本机安装、启动、日志和卸载入口'
$subtitle.Left = 26
$subtitle.Top = 60
$subtitle.Width = 560
$subtitle.Height = 24
$form.Controls.Add($subtitle)

$installState = New-Object System.Windows.Forms.Label
$installState.Left = 28
$installState.Top = 100
$installState.Width = 540
$installState.Height = 22
$form.Controls.Add($installState)

$serverState = New-Object System.Windows.Forms.Label
$serverState.Left = 28
$serverState.Top = 126
$serverState.Width = 540
$serverState.Height = 22
$form.Controls.Add($serverState)

$clientState = New-Object System.Windows.Forms.Label
$clientState.Left = 28
$clientState.Top = 152
$clientState.Width = 540
$clientState.Height = 22
$form.Controls.Add($clientState)

$runRootState = New-Object System.Windows.Forms.Label
$runRootState.Left = 28
$runRootState.Top = 178
$runRootState.Width = 540
$runRootState.Height = 22
$form.Controls.Add($runRootState)

function New-Button {
    param(
        [string]$Text,
        [int]$Left,
        [int]$Top,
        [scriptblock]$Action
    )

    $button = New-Object System.Windows.Forms.Button
    $button.Text = $Text
    $button.Left = $Left
    $button.Top = $Top
    $button.Width = 170
    $button.Height = 38
    $button.Add_Click({
        try {
            & $Action
            Refresh-Status
        } catch {
            Show-Notice '操作失败' $_.Exception.Message ([System.Windows.Forms.MessageBoxIcon]::Error)
        }
    })
    $form.Controls.Add($button)
}

New-Button '安装 / 修复' 28 225 { Invoke-InstallerScript 'install-current-user.ps1' }
New-Button '启动 Server' 220 225 { Start-WowExe 'management-server.exe' }
New-Button '启动 Client' 412 225 { Start-WowExe 'client-agent.exe' }
New-Button '打开 Web 管理页' 28 275 { Start-Process -FilePath $serverUrl | Out-Null }
New-Button '打开日志目录' 220 275 { Open-Folder (Join-Path (Resolve-RunRoot) 'logs') }
New-Button '打开程序目录' 412 275 { Open-Folder (Resolve-RunRoot) }
New-Button '刷新状态' 28 325 { Refresh-Status }
New-Button '卸载程序' 220 325 { Invoke-InstallerScript 'uninstall-current-user.ps1' }
New-Button '关闭窗口' 412 325 { $form.Close() }

Refresh-Status
[void]$form.ShowDialog()
