# Client 远程配置下发设计

## 阶段定位
P26 解决“Server 能看到 Client，但不能正式套用 Client 本机设置”的断点。目标是让 Web Admin 可以对指定 Client 下发受控配置，Client 写回本机 `client-agent.toml`，并在 monitor 下一轮自动重载。

## 改动边界
| 项目 | 决策 |
|------|------|
| 目标版本 | v1.19.0 |
| 命令类型 | 新增 `config.apply` |
| 配置范围 | 仅允许 Server 上报、Lua bootstrap、安全门权限、DmBridge 路径 |
| Client ID | 不允许远程修改，避免历史状态和命令回执断裂 |
| Lua 内容 | 不下发任意 Lua 文本 |
| 生效方式 | 写回 TOML 后，monitor 下一轮重载配置 |
| 回执 | 复用 P24 命令回执，返回配置写回结果 |

## 协议
`config.apply` payload 使用 `ClientConfigPatch`：
- `lua.bootstrap_name`
- `lua.bootstrap_path`
- `lua.instruction_limit`
- `script_security.enabled`
- `script_security.manifest_path`
- `script_security.trusted_signer_public_key`
- `script_security.allowed_permissions`
- `dm.bridge_path`
- `server.enabled`
- `server.host`
- `server.port`
- `server.status_path`
- `server.connect_timeout_ms`

## Client 执行流程
1. monitor 轮询 Server 命令队列。
2. 收到 `config.apply` 后解析 payload。
3. 读取本机 `config/client-agent.toml`。
4. 只应用白名单字段。
5. 保存前执行完整配置校验。
6. 写回 TOML。
7. 上报命令回执。
8. 下一轮 monitor 重新读取配置。

## Web Admin
新增 `ClientConfigApplyPanel.vue`，放在设置页面：
- 选择单台 Client。
- 设置 Server 上报地址。
- 设置 Lua bootstrap。
- 设置脚本安全门权限。
- 设置 DmBridge 路径。
- 下发 `config.apply`。

## 安全边界
- `config.apply` payload 不能为空。
- Server 校验 payload 最大 4000 字符。
- Client 只接受已知脚本权限：`host.log`、`config.read`、`dm.access`。
- 远程配置不包含脚本文本、签名私钥、账号资料和大漠授权。
- 生产联网前仍必须补登录鉴权、操作者身份、持久化审计和防重放。
