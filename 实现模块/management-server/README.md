# management-server 说明

## 职责
管理服务入口，后续负责：
- Client 注册与认证。
- 状态、日志、配置、脚本元数据持久化。
- WebSocket 实时推送、命令下发与结果接收。

## 当前状态
P8 阶段已完成 Web Admin 读取所需的本地 HTTP API，并可保存每个 Client 最近 50 条进程内历史状态。

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
| `src/state.rs` | Client 最新状态和短期历史内存仓库 |
| `src/error.rs` | API 错误响应 |
| `src/app.rs` | Axum Router 和 handler |

## P4 说明
- 当前 CORS 使用开发期宽松配置，方便 `web-admin` 本地 Vite 调试。
- 生产部署前必须结合鉴权、TLS 和来源白名单收紧。

## P8 历史说明
- 当前历史只保存在 Management Server 进程内。
- 每个 Client 最多保留 50 条状态。
- Server 重启后历史清空。
- 生产持久化必须另行设计数据库 schema、索引、清理和备份。

## 验证命令
```powershell
cargo run -p management-server
```

默认监听：

```text
127.0.0.1:18080
```
