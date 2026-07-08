# management-server 说明

## 职责
管理服务入口，后续负责：
- Client 注册与认证。
- 状态、日志、配置、脚本元数据持久化。
- WebSocket 实时推送、命令下发与结果接收。

## 当前状态
P38 阶段已支持 `config.apply` 远程配置命令校验、`/api/client/sync` 合并同步、Client 状态分页过滤、Server 操作审计、Lua 热推送命令校验、Lua 启停状态命令和独立桌面控制台入口。Server 保留 JSONL 历史持久化、Web Admin 内嵌、Client 消息队列、远程命令队列、命令执行回执和可选审计 JSONL 能力。

## 当前 API
| 方法 | 路径 | 说明 |
|------|------|------|
| `GET` | `/health` | 健康检查 |
| `POST` | `/api/client/status` | 接收 `WsEnvelope<ClientStatus>` 状态上报 |
| `POST` | `/api/client/sync` | 接收状态上报，并返回消息列表和待执行命令 |
| `GET` | `/api/client/status` | 查询所有 Client 最新状态 |
| `GET` | `/api/client/status-page` | 分页查询 Client 最新状态，可按分组、标签、在线状态和关键字过滤 |
| `GET` | `/api/client/status/{client_id}` | 查询指定 Client 最新状态 |
| `GET` | `/api/client/history/{client_id}` | 查询指定 Client 最近历史状态 |
| `POST` | `/api/client/messages/{client_id}` | 给指定 Client 写入一条 Server 消息 |
| `GET` | `/api/client/messages/{client_id}` | 查询指定 Client 当前内存消息队列 |
| `POST` | `/api/client/commands/{client_id}` | 给指定 Client 写入一条白名单本机操作命令 |
| `GET` | `/api/client/commands/{client_id}` | 取出并消费指定 Client 当前内存命令队列 |
| `POST` | `/api/client/command-receipts/{client_id}` | 接收指定 Client 的命令执行回执 |
| `GET` | `/api/client/command-receipts/{client_id}` | 查询指定 Client 最近命令执行回执 |
| `GET` | `/api/server/audit` | 查询最近 Server 审计事件 |

## 当前目录
| 路径 | 职责 |
|------|------|
| `src/main.rs` | Server 启动入口 |
| `src/config.rs` | 监听地址、历史文件和审计文件配置 |
| `src/state.rs` | Client 最新状态、短期历史、分页过滤、命令队列和审计事件 |
| `src/persistence.rs` | JSONL 历史和审计持久化读写 |
| `src/error.rs` | API 错误响应 |
| `src/embedded_web.rs` | 编译期内嵌 Web Admin 资源响应 |
| `src/app.rs` | Axum Router、handler、上线日志和 Web Admin 静态文件 fallback |
| `src/app_validation.rs` | API 请求字段校验 |
| `src/app_tests.rs` | app 路由、状态、命令和回执测试 |
| `build.rs` | release 构建时读取 `web-admin/dist` 并生成内嵌资源表 |

## P4 说明
- 当前 CORS 使用开发期宽松配置，方便 `web-admin` 本地 Vite 调试。
- 生产部署前必须结合鉴权、TLS 和来源白名单收紧。

## P9 持久化说明
- 不配置 `MANAGEMENT_SERVER_HISTORY_PATH` 时，保持 P8 的进程内短期历史。
- 配置 `MANAGEMENT_SERVER_HISTORY_PATH` 后，Server 会把状态上报追加写入 JSONL。
- Server 启动时会读取 JSONL，恢复最新状态和每个 Client 最近 50 条历史。
- JSONL 文件可能包含 Client 运行信息，不得提交到 GitHub。

## P10 Web 托管说明
- 不配置 `MANAGEMENT_SERVER_WEB_DIR` 时，只提供 API，行为保持兼容。
- 配置 `MANAGEMENT_SERVER_WEB_DIR` 后，Server 会从该目录提供 Web Admin 静态文件。
- 未匹配到 API 路由时会回退到 `index.html`，保证前端路由刷新时仍可打开页面。
- Web 静态目录只接收编译后的 `dist` 内容，不在 Server 内部执行前端构建。

