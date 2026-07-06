# management-server 说明

## 职责
管理服务入口，后续负责：
- Client 注册与认证。
- WebSocket Hub。
- 命令下发与结果接收。
- 状态、日志、配置、脚本元数据持久化。

## 当前状态
P0 阶段只复用 shared-types，验证服务端和客户端共享同一份消息契约。

## 验证命令
```powershell
cargo run -p management-server
```
