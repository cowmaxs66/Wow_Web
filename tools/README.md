# 工具脚本说明

本目录保存 P10-P14 一键运行和维护脚本。P14 起普通用户优先直接双击根目录 `management-server.exe` 和 `client-agent.exe`，本目录脚本主要用于源码开发、自动化和维护。

## 脚本清单
| 文件 | 职责 |
|------|------|
| `common.ps1` | 公共路径解析、构建检测和健康检查函数 |
| `start-server.ps1` | 启动 Management Server，可同时托管 Web Admin |
| `start-client.ps1` | 运行 Client Agent，可执行 monitor、tray、settings、service、update 和开机启动设置 |
| `start-server.cmd` | Windows 双击入口，调用 `start-server.ps1` |
| `start-client.cmd` | Windows 双击入口，调用 `start-client.ps1` |
| `run-local.cmd` | Windows 双击入口，启动 Server 后启动 Client 托盘 |

## 源码目录使用
```powershell
.\tools\run-local.cmd
```

## 发布包使用
```powershell
.\tools\run-local.cmd
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
```

## DM 说明
- x64 Client 只能用于基础 Client、Server、Web Admin 和 JSONL 持久化。
- P11 发布包默认 `client-agent.exe` 使用 x86 构建，便于后续接入 32 位大漠。
- 32 位大漠需要 x86 Client、Win32 DmBridge 和本机私有 `dm.dll`/注册信息。
- 本脚本不会复制或注册大漠 DLL。
