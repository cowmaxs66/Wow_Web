# Web 使用体验与 DM/Lua 操作流设计

## 阶段目标
P23 面向 v1.16.0，解决 Web 管理端“操作太复杂、Client 列表不直观、仪表盘简陋、设置说明不清、DM/Lua 使用关系不明确”的问题。

## 影响评估
- 是否影响旧结构：否。
- 是否影响已完成任务：否。
- 是否需要兼容处理：是，保留原有状态 API、消息 API、命令队列和 Client 上报协议。

## 实现原则
- Web 端只做管理与引导，不直接传入任意 Lua 文本。
- Client 只执行本机已配置、已通过安全校验的 `scripts/bootstrap.lua`。
- DM 能力必须通过 `dm.access` 权限显式开启。
- 普通用户优先看到控制中心和按钮流程，PowerShell 命令只作为进阶排错信息。

## 主要改动
- Client 列表增加搜索、筛选、在线摘要、DM 权限、脚本、版本和相对上报时间。
- 快照分析增加健康分、上报开关、安全门、DM 权限、架构分布和风险提示。
- 设置向导改为标准操作流程，隐藏命令行内容到进阶排错区。
- 新增 DM/Lua 使用流程面板，解释 Client 如何读取配置、执行 Lua、套用 DM 权限。
- 新增远程命令 `script.run_bootstrap`，允许 Server 让选中 Client 重新执行本机 bootstrap。

## 安全边界
- 不允许 Server 下发任意 Lua 文本。
- 不允许 Web 端直接修改 Client 本机 TOML、manifest 或真实脚本。
- 不提交大漠二进制、授权文件、真实账号、私有脚本和 JSONL 运行数据。
- `script.run_bootstrap` 只复用 Client 本机已有配置，仍经过 manifest、hash、签名和权限白名单。

## 验证标准
- Rust fmt、clippy、workspace test 通过。
- Web Admin build 通过。
- Web 页面能正常展示 Client 列表、仪表盘、设置向导和 DM/Lua 面板。
- Server 能接受 `script.run_bootstrap` 命令。
- Client monitor 能执行 `script.run_bootstrap` 并写入本机日志摘要。
