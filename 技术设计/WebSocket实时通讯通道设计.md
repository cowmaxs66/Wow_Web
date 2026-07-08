# WebSocket 实时通讯通道设计

## 阶段定位
P40 目标是解决多机器管理时 HTTP 轮询命令下发慢的问题。当前实现不推翻 P30 `/api/client/sync` 合并同步，而是在其旁边新增 WebSocket 实时通道：

- Client 常驻连接：`/ws/client/{client_id}`。
- 兼容连接：`/api/client/ws/{client_id}`。
- Web Admin 事件连接：`/ws/admin`。
- HTTP `/api/client/sync`、消息、命令、回执 API 全部保留。

## 设计原则
- WebSocket 是加速通道，不是唯一通道。
- Server 命令仍先写入原 `commands` 队列。
- Server 若发现 Client 有实时连接，会立即推送 `command` frame。
- Client 使用共享去重表，避免 WS 和 HTTP 同时拿到同一命令时重复执行。
- Client 执行后优先通过 WS 回传 `command_receipt`，Server 收到回执后移除待命令队列中的同一 command_id。
- 如果 WS 断开或回执失败，HTTP 合并同步仍会兜底。

## 协议
协议定义在 `shared-types`：

- `ClientRealtimeMessage`
  - `hello`
  - `command_receipt`
- `ServerRealtimeMessage`
  - `ready`
  - `command`
  - `message`
  - `error`
- `AdminRealtimeMessage`
  - `client_connected`
  - `client_disconnected`
  - `client_status`
  - `command_queued`
  - `command_receipt`
  - `client_message`

Admin 事件 payload 在 Rust 内部使用 `Box` 降低 enum 尺寸，JSON 输出形状不变。

## Server 实现
- `management-server/src/realtime_ws.rs` 负责 WebSocket upgrade、读写任务和 frame 解析。
- `ServerState` 增加实时 Client/Admin sender registry。
- 同一 Client 只保留最新实时连接，旧连接断开时不会误删新连接。
- `save_status` 会广播 Admin `client_status`。
- `push_command` 会广播 Admin `command_queued` 并向在线 Client 推送 `command`。
- `push_command_receipt` 会广播 Admin `command_receipt` 并按 `command_id` 清理待命令。

## Client 实现
- `client-agent/src/client_realtime.rs` 启动独立阻塞线程连接 Server。
- 连接失败后每 3 秒重试。
- 读取超时使用 500ms tick，保证退出时不会卡死。
- 收到 `command` 后复用 `remote_command::execute_remote_command` 和 monitor 的回执生成逻辑。
- 连接建立时发送 `hello`，Server 返回 `ready` 并补发当前待命令。
- monitor 原有 HTTP 同步继续执行，用共享 `SharedSeenCommands` 去重。

## Web Admin 实现
- `web-admin/src/api/realtime.ts` 负责连接 `/ws/admin`、自动重连和解析事件。
- `useDashboardStatus` 收到任意实时事件后做 300ms 防抖刷新。
- 前端不直接手改分页列表，仍走原 HTTP 查询，保留筛选、分页和离线判定。
- 顶部栏新增“实时通道 / HTTP 保底”状态提示。

## 验证
- `cargo test --workspace`：通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `npm --prefix 实现模块/web-admin run build`：通过。
- 本机 WS smoke：启动 `management-server.exe --api-only` 到 `127.0.0.1:18180`，Node WebSocket 连接 `/ws/admin` 后 POST 一条 `smoke-client` 状态，成功收到 `client_status` 事件。

## 当前边界
- WebSocket 当前不做登录鉴权，仍属于内部测试和内网使用边界。
- 命令队列仍是内存队列，Server 重启后未持久化的命令和回执会丢失。
- WebSocket 只优化命令和事件延迟，不替代后续数据库、鉴权、审计操作者身份和外网 TLS。
