# Server 通讯闭环设计

## 目标
P3 阶段建立最小可验证通讯链路：

```text
Client Agent -> HTTP POST 状态消息 -> Management Server -> 内存状态仓库 -> HTTP GET 查询状态
```

## 本阶段做什么
| 能力 | 说明 |
|------|------|
| 健康检查 | `GET /health` 返回服务可用状态 |
| 状态上报 | `POST /api/client/status` 接收 `WsEnvelope<ClientStatus>` |
| 状态查询 | `GET /api/client/status/{client_id}` 返回最新状态消息 |
| Client 可选上报 | 默认不连接 Server，通过配置或环境变量启用 |
| 本地烟测 | 启动 Server 后运行 Client，确认状态可被查询 |

## 本阶段不做什么
- 不做账号登录。
- 不做鉴权 token。
- 不做 WebSocket 命令下发。
- 不做数据库持久化。
- 不做 Web 管理端。
- 不传输真实脚本、账号、窗口标题或商业资源。

## 协议边界
P3 继续复用已有共享协议：

```text
WsEnvelope<ClientStatus>
```

新增最小响应：

```text
StatusAck
```

用于说明 Server 已接收某个 `message_id`。

## API 设计
| 方法 | 路径 | 输入 | 输出 |
|------|------|------|------|
| `GET` | `/health` | 无 | `HealthResponse` |
| `POST` | `/api/client/status` | `WsEnvelope<ClientStatus>` | `StatusAck` |
| `GET` | `/api/client/status/{client_id}` | URL client_id | `WsEnvelope<ClientStatus>` |

## 校验规则
- `envelope.client_id` 必须等于 `envelope.data.client_id`。
- `message_type` 必须是 `status`。
- `schema_version` 当前必须是 `1`。
- `client_id` 不能为空。
- Server 只保存每个 Client 的最新状态。

## Client 上报规则
- 默认不上报，保证 `cargo run -p client-agent` 在无 Server 时仍可运行。
- 开发期通过配置或环境变量启用上报。
- 上报失败必须返回明确错误，不能假装成功。
- 仅支持本阶段的本机 HTTP，不支持 HTTPS、代理和公网地址。

## 目录排版
| 模块 | 文件 | 职责 |
|------|------|------|
| `shared-types` | `src/lib.rs` | 共用消息和响应类型 |
| `management-server` | `src/main.rs` | Server 启动入口 |
| `management-server` | `src/config.rs` | 绑定地址配置 |
| `management-server` | `src/state.rs` | Client 状态内存仓库 |
| `management-server` | `src/error.rs` | API 错误响应 |
| `management-server` | `src/app.rs` | Router 与 handler |
| `client-agent` | `src/server_reporter.rs` | HTTP 状态上报入口 |
| `client-agent` | `src/server_reporter/error.rs` | 上报错误类型 |
| `client-agent` | `src/server_reporter/response.rs` | HTTP 响应解析 |

## 验收标准
- `cargo test --workspace` 通过。
- `cargo run -p management-server` 可启动 HTTP 服务。
- `cargo run -p client-agent` 默认仍可输出状态 JSON。
- 启用上报后，Client 状态可被 Server 查询到。
- P3 不提交临时日志、Server 输出、真实业务数据。
