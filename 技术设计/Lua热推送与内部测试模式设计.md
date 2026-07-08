# Lua 热推送与内部测试模式设计

## 阶段目标
P33 解决 v1.24.0 实机测试暴露的两个直接问题：

- 用户在 Client 包内修改 `scripts/bootstrap.lua` 后，manifest 中的 SHA-256 仍是旧值，Client 按安全校验拒绝执行。
- 管理端只能让 Client 重跑本机脚本，不能直接把服务端编辑的 Lua 推送到选中机器，也不能远程启动、停止或查询 Lua 状态。

本阶段目标版本为 `v1.25.0`，默认面向内部测试：先让脚本推送和 DM 测试跑通，再由后台设置决定是否重新开启 manifest 校验。

## 影响评估
| 项目 | 结论 |
|------|------|
| 是否影响旧结构 | 是，扩展远程命令协议和 Client 配置字段 |
| 是否推翻旧结构 | 否，沿用 Server 命令队列、Client monitor 轮询和 Web 多选目标 |
| 是否需要兼容处理 | 是，旧 `script.run_bootstrap` 保留，新命令增量接入 |
| 是否影响发布包 | 是，Client 默认配置改为内部测试模式 |

## 命令模型
| 命令 | 作用 | 执行位置 |
|------|------|----------|
| `script.deploy_bundle` | 把 Web Admin 输入的 Lua 内容写入 Client `scripts/` 目录，可选择立即启用和执行 | Client |
| `script.start` | 将 `lua.enabled` 写为 `true`，并立即执行一次当前 bootstrap | Client |
| `script.stop` | 将 `lua.enabled` 写为 `false`，Client monitor 保持在线但下一轮不再执行 Lua | Client |
| `script.status` | 返回当前 Lua 开关、脚本路径、安全门和权限摘要 | Client |
| `script.run_bootstrap` | 保留旧能力，重新执行本机已配置的 bootstrap | Client |

## Client 配置策略
P33 新增 `lua.enabled`：

```toml
[lua]
enabled = true
bootstrap_name = "bootstrap"
bootstrap_path = "scripts/bootstrap.lua"
```

内部测试默认值：

- `lua.enabled = true`
- `script_security.enabled = false`
- `script_security.allowed_permissions = ["host.log", "config.read", "dm.access"]`

这样用户直接改 Lua 或从 Web 热推送 Lua 后，不需要同步维护 manifest hash 和签名。后续需要重新收紧时，管理端可以通过配置面板把 `script_security.enabled` 打开，并推送带 manifest 的脚本包。

## 热推送数据流
```mermaid
flowchart LR
    A["Web Admin 选择 Client"] --> B["填写 Lua 内容"]
    B --> C["POST /api/client/commands/{client_id}"]
    C --> D["Server 命令队列"]
    D --> E["Client monitor /api/client/sync"]
    E --> F["写入 scripts/bootstrap.lua"]
    F --> G["更新 config/client-agent.toml"]
    G --> H["可选立即执行 Lua"]
    H --> I["Client 回传命令回执"]
```

## 防重复下发
状态上报存在 monitor 周期延迟，同一操作可能被用户连续点击。P33 在 Web Admin 增加两层前端拦截：

- 普通命令：同一批 Client、同一命令、同一 payload 在 15 秒内重复发送会被拦截。
- 脚本推送：同一批 Client、同一 Lua 内容、同一配置在 15 秒内重复推送会被拦截。

Server 仍按原命令队列保存请求，不改变旧 API 语义。后续如要做跨浏览器防重，需要在 Server 持久化 `dedupe_key`。

## 路径边界
热推送只允许写入 Client 包内 `scripts/` 目录，并拒绝绝对路径和 `..` 路径穿越。

这不是生产安全门，而是防止误把脚本写到 Client 安装目录外，破坏配置、日志或系统文件。内部测试仍默认放开 Lua 权限和 manifest 校验。

## Lua 停止边界
`script.stop` 是软停止：

- 会把 `lua.enabled` 写成 `false`。
- Client monitor 继续在线、继续拉取 Server 命令、继续上报状态。
- 已经进入一次 `run_once` 的 Lua 不做线程级强杀；当前 Lua 宿主仍依赖指令上限和脚本返回来结束。

如果后续需要真正中断长时间运行脚本，需要把 Lua 运行从单次执行升级为可取消任务模型。

## 验证标准
- `cargo fmt --all --check` 通过。
- `cargo test --workspace` 通过，覆盖新增命令白名单、Server payload 校验和 Client 热推送写入。
- `cargo clippy --workspace -- -D warnings` 通过。
- `npm run build` 通过。
- 打包后 Client 分包可直接运行，默认 `script_security.enabled = false`，并可从 Web Admin 对勾选机器推送 Lua。
