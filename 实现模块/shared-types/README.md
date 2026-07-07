# shared-types 说明

## 职责
本模块保存 Client Agent、Management Server 和 Web Admin 共同依赖的协议结构。

## 当前状态
P25 已扩展共享协议与命令目录，当前包含：
- 基础状态：`client_id`、`online`、`current_script`。
- 运行详情：框架版本、操作系统、架构、进程 ID。
- 脚本摘要：bootstrap 名称、Lua 指令上限、脚本安全门和允许权限。
- 上报摘要：是否启用 Server 上报以及上报目标。
- 历史响应：`ClientStatusHistory` 保存指定 Client 的状态样本列表、上限和数量。
- 远程命令：`REMOTE_COMMAND_TYPES` 保存当前 Server/Client 共用的远程命令类型清单。
- 命令回执：`ClientCommandReceipt*` 保存 Client 执行结果摘要。

## 约束
- 字段必须来自真实配置或运行时，不在协议层制造假数据。
- 修改协议后必须同步 Web Admin TypeScript 类型。
- 生产字段扩展前必须考虑向后兼容和 Server 持久化迁移。
