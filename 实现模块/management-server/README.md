# management-server 说明

## 职责
管理服务入口，后续负责：
- Client 注册与认证。
- 状态、日志、配置、脚本元数据持久化。
- WebSocket 实时推送、命令下发与结果接收。

## 当前状态
P10 阶段已完成 Web Admin 静态文件托管入口。Server 仍保留 P9 的 JSONL 历史持久化能力，并可通过环境变量挂载 Web Admin 编译产物，支持一键运行后直接访问管理页面。

## 当前 API
| 方法 | 路径 | 说明 |
|------|------|------|
| `GET` | `/health` | 健康检查 |
| `POST` | `/api/client/status` | 接收 `WsEnvelope<ClientStatus>` 状态上报 |
| `GET` | `/api/client/status` | 查询所有 Client 最新状态 |
| `GET` | `/api/client/status/{client_id}` | 查询指定 Client 最新状态 |
| `GET` | `/api/client/history/{client_id}` | 查询指定 Client 最近历史状态 |

## 当前目录
| 路径 | 职责 |
|------|------|
| `src/main.rs` | Server 启动入口 |
| `src/config.rs` | 监听地址配置 |
| `src/state.rs` | Client 最新状态、短期历史和启动回放 |
| `src/persistence.rs` | JSONL 历史持久化读写 |
| `src/error.rs` | API 错误响应 |
| `src/app.rs` | Axum Router、handler 和 Web Admin 静态文件 fallback |

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

挂载 Web Admin 静态目录：

```powershell
$env:MANAGEMENT_SERVER_WEB_DIR='实现模块/web-admin/dist'
cargo run -p management-server
```
