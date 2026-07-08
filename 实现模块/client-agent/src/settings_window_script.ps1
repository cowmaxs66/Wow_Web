Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing

$configPath = '__CONFIG_PATH__'

function Read-ConfigText {
  if (-not (Test-Path -LiteralPath $configPath)) {
    return ''
  }

  return [System.IO.File]::ReadAllText($configPath, [System.Text.Encoding]::UTF8)
}

function Get-SectionText([string]$text, [string]$sectionName) {
  # 只解析本项目已知的简单 TOML 结构，输入是完整配置文本，输出是指定 section 的正文。
  # 这里不做通用 TOML 解析器，避免在设置窗口里引入额外依赖和过度复杂逻辑。
  $pattern = '(?ms)^\[' + [regex]::Escape($sectionName) + '\]\s*(.*?)(?=^\[|\z)'
  $match = [regex]::Match($text, $pattern)
  if ($match.Success) {
    return $match.Groups[1].Value
  }

  return ''
}

function Get-ScalarValue([string]$sectionText, [string]$key, [string]$defaultValue) {
  $pattern = '(?m)^\s*' + [regex]::Escape($key) + '\s*=\s*(.+?)\s*$'
  $match = [regex]::Match($sectionText, $pattern)
  if (-not $match.Success) {
    return $defaultValue
  }

  $value = $match.Groups[1].Value.Trim()
  if ($value.StartsWith('"') -and $value.EndsWith('"')) {
    return $value.Substring(1, $value.Length - 2)
  }

  return $value
}

function Get-BoolValue([string]$sectionText, [string]$key, [bool]$defaultValue) {
  $value = (Get-ScalarValue $sectionText $key '').Trim().ToLowerInvariant()
  if ($value -eq 'true' -or $value -eq '1') {
    return $true
  }
  if ($value -eq 'false' -or $value -eq '0') {
    return $false
  }

  return $defaultValue
}

function Get-ArrayItems([string]$sectionText, [string]$key) {
  $pattern = '(?m)^\s*' + [regex]::Escape($key) + '\s*=\s*\[(.*?)\]\s*$'
  $match = [regex]::Match($sectionText, $pattern)
  if (-not $match.Success) {
    return @()
  }

  $items = @()
  foreach ($item in [regex]::Matches($match.Groups[1].Value, '"([^"]+)"')) {
    $items += $item.Groups[1].Value
  }
  return $items
}

