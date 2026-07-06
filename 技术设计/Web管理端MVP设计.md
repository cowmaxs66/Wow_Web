# Web 管理端 MVP 设计

## 目标
P4 建立最小 Web 管理端闭环：

```text
Browser Web Admin -> Management Server HTTP API -> Client 最新状态
```

## 本阶段做什么
| 能力 | 说明 |
|------|------|
| Server 健康检查 | 调用 `GET /health`，展示在线/离线 |
| Client 状态列表 | 调用 `GET /api/client/status`，展示所有已上报 Client |
| Client 状态详情 | 支持按 `client_id` 查询 `GET /api/client/status/{client_id}` |
| 手动刷新 | 用户可修改 Server 地址和 Client ID 后刷新 |
| 本地联调 | Server、Client、Web Admin 三端本机联调 |

## 本阶段不做什么
- 不做登录、鉴权和权限模型。
- 不做脚本上传、脚本下发和远程命令。
- 不做 WebSocket 实时推送。
- 不做数据库持久化。
- 不展示真实账号、窗口标题、业务脚本内容或敏感资源。

## API 设计
| 方法 | 路径 | 输入 | 输出 |
|------|------|------|------|
| `GET` | `/health` | 无 | `HealthResponse` |
| `GET` | `/api/client/status` | 无 | `WsEnvelope<ClientStatus>[]` |
| `GET` | `/api/client/status/{client_id}` | URL client_id | `WsEnvelope<ClientStatus>` |

## 前端交互边界
- 默认 Server 地址为 `http://127.0.0.1:18080`。
- 默认 Client ID 为 `local-dev-client`。
- 刷新时先请求健康检查，再请求 Client 状态列表。
- 如果列表为空，再按输入的 Client ID 查询一次，便于 P3 单 Client 烟测。
- 所有失败必须显示明确错误，不伪造在线状态。

## 目录排版
| 模块 | 文件 | 职责 |
|------|------|------|
| `web-admin` | `src/App.vue` | 页面状态组合和刷新流程 |
| `web-admin` | `src/api/managementServer.ts` | Management Server API 客户端 |
| `web-admin` | `src/types/protocol.ts` | 与 Rust shared-types 对齐的前端类型 |
| `web-admin` | `src/components/` | App Shell、指标、列表、详情、连接表单 |
| `web-admin` | `src/styles/` | 设计 tokens 和全局样式 |
| `management-server` | `src/app.rs` | 状态列表 API 和 CORS |

## 验收标准
- `cargo test --workspace` 通过。
- `cargo clippy --workspace -- -D warnings` 通过。
- `npm run build` 在 `实现模块/web-admin` 通过。
- 启动 Server 与 Client 后，Web 页面可展示 `local-dev-client` 状态。
- 浏览器桌面和移动视口无文字重叠、无横向溢出。

## 当前实现状态
- 已完成 Vue/Vite Web Admin MVP。
- 已完成 Management Server 状态列表 API。
- 已完成 Playwright fallback 浏览器验证。
- 已记录开发期 CORS 风险，生产前必须收紧。
