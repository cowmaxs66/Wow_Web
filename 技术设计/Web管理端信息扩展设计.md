# Web 管理端信息扩展设计

## 目标
P7 扩展 Web 管理端的信息密度，让页面能回答三个问题：
- 当前有多少 Client 在线，脚本分布如何。
- 选中 Client 的真实运行详情是什么。
- Web Admin 当前连接的是哪个 Server 与 Client 查询目标。

## 数据来源
当前只允许使用真实状态数据：
- `GET /health`
- `GET /api/client/status`
- `GET /api/client/status/{client_id}`

不允许在前端硬造 CPU、内存、账号、窗口、地图、角色、任务、历史趋势等后端尚未上报的数据。

## 状态协议扩展
在 `shared-types::ClientStatus` 中增加三个摘要对象：

| 字段 | 说明 | 来源 |
|------|------|------|
| `runtime.release_version` | 当前框架版本 | 根目录 `VERSION` |
| `runtime.os` | Agent 运行系统 | Rust `std::env::consts::OS` |
| `runtime.arch` | Agent 运行架构 | Rust `std::env::consts::ARCH` |
| `runtime.process_id` | Agent 进程 ID | Rust `std::process::id()` |
| `script.bootstrap_name` | bootstrap 名称 | `client-agent.toml` |
| `script.instruction_limit` | Lua 指令上限 | `client-agent.toml` |
| `script.security_enabled` | 脚本安全门是否开启 | `client-agent.toml` |
| `script.allowed_permissions` | 本机允许的脚本权限 | `client-agent.toml` |
| `server.report_enabled` | 是否启用状态上报 | `client-agent.toml` 和环境变量覆盖后结果 |
| `server.report_target` | 上报目标 | `server.host:server.port + status_path` |

## Web 展示边界
| 区域 | P7 实现 | 不做 |
|------|---------|------|
| 快照分析 | 在线比例、脚本分布、最新上报、脚本安全门统计 | 历史趋势、数据库统计 |
| 客户端详情 | runtime、script、server、协议 JSON | CPU/内存/窗口/账号等未上报字段 |
| 设置 | Web Admin 本地 Server URL、查询 Client ID、刷新 | 远程修改 Agent 配置 |

## 模块拆分
| 模块 | 职责 |
|------|------|
| `shared-types` | 定义跨 Client/Server/Web 的状态协议 |
| `client-agent/src/status.rs` | 把本机配置和脚本运行结果映射成状态协议 |
| `web-admin/src/types/protocol.ts` | 前端协议类型与时间格式化工具 |
| `web-admin/src/composables/useDashboardStatus.ts` | 管理刷新、选中项和衍生指标 |
| `web-admin/src/components/SnapshotAnalytics.vue` | 展示当前状态快照分析 |
| `web-admin/src/components/ClientDetail.vue` | 展示选中 Client 的详细状态 |
| `web-admin/src/components/ClientSettingsPanel.vue` | 管理 Web Admin 本地连接设置 |

## 验证标准
- `cargo test --workspace` 通过。
- `cargo run -p client-agent` 输出包含 `runtime`、`script`、`server`。
- Server/Client 上报烟测后，Web Admin 能显示新字段。
- `npm run build` 通过。
- 浏览器桌面和移动视口无横向溢出，分析、详情、设置区域均可见。