function Escape-TomlString([string]$value) {
  return $value.Replace('\', '\\').Replace('"', '\"')
}

function Build-PermissionsText {
  $items = @()
  if ($permHostLog.Checked) {
    $items += '"host.log"'
  }
  if ($permConfigRead.Checked) {
    $items += '"config.read"'
  }
  if ($permDmAccess.Checked) {
    $items += '"dm.access"'
  }

  return ($items -join ', ')
}

function Build-TagsText {
  $items = @()
  foreach ($tag in $clientTags.Text.Split(',')) {
    $tag = $tag.Trim()
    if (-not [string]::IsNullOrWhiteSpace($tag)) {
      $items += '"' + (Escape-TomlString $tag) + '"'
    }
  }

  return ($items -join ', ')
}

function Read-IntField([System.Windows.Forms.TextBox]$box, [string]$label, [int]$minValue, [int]$maxValue) {
  $number = 0
  if (-not [int]::TryParse($box.Text.Trim(), [ref]$number)) {
    throw "$label 必须是整数"
  }
  if ($number -lt $minValue -or $number -gt $maxValue) {
    throw "$label 必须在 $minValue 到 $maxValue 之间"
  }

  return $number
}

function Ensure-Required([System.Windows.Forms.TextBox]$box, [string]$label) {
  if ([string]::IsNullOrWhiteSpace($box.Text)) {
    throw "$label 不能为空"
  }
}

function Load-FormFromConfig {
  # 输入是当前 TOML 文件；输出是表单控件状态。
  # 配置缺失时填充安全默认值，用户保存后会生成标准配置文件。
  $text = Read-ConfigText
  $clientSection = Get-SectionText $text 'client'
  $luaSection = Get-SectionText $text 'lua'
  $securitySection = Get-SectionText $text 'script_security'
  $dmSection = Get-SectionText $text 'dm'
  $serverSection = Get-SectionText $text 'server'

  $clientId.Text = Get-ScalarValue $clientSection 'id' 'local-dev-client'
  $clientDisplayName.Text = Get-ScalarValue $clientSection 'display_name' 'Local Dev Client'
  $clientGroupName.Text = Get-ScalarValue $clientSection 'group' 'default'
  $clientTags.Text = ((Get-ArrayItems $clientSection 'tags') -join ', ')
  $luaEnabled.Checked = Get-BoolValue $luaSection 'enabled' $true
  $bootstrapName.Text = Get-ScalarValue $luaSection 'bootstrap_name' 'bootstrap'
  $bootstrapPath.Text = Get-ScalarValue $luaSection 'bootstrap_path' 'scripts/bootstrap.lua'
  $instructionLimit.Text = Get-ScalarValue $luaSection 'instruction_limit' '100000'
  $securityEnabled.Checked = Get-BoolValue $securitySection 'enabled' $false
  $manifestPath.Text = Get-ScalarValue $securitySection 'manifest_path' 'scripts/bootstrap.manifest.json'
  $publicKey.Text = Get-ScalarValue $securitySection 'trusted_signer_public_key' ''
  $dmBridgePath.Text = Get-ScalarValue $dmSection 'bridge_path' 'dm-bridge/Win32/DmBridge.dll'
  $serverEnabled.Checked = Get-BoolValue $serverSection 'enabled' $true
  $serverHost.Text = Get-ScalarValue $serverSection 'host' '127.0.0.1'
  $serverPort.Text = Get-ScalarValue $serverSection 'port' '18080'
  $statusPath.Text = Get-ScalarValue $serverSection 'status_path' '/api/client/status'
  $connectTimeout.Text = Get-ScalarValue $serverSection 'connect_timeout_ms' '3000'

  $permissions = @(Get-ArrayItems $securitySection 'allowed_permissions')
  $permHostLog.Checked = $permissions -contains 'host.log'
  $permConfigRead.Checked = $permissions -contains 'config.read'
  $permDmAccess.Checked = $permissions -contains 'dm.access'
  $status.Text = '已读取配置：' + (Get-Date).ToString('HH:mm:ss')
}

function Validate-Form {
  # 保存前只校验会导致 Client 启动失败或 Server 上报失败的关键字段。
  # 输入是表单控件状态；失败时抛出中文错误，成功时不返回数据。
  Ensure-Required $clientId 'Client ID'
  Ensure-Required $clientDisplayName '显示名称'
  Ensure-Required $clientGroupName '分组'
  Ensure-Required $bootstrapName 'Bootstrap 名称'
  Ensure-Required $bootstrapPath 'Bootstrap 路径'
  Ensure-Required $dmBridgePath 'DmBridge 路径'
  Ensure-Required $serverHost 'Server Host'
  Ensure-Required $statusPath 'Status Path'

  [void](Read-IntField $instructionLimit 'Lua 指令上限' 1 2147483647)
  [void](Read-IntField $serverPort 'Server Port' 1 65535)
  [void](Read-IntField $connectTimeout '连接超时 ms' 1 2147483647)

  if (-not $statusPath.Text.Trim().StartsWith('/')) {
    throw 'Status Path 必须以 / 开头'
  }

  if ($securityEnabled.Checked -and $publicKey.Text.Trim() -notmatch '^[0-9a-fA-F]{64}$') {
    throw '开启安全门时，Ed25519 公钥必须是 64 位十六进制'
  }
}

function Build-ConfigText {
  # 输出是完整 TOML 文本。字段顺序固定，便于后续 diff、回滚和远程 config.apply 对齐。
  $permissionsText = Build-PermissionsText
  $tagsText = Build-TagsText
  $serverEnabledText = $serverEnabled.Checked.ToString().ToLowerInvariant()
  $securityEnabledText = $securityEnabled.Checked.ToString().ToLowerInvariant()
  $luaEnabledText = $luaEnabled.Checked.ToString().ToLowerInvariant()
  $config = @"
[client]
id = "$(Escape-TomlString ($clientId.Text.Trim()))"
display_name = "$(Escape-TomlString ($clientDisplayName.Text.Trim()))"
group = "$(Escape-TomlString ($clientGroupName.Text.Trim()))"
tags = [$tagsText]

[lua]
enabled = $luaEnabledText
bootstrap_name = "$(Escape-TomlString ($bootstrapName.Text.Trim()))"
bootstrap_path = "$(Escape-TomlString ($bootstrapPath.Text.Trim()))"
instruction_limit = $(Read-IntField $instructionLimit 'Lua 指令上限' 1 2147483647)

[script_security]
enabled = $securityEnabledText
manifest_path = "$(Escape-TomlString ($manifestPath.Text.Trim()))"
trusted_signer_public_key = "$(Escape-TomlString ($publicKey.Text.Trim()))"
allowed_permissions = [$permissionsText]

[dm]
bridge_path = "$(Escape-TomlString ($dmBridgePath.Text.Trim()))"

[server]
enabled = $serverEnabledText
host = "$(Escape-TomlString ($serverHost.Text.Trim()))"
port = $(Read-IntField $serverPort 'Server Port' 1 65535)
status_path = "$(Escape-TomlString ($statusPath.Text.Trim()))"
connect_timeout_ms = $(Read-IntField $connectTimeout '连接超时 ms' 1 2147483647)
"@

  return $config
}

function Save-Config {
  try {
    Validate-Form
    $config = Build-ConfigText
    $configDir = [System.IO.Path]::GetDirectoryName($configPath)
    if (-not [string]::IsNullOrWhiteSpace($configDir)) {
      [System.IO.Directory]::CreateDirectory($configDir) | Out-Null
    }
    [System.IO.File]::WriteAllText($configPath, $config, [System.Text.Encoding]::UTF8)
    $status.Text = '已保存：' + (Get-Date).ToString('HH:mm:ss') + '；monitor 下一轮会读取新设置'
  } catch {
    [System.Windows.Forms.MessageBox]::Show($_.Exception.Message, '设置无法保存', 'OK', 'Warning') | Out-Null
    $status.Text = '保存失败：' + $_.Exception.Message
  }
}

function New-Label([string]$text, [int]$left, [int]$top, [int]$width) {
  $label = New-Object System.Windows.Forms.Label
  $label.Text = $text
  $label.Left = $left
  $label.Top = $top
  $label.Width = $width
  $label.Height = 18
  return $label
}

function New-TextBox([int]$left, [int]$top, [int]$width) {
  $box = New-Object System.Windows.Forms.TextBox
  $box.Left = $left
  $box.Top = $top
  $box.Width = $width
  return $box
}

function New-CheckBox([string]$text, [int]$left, [int]$top, [int]$width) {
  $box = New-Object System.Windows.Forms.CheckBox
  $box.Text = $text
  $box.Left = $left
  $box.Top = $top
  $box.Width = $width
  return $box
}

$form = New-Object System.Windows.Forms.Form
$form.Text = 'WoW Client 本机设置'
$form.Width = 820
$form.Height = 810
$form.StartPosition = 'CenterScreen'
$form.FormBorderStyle = 'FixedDialog'
$form.MaximizeBox = $false

$intro = New-Label '填写后保存到 client-agent.toml。monitor / service 会在下一轮刷新时读取新设置。' 16 14 760
$intro.ForeColor = [System.Drawing.Color]::FromArgb(70, 82, 98)
$form.Controls.Add($intro)

$clientGroup = New-Object System.Windows.Forms.GroupBox
$clientGroup.Text = '基础'
$clientGroup.Left = 16
$clientGroup.Top = 42
$clientGroup.Width = 372
$clientGroup.Height = 178
$form.Controls.Add($clientGroup)

$clientGroup.Controls.Add((New-Label 'Client ID' 14 28 90))
$clientId = New-TextBox 110 24 240
$clientGroup.Controls.Add($clientId)
$clientGroup.Controls.Add((New-Label '显示名称' 14 64 90))
$clientDisplayName = New-TextBox 110 60 240
$clientGroup.Controls.Add($clientDisplayName)
$clientGroup.Controls.Add((New-Label '分组' 14 100 90))
$clientGroupName = New-TextBox 110 96 240
$clientGroup.Controls.Add($clientGroupName)
$clientGroup.Controls.Add((New-Label '标签' 14 136 90))
$clientTags = New-TextBox 110 132 240
$clientGroup.Controls.Add($clientTags)

$serverGroup = New-Object System.Windows.Forms.GroupBox
$serverGroup.Text = 'Server 上报'
$serverGroup.Left = 408
$serverGroup.Top = 42
$serverGroup.Width = 376
$serverGroup.Height = 176
$form.Controls.Add($serverGroup)

$serverEnabled = New-CheckBox '启用 Client 上报' 14 26 180
$serverGroup.Controls.Add($serverEnabled)
$serverGroup.Controls.Add((New-Label 'Host' 14 62 90))
$serverHost = New-TextBox 110 58 236
$serverGroup.Controls.Add($serverHost)
$serverGroup.Controls.Add((New-Label 'Port' 14 96 90))
$serverPort = New-TextBox 110 92 96
$serverGroup.Controls.Add($serverPort)
$serverGroup.Controls.Add((New-Label '超时 ms' 220 96 58))
$connectTimeout = New-TextBox 282 92 64
$serverGroup.Controls.Add($connectTimeout)
$serverGroup.Controls.Add((New-Label 'Status Path' 14 130 90))
$statusPath = New-TextBox 110 126 236
$serverGroup.Controls.Add($statusPath)

$luaGroup = New-Object System.Windows.Forms.GroupBox
$luaGroup.Text = 'Lua 脚本'
$luaGroup.Left = 16
$luaGroup.Top = 230
$luaGroup.Width = 372
$luaGroup.Height = 178
$form.Controls.Add($luaGroup)

$luaEnabled = New-CheckBox '启用 Lua' 14 26 100
$luaGroup.Controls.Add($luaEnabled)
$luaGroup.Controls.Add((New-Label 'Bootstrap 名称' 14 62 96))
$bootstrapName = New-TextBox 120 58 230
$luaGroup.Controls.Add($bootstrapName)
$luaGroup.Controls.Add((New-Label 'Bootstrap 路径' 14 98 96))
$bootstrapPath = New-TextBox 120 94 230
$luaGroup.Controls.Add($bootstrapPath)
$luaGroup.Controls.Add((New-Label '指令上限' 14 134 96))
$instructionLimit = New-TextBox 120 130 120
$luaGroup.Controls.Add($instructionLimit)
$luaNote = New-Label '路径相对 Client 包根目录，例如 scripts/bootstrap.lua。' 14 158 330
$luaNote.ForeColor = [System.Drawing.Color]::FromArgb(92, 104, 120)
$luaGroup.Controls.Add($luaNote)

$securityGroup = New-Object System.Windows.Forms.GroupBox
$securityGroup.Text = '脚本安全门'
$securityGroup.Left = 16
$securityGroup.Top = 420
$securityGroup.Width = 768
$securityGroup.Height = 190
$form.Controls.Add($securityGroup)

$securityEnabled = New-CheckBox '启用 manifest / hash / 签名校验' 14 26 260
$securityGroup.Controls.Add($securityEnabled)
$securityGroup.Controls.Add((New-Label 'Manifest 路径' 14 64 104))
$manifestPath = New-TextBox 126 60 612
$securityGroup.Controls.Add($manifestPath)
$securityGroup.Controls.Add((New-Label 'Ed25519 公钥' 14 100 104))
$publicKey = New-TextBox 126 96 612
$securityGroup.Controls.Add($publicKey)
$securityGroup.Controls.Add((New-Label 'Lua 权限' 14 136 104))
$permHostLog = New-CheckBox 'host.log' 126 132 100
$permConfigRead = New-CheckBox 'config.read' 244 132 120
$permDmAccess = New-CheckBox 'dm.access' 386 132 120
$securityGroup.Controls.Add($permHostLog)
$securityGroup.Controls.Add($permConfigRead)
$securityGroup.Controls.Add($permDmAccess)

$dmGroup = New-Object System.Windows.Forms.GroupBox
$dmGroup.Text = 'DM Bridge'
$dmGroup.Left = 16
$dmGroup.Top = 620
$dmGroup.Width = 768
$dmGroup.Height = 74
$form.Controls.Add($dmGroup)

$dmGroup.Controls.Add((New-Label 'DmBridge.dll 路径' 14 32 112))
$dmBridgePath = New-TextBox 136 28 602
$dmGroup.Controls.Add($dmBridgePath)

$status = New-Object System.Windows.Forms.Label
$status.Left = 16
$status.Top = 706
$status.Width = 460
$status.Height = 24
$form.Controls.Add($status)

$save = New-Object System.Windows.Forms.Button
$save.Text = '保存设置'
$save.Left = 486
$save.Top = 702
$save.Width = 92
$save.Add_Click({ Save-Config })
$form.Controls.Add($save)

$reload = New-Object System.Windows.Forms.Button
$reload.Text = '重新读取'
$reload.Left = 590
$reload.Top = 702
$reload.Width = 100
$reload.Add_Click({ Load-FormFromConfig })
$form.Controls.Add($reload)

$openFile = New-Object System.Windows.Forms.Button
$openFile.Text = '打开配置文件'
$openFile.Left = 702
$openFile.Top = 702
$openFile.Width = 82
$openFile.Add_Click({ Start-Process -FilePath 'notepad.exe' -ArgumentList $configPath })
$form.Controls.Add($openFile)

Load-FormFromConfig
[void]$form.ShowDialog()
