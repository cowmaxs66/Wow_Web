# client-agent 说明

## 职责
客户端代理入口，后续负责：
- 启动 Lua Runtime。
- 管理脚本生命周期。
- 与 Server 建立 WebSocket。
- 上报状态、日志和执行结果。

## 当前状态
P1 阶段正在建立配置读取与 Lua 宿主最小闭环。

## 当前目录
| 路径 | 职责 |
|------|------|
| `src/main.rs` | 程序入口，只负责串接配置、Lua 宿主和状态输出 |
| `src/config/` | 配置读取、错误类型、默认路径解析 |
| `src/lua_host.rs` | Lua 宿主和白名单 API 注册 |
| `config/client-agent.toml` | 开发期本地配置样例 |

## 验证命令
```powershell
cargo run -p client-agent
```
