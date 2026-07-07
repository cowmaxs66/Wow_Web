# Client 原生设置表单化设计

## 阶段定位
P27 解决 Client 本机设置窗口过于工程化的问题。旧窗口直接展示 `client-agent.toml` 文本，不适合正式用户操作；本阶段将其改为结构化表单。

## 改动边界
| 项目 | 决策 |
|------|------|
| 目标版本 | v1.20.0 |
| 改动入口 | `client-agent.exe --settings-window` |
| UI 形式 | WinForms 表单控件 |
| 保存格式 | 继续写回 `config/client-agent.toml` |
| 协议变更 | 无 |
| Server/Web 远程配置 | 不改变 P26 `config.apply` |

## 表单区域
- 基础：Client ID。
- Server 上报：是否启用、Host、Port、Status Path、连接超时。
- Lua 脚本：bootstrap 名称、脚本路径、指令上限。
- 脚本安全门：是否启用、manifest 路径、公钥、权限勾选。
- DM Bridge：DmBridge.dll 路径。

## 保存规则
- 保存前做基础校验。
- 数字字段必须是正整数。
- Server Port 必须在 1-65535。
- Status Path 必须以 `/` 开头。
- 开启安全门时，Ed25519 公钥必须是 64 位十六进制。
- 保存后写回标准 TOML，monitor 下一轮自动读取。

## 保留高级入口
设置窗口保留“打开配置文件”按钮，用于高级排错和手工检查，但普通使用不再要求用户直接编辑 TOML。
