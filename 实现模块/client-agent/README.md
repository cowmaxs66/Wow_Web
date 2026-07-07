# client-agent 说明

## 职责
客户端代理入口，后续负责：
- 启动 Lua Runtime。
- 管理脚本生命周期。
- 上报状态、日志和执行结果。
- 后续再接入实时通讯和命令执行。

## 当前状态
P16 阶段已完成配置读取、Lua 文件加载、指令上限、状态输出、结构化日志、DmBridge 最小 Lua 高层 API、Server 状态上报、脚本安全门、运行详情摘要、Web 展示联调、普通编译包路径兼容、monitor/setup/open-log/notify、当前用户开机启动、Windows Service、托盘、设置窗口、更新检查/下载/自替换和远程命令入口。

## 当前目录
| 路径 | 职责 |
|------|------|
| `src/main.rs` | 程序入口，只负责命令分发 |
| `src/agent.rs` | 单次执行 Lua、生成状态、上报 Server |
| `src/cli.rs` | monitor、setup、open-log、notify、startup、service、tray、settings 和 update 参数解析 |
| `src/monitor.rs` | 常驻监控、周期上报、轮询 Server 消息 |
| `src/local_log.rs` | 本地事件日志和状态 JSONL |
| `src/notifier.rs` | Windows 通知气泡 |
| `src/startup.rs` | 当前用户开机启动查询、启用和停用 |
| `src/service_runtime.rs` | Windows Service 运行入口和管理命令 |
| `src/tray.rs` | WinForms 托盘常驻和右键菜单 |
| `src/settings_window.rs` | WinForms 本机设置窗口 |
| `src/updater.rs` | GitHub Release 检查、下载和自替换更新 |
| `src/remote_command.rs` | Server 白名单命令执行分发 |
| `src/config/` | 配置读取、错误类型、默认路径解析 |
| `src/script/` | Lua 脚本文件加载、manifest、签名、hash 和权限校验 |
| `src/lua_host.rs` | Lua 宿主和按权限注册的白名单 API |
| `src/lua_dm.rs` | Lua `dm` 高层 API 注册，不暴露 C ABI 指针 |
| `src/dm_bridge/` | Rust `libloading` DmBridge 安全封装 |
| `src/server_reporter.rs` | Management Server HTTP 状态上报入口 |
| `src/server_reporter/error.rs` | 状态上报错误类型 |
| `src/server_reporter/response.rs` | Server HTTP 响应解析 |
| `src/status.rs` | Client Agent 内部状态到共享协议状态的映射 |
| `src/logging.rs` | 本地 tracing 日志初始化 |
| `config/client-agent.toml` | 开发期本地配置样例 |
| `scripts/bootstrap.lua` | 开发期 bootstrap Lua 脚本 |
| `scripts/bootstrap.manifest.json` | bootstrap manifest、hash、权限和签名 |

## P5 脚本安全
- 默认启用 `script_security`。
- manifest 必须通过 Ed25519 签名验证。
- Lua 文件必须匹配 manifest 中的 SHA-256。
- manifest 请求权限必须包含在本机 `allowed_permissions` 白名单内。
- 未授权的 Lua API 不会注册到 Lua globals。

## P7 状态详情
- `runtime`：上报框架版本、系统、架构和进程 ID。
- `script`：上报 bootstrap 名称、Lua 指令上限、脚本安全门和允许权限。
- `server`：上报状态上报是否启用以及目标地址。
- 状态摘要不包含签名私钥、真实账号、商业脚本和大漠授权资料。

## P9 打包路径
- `config/client-agent.toml` 优先从当前工作目录读取，适配普通编译包。
- `scripts/bootstrap.lua` 和 `scripts/bootstrap.manifest.json` 优先从当前工作目录读取，找不到才回退到源码模块目录。
- 普通编译包不包含 `dm.dll`、`RegDll.dll` 和授权资料；如后续启用大漠能力，需要在本机私有目录配置桥接 DLL。

## P10 x86/x64 运行边界
- `bin/client-agent.exe` 是 x64 Client，用于基础状态、Lua bootstrap、Server 上报和 Web Admin 联调。
- `bin/client-agent-x86.exe` 是 x86 Client，用于后续加载 Win32 DmBridge 与 32 位大漠环境。
- x64 Management Server 可以接收 x86 Client 上报。
- 32 位大漠 `dm.dll` 不能直接放入 x64 Client 进程；必须使用 x86 Client 路径。

## P11 Client 入口
```powershell
client-agent.exe
client-agent.exe --run-once
client-agent.exe --monitor
client-agent.exe --setup
client-agent.exe --open-log
client-agent.exe --notify
client-agent.exe --startup-status
client-agent.exe --enable-startup
client-agent.exe --disable-startup
client-agent.exe --tray
client-agent.exe --settings-window
client-agent.exe --service-status
client-agent.exe --service-install
client-agent.exe --service-start
client-agent.exe --service-stop
client-agent.exe --service-uninstall
client-agent.exe --update-check
client-agent.exe --update-download
client-agent.exe --update-apply
```

- 默认模式启动托盘常驻 UI，并由托盘拉起 monitor。
- `--run-once` 执行一次并输出状态 JSON，供测试和维护使用。
- `--monitor` 常驻运行，周期上报状态、轮询 Server 消息、写入本地日志并弹出通知。
- `--setup` 打开本机配置文件。
- `--open-log` 打开 `logs/client-agent.log`。
- `--notify` 执行一次后弹出状态通知。

## P12 开机启动入口
- `--startup-status` 查询当前用户开机启动状态。
- `--enable-startup` 写入 `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`，启动命令为当前 `client-agent.exe`。
- `--disable-startup` 删除同名当前用户开机启动项。
- 移动发布包目录后需要重新执行 `--enable-startup`。

## P13 正式运行入口
- `--service-run` 是 Windows Service Control Manager 调用入口。
- `--service-install/start/stop/status/uninstall` 管理 `WoWClientAgent` 服务，安装和启停通常需要管理员权限。
- `--tray` 启动托盘常驻 UI，右键菜单可控制 monitor、设置窗口、日志、开机启动、Service 和更新。
- `--settings-window` 打开本机配置编辑窗口。
- `--update-check` 查询 GitHub latest release。
- `--update-download` 下载最新发布包到 `%LOCALAPPDATA%\WoWFramework\updates`。
- `--update-apply` 检查新版、下载发布包，并安排独立 updater 在进程退出后替换安装目录。
- Service 不打开交互窗口；托盘和设置窗口必须运行在当前用户 Session。

## 验证命令
```powershell
cargo test --workspace
cargo clippy --workspace -- -D warnings
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
