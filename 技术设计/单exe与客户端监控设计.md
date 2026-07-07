# 单 exe 与客户端监控设计

## 阶段边界
P11 目标是把 P10 的“脚本一键运行”推进为更接近真实产品的试运行形态：
- Server 一个 exe 可直接提供 API 和 Web Admin。
- Client 一个 exe 可执行一次、常驻监控、打开配置、打开日志。
- Server 可向指定 Client 写入消息。
- Client monitor 轮询 Server 消息，记录本地日志并弹出系统通知。

## 当前实现
| 项目 | 设计 |
|------|------|
| 目标版本 | v1.5.0 |
| Server exe | `management-server.exe` |
| Client exe | `client-agent.exe` |
| Web Admin | release 构建时内嵌到 Server exe |
| Server 消息 API | `POST/GET /api/client/messages/{client_id}` |
| Client 监控模式 | `client-agent.exe --monitor` |
| Client 设置入口 | `client-agent.exe --setup` |
| Client 日志入口 | `client-agent.exe --open-log` |
| Client 通知入口 | `client-agent.exe --notify` 或 monitor 收到消息 |

## Server 单 exe
- `management-server/build.rs` 在编译期读取 `web-admin/dist`。
- 若 dist 存在，编译进 `management-server.exe`。
- 若 dist 不存在，Server 保持 API-only，不影响源码开发。
- `MANAGEMENT_SERVER_WEB_DIR` 仍优先于内嵌资源，方便开发调试。

## Client 监控模式
Client monitor 循环执行：
1. 运行 Lua bootstrap。
2. 生成 Client 状态。
3. 上报 Management Server。
4. 写入 `logs/status-history.jsonl`。
5. 轮询 Server 消息队列。
6. 新消息写入 `logs/client-agent.log`。
7. Windows 下使用系统托盘气泡展示消息。

## 当前不包含
- 持久托盘图标。
- 右键菜单。
- 原生设置窗口。
- WebSocket 长连接。
- 消息送达确认、重试、持久化和权限控制。

这些能力需要 P12 单独设计，避免把 Win32 UI、消息协议和安全边界混在一个阶段里。

