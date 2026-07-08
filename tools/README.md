# 工具脚本说明

本目录保存一键运行、维护和打包脚本。P15 起普通用户优先直接双击根目录 GUI 入口，核心维护 exe 放入发布包 `bin` 目录；P18 起打包脚本同时输出总包、Server 分包和 Client 分包；P21 起 Server 和 Client 都提供托盘入口；P22 起 `WoW-Manager.exe` 打开本机控制中心。

## 脚本清单
| 文件 | 职责 |
|------|------|
| `common.ps1` | 公共路径解析、构建检测和健康检查函数 |
| `start-server.ps1` | 启动 Management Server，可启动 Server 托盘或维护模式 |
| `start-client.ps1` | 运行 Client Agent，可执行 monitor、tray、settings、service、update 和开机启动设置 |
| `start-server.cmd` | Windows 双击入口，调用 `start-server.ps1` |
| `start-client.cmd` | Windows 双击入口，调用 `start-client.ps1` |
| `run-local.cmd` | Windows 双击入口，启动 Server 后启动 Client 托盘 |
| `package-release.ps1` | 生成正式发布包，输出总包、Server 分包和 Client 分包 |
| `installer/` | 当前用户安装和卸载脚本 |

## 源码目录使用
```powershell
.\tools\run-local.cmd
```

## 发布包使用
```powershell
.\management-server.exe
.\client-agent.exe
.\WoW-Manager.exe
```

P22 发布包中，`WoW-Manager.exe` 是本机控制中心，可安装/修复、启动双端、打开 Web、打开日志和卸载。脚本会优先调用 `bin\management-server-core.exe`、`bin\client-agent-core.exe`，避免维护参数传给 GUI launcher。

## Server 维护入口
```powershell
.\tools\start-server.ps1 -Tray
.\tools\start-server.ps1 -OpenBrowser
.\tools\start-server.ps1
```

## Client 维护入口
```powershell
.\tools\start-client.ps1 -RunOnce
.\tools\start-client.ps1 -Monitor
.\tools\start-client.ps1 -OpenLog
.\tools\start-client.ps1 -Setup
.\tools\start-client.ps1 -Notify
.\tools\start-client.ps1 -StartupStatus
.\tools\start-client.ps1 -EnableStartup
.\tools\start-client.ps1 -DisableStartup
.\tools\start-client.ps1 -Tray
.\tools\start-client.ps1 -SettingsWindow
.\tools\start-client.ps1 -ServiceStatus
.\tools\start-client.ps1 -UpdateCheck
.\tools\start-client.ps1 -UpdateDownload
.\tools\start-client.ps1 -UpdateApply
```

## 正式打包
```powershell
pwsh -NoProfile -ExecutionPolicy Bypass -File .\tools\package-release.ps1
```

说明：
- 打包脚本需要处理中文源码目录，使用 PowerShell 7 (`pwsh`) 执行。
- 发布包内安装/卸载脚本为 ASCII，可由 Windows PowerShell 直接运行。
- 输出文件包括 `WoW_Framework_vX.Y.Z_windows.zip`、`WoW_Server_vX.Y.Z_windows.zip`、`WoW_Client_vX.Y.Z_windows.zip`。
- `WoW_Framework` 总包继续用于当前用户安装器和自动更新兼容。
- P32 起，总包和 Client 分包会从 `I:\图色\工具\大漠\7.2149` 复制 `dm.dll`、`RegDll.dll` 到 `dm-bridge/Win32/`；其他机器打包时可用 `WOW_DM_RUNTIME_DIR` 指定来源目录。
- P35 起，打包脚本会把 `assets/icons/client.ico` 和 `assets/icons/server.ico` 写入发布包内对应 EXE 的图标资源，根目录入口和 `bin/*-core.exe` 都会带正式图标。

## DM 说明
- x64 Client 只能用于基础 Client、Server、Web Admin 和 JSONL 持久化。
- P11 发布包默认 `client-agent.exe` 使用 x86 构建，便于后续接入 32 位大漠。
- 32 位大漠需要 x86 Client、Win32 DmBridge、`dm.dll`、COM 注册和授权。
- P32 起发布包会携带 `dm.dll` 与 `RegDll.dll`，但不会复制授权文件，也不会自动完成 COM 注册。
