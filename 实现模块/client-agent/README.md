# client-agent 说明

## 职责
客户端代理入口，后续负责：
- 启动 Lua Runtime。
- 管理脚本生命周期。
- 与 Server 建立 WebSocket。
- 上报状态、日志和执行结果。

## 当前状态
P3 阶段已完成配置读取、Lua 文件加载、指令上限、状态输出、结构化日志、DmBridge 最小 Lua 高层 API 和 Server 状态上报。

## 当前目录
| 路径 | 职责 |
|------|------|
| `src/main.rs` | 程序入口，只负责串接配置、Lua 宿主和状态输出 |
| `src/config/` | 配置读取、错误类型、默认路径解析 |
| `src/script/` | Lua 脚本文件加载与路径解析 |
| `src/lua_host.rs` | Lua 宿主和白名单 API 注册 |
| `src/lua_dm.rs` | Lua `dm` 高层 API 注册，不暴露 C ABI 指针 |
| `src/dm_bridge/` | Rust `libloading` DmBridge 安全封装 |
| `src/server_reporter.rs` | Management Server HTTP 状态上报入口 |
| `src/server_reporter/error.rs` | 状态上报错误类型 |
| `src/server_reporter/response.rs` | Server HTTP 响应解析 |
| `src/status.rs` | Client Agent 内部状态到共享协议状态的映射 |
| `src/logging.rs` | 本地 tracing 日志初始化 |
| `config/client-agent.toml` | 开发期本地配置样例 |
| `scripts/bootstrap.lua` | 开发期 bootstrap Lua 脚本 |

## 验证命令
```powershell
cargo test --workspace
cargo run -p client-agent
```

## Server 上报烟测
先启动 Server：

```powershell
cargo run -p management-server
```

另一个终端启用上报：

```powershell
$env:CLIENT_AGENT_SERVER_ENABLED='1'
cargo run -p client-agent
```

## DmBridge 烟测
32 位 DmBridge 需要使用 32 位 Rust target：

```powershell
$env:DM_BRIDGE_DLL=(Resolve-Path 'target\dm-bridge\Win32\DmBridge.dll').Path
$env:DM_BRIDGE_COM_SMOKE='1'
cargo test -p client-agent --target i686-pc-windows-msvc dm_bridge_com_ver_and_color_smoke_when_enabled
cargo test -p client-agent --target i686-pc-windows-msvc lua_dm_api_com_ver_and_color_smoke_when_enabled
```
