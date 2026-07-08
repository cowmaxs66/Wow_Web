# shared-types 说明

## 职责
本模块保存 Client Agent、Management Server 和 Web Admin 共同依赖的协议结构。

## 当前状态
P40 已扩展共享协议与实时通讯 frame，当前包含：
- 基础状态：`client_id`、`online`、`current_script`。
- 身份信息：`display_name`、`group`、`tags`，用于多机器分组、标签检索和 Web 展示。
- 运行详情：框架版本、操作系统、架构、进程 ID。
- 脚本摘要：bootstrap 名称、Lua 开关、Lua 指令上限、脚本安全门和允许权限。
- 上报摘要：是否启用 Server 上报以及上报目标。
- 历史响应：`ClientStatusHistory` 保存指定 Client 的状态样本列表、上限和数量。
- 分页响应：`ClientStatusPage` 保存 Server 端过滤后的分页元数据和当前页状态。
- 远程命令：`REMOTE_COMMAND_TYPES` 保存当前 Server/Client 共用的远程命令类型清单。
- 脚本热推送：`ClientScriptDeployBundle` 保存 Server 下发到 Client 的 Lua 内容、manifest 内容、权限、启用和立即执行选项。
- 远程配置：`ClientConfigPatch` 保存 `config.apply` 允许下发的 Client 配置补丁。
- 命令回执：`ClientCommandReceipt*` 保存 Client 执行结果摘要。
- 合并同步：`ClientSyncRequest/ClientSyncResponse` 支持 Client 一次 HTTP 完成状态上报、消息拉取和命令拉取。
- Server 审计：`ServerAuditEvent/ServerAuditEventList` 保存消息、命令和回执摘要。
- 实时通讯：`ClientRealtimeMessage`、`ServerRealtimeMessage`、`AdminRealtimeMessage` 支持 Client 命令推送、回执和 Web Admin 事件刷新。

## 约束
- 字段必须来自真实配置或运行时，不在协议层制造假数据。
- 修改协议后必须同步 Web Admin TypeScript 类型。
- 生产字段扩展前必须考虑向后兼容和 Server 持久化迁移。
