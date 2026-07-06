# MVP 收敛设计

## 背景
原始方案包含 Client Agent、Lua Runtime、大漠桥接、Management Server、Web Admin、脚本管理和热更新。该终局方向成立，但不适合作为第一步全部实现。

## MVP 目标
第一版只验证最小闭环：

```text
Rust workspace -> shared-types -> client-agent 生成状态消息 -> management-server 复用协议类型
```

## 明确不做
- 不接入真实大漠 DLL。
- 不做 Lua 热重载。
- 不做 Web 前端。
- 不做远端脚本下发。
- 不做 Windows Service。

## 模块边界
| 模块 | 当前职责 | 后续扩展 |
|------|----------|----------|
| `shared-types` | 定义 Client/Server 共用消息结构 | 增加命令、ACK、错误码 |
| `client-agent` | 输出标准状态消息 | 接入 Lua 宿主、WebSocket |
| `management-server` | 复用协议类型，先作为占位可编译服务入口 | 接入 Axum、DB、WebSocket Hub |
| `dm-bridge` | 只保留说明目录 | P2 再实现 Delphi DLL |
| `web-admin` | 只保留说明目录 | P4 再初始化 Vue 项目 |

## 当前验收标准
- `cargo test --workspace` 通过。
- `cargo run -p client-agent` 输出标准 JSON 状态消息。
- 每个目录有中文说明文档。
- 风险、变更、进度文档已更新。
