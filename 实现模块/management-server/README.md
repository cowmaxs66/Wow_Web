# management-server 说明

## 职责
管理服务入口，后续负责：
- Client 注册与认证。
- WebSocket Hub。
- 命令下发与结果接收。
- 状态、日志、配置、脚本元数据持久化。

## 当前状态
P3 阶段已完成本地 HTTP 状态上报闭环。

## 当前 API
| 方法 | 路径 | 说明 |
|------|------|------|
| `GET` | `/health` | 健康检查 |
| `POST` | `/api/client/status` | 接收 `WsEnvelope<ClientStatus>` 状态上报 |
| `GET` | `/api/client/status/{client_id}` | 查询指定 Client 最新状态 |

## 当前目录
| 路径 | 职责 |
|------|------|
| `src/main.rs` | Server 启动入口 |
| `src/config.rs` | 监听地址配置 |
| `src/state.rs` | Client 状态内存仓库 |
| `src/error.rs` | API 错误响应 |
| `src/app.rs` | Axum Router 和 handler |

## 验证命令
```powershell
cargo run -p management-server
```

默认监听：

```text
127.0.0.1:18080
```
