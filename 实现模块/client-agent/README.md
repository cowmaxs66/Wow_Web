# client-agent 说明

## 职责
客户端代理入口，后续负责：
- 启动 Lua Runtime。
- 管理脚本生命周期。
- 与 Server 建立 WebSocket。
- 上报状态、日志和执行结果。

## 当前状态
P0 阶段只输出标准 JSON 状态消息，用于验证 shared-types 可被客户端复用。

## 验证命令
```powershell
cargo run -p client-agent
```