## P11 单 exe 说明
- 构建前若 `实现模块/web-admin/dist` 存在，`build.rs` 会将其内嵌进 `management-server.exe`。
- 不配置 `MANAGEMENT_SERVER_WEB_DIR` 时，Server 会使用内嵌 Web Admin。
- 配置 `MANAGEMENT_SERVER_WEB_DIR` 时，外部目录优先，方便开发调试。
- `--open-browser` 可在启动后打开浏览器。
- Client 消息队列当前只在内存中保存，Server 重启后丢失。

## P14 双击入口说明
- `management-server.exe` 无参数启动 Server 并打开 Web 管理页。
- `--open-browser` 保留兼容旧脚本。
- `--no-open-browser` 或 `--api-only` 用于自动化、服务化或测试场景，不弹出浏览器。

## P26 远程配置说明
- `POST /api/client/commands/{client_id}` 支持 `config.apply`。
- `config.apply` payload 必须包含至少一个配置项，且 JSON 文本不超过 4000 字符。
- Server 只校验协议形状和命令白名单，真正的本机配置合法性由 Client 写回前校验。

## P29/P30 多机器与合并同步说明
- Client 状态包含 `identity.display_name`、`identity.group` 和 `identity.tags`，用于 Web 分组展示。
- `config.apply` 可改显示名、分组和标签，但不允许改 `client.id`。
- `/api/client/sync` 保存 Client 状态后，返回同一 Client 的消息列表和取出的命令列表。
- 命令在 sync 中仍保持取出即清空语义，避免重复执行。

## P31 分页过滤与审计说明
- `/api/client/status-page` 只查询最新状态快照，不替代 `/api/client/history/{client_id}`。
- 分页大小限制为 1 到 100，避免 Web 在多机器场景一次拉取过多数据。
- `MANAGEMENT_SERVER_AUDIT_PATH` 未配置时只保留进程内最近审计事件。
- `MANAGEMENT_SERVER_AUDIT_PATH` 配置后，消息、命令和命令回执会追加写入 JSONL，Server 启动时会回放最近审计事件。
- 审计事件只保存摘要，不保存完整 payload；JSONL 仍可能包含 Client ID 和命令摘要，不得提交到 GitHub。

## P33 Lua 热推送说明
- `POST /api/client/commands/{client_id}` 支持 `script.deploy_bundle`、`script.start`、`script.stop` 和 `script.status`。
- `script.deploy_bundle` payload 文本上限为 200000 字符，Lua 内容上限为 120000 字符。
- 内部测试模式允许 `security_enabled = false` 且不携带 manifest。
- 如果 `security_enabled = true`，payload 必须同时携带 manifest 路径和 manifest 内容。
- Server 只校验 payload 形状、路径和大小；脚本写入、配置更新和可选执行由 Client 完成并回传命令回执。

## P34/P38 桌面控制台说明
- P34 曾使用 Edge App 模式作为桌面控制台过渡方案。
- P38 起，Server 托盘默认启动发布包根目录的 `WoW-Desktop.exe` 独立桌面控制台，不再调用 `msedge.exe --app`。
- `WoW-Desktop.exe` 使用 Windows WebView2 Runtime 渲染内嵌 Web Admin，这是 Tauri/WRY 类桌面壳的常规方式；它不是 Edge 浏览器窗口。
- 托盘菜单保留“浏览器打开 Web 管理页”作为排错备用入口；只有缺少 `WoW-Desktop.exe` 时，默认桌面入口才会回退到浏览器。

## 验证命令
```powershell
cargo run -p management-server
```

默认监听：

```text
127.0.0.1:18080
```

启用 JSONL 历史持久化：

```powershell
$env:MANAGEMENT_SERVER_HISTORY_PATH='data/status-history.jsonl'
cargo run -p management-server
```

启用 Server 审计 JSONL：

```powershell
$env:MANAGEMENT_SERVER_AUDIT_PATH='data/server-audit.jsonl'
cargo run -p management-server
```

挂载 Web Admin 静态目录：

```powershell
$env:MANAGEMENT_SERVER_WEB_DIR='实现模块/web-admin/dist'
cargo run -p management-server
```
