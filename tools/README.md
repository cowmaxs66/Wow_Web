# 工具脚本说明

本目录保存 P10-P15 一键运行、维护和打包脚本。P15 起普通用户优先直接双击根目录 GUI 入口，核心维护 exe 放入发布包 `bin` 目录。

## 脚本清单
| 文件 | 职责 |
|------|------|
| `common.ps1` | 公共路径解析、构建检测和健康检查函数 |
| `start-server.ps1` | 启动 Management Server，可同时托管 Web Admin |
| `start-client.ps1` | 运行 Client Agent，可执行 monitor、tray、settings、service、update 和开机启动设置 |
| `start-server.cmd` | Windows 双击入口，调用 `start-server.ps1` |
| `start-client.cmd` | Windows 双击入口，调用 `start-client.ps1` |
| `run-local.cmd` | Windows 双击入口，启动 Server 后启动 Client 托盘 |
| `package-release.ps1` | 生成正式发布包，根目录放 GUI launcher，`bin` 放 core exe |
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

P15 发布包中，脚本会优先调用 `bin\management-server-core.exe`、`bin\client-agent-core.exe`，避免维护参数传给 GUI launcher。

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

## DM 说明
- x64 Client 只能用于基础 Client、Server、Web Admin 和 JSONL 持久化。
- P11 发布包默认 `client-agent.exe` 使用 x86 构建，便于后续接入 32 位大漠。
- 32 位大漠需要 x86 Client、Win32 DmBridge 和本机私有 `dm.dll`/注册信息。
- 本脚本不会复制或注册大漠 DLL。
