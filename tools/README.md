# 工具脚本说明

本目录保存 P10 一键运行脚本。目录名使用 ASCII，避免 Windows PowerShell 在中文路径下出现 `-File` 解析问题。

## 脚本清单
| 文件 | 职责 |
|------|------|
| `common.ps1` | 公共路径解析、构建检测和健康检查函数 |
| `start-server.ps1` | 启动 Management Server，可同时托管 Web Admin |
| `start-client.ps1` | 运行 Client Agent 并上报状态 |
| `start-server.cmd` | Windows 双击入口，调用 `start-server.ps1` |
| `start-client.cmd` | Windows 双击入口，调用 `start-client.ps1` |
| `run-local.cmd` | Windows 双击入口，启动 Server 后运行一次 Client |

## 源码目录使用
```powershell
.\tools\run-local.cmd
```

## 发布包使用
```powershell
.\tools\run-local.cmd
```

## DM 说明
- x64 Client 只能用于基础 Client、Server、Web Admin 和 JSONL 持久化。
- 32 位大漠需要 x86 Client、Win32 DmBridge 和本机私有 `dm.dll`/注册信息。
- 本脚本不会复制或注册大漠 DLL。
