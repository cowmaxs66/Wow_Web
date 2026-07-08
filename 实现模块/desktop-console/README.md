# desktop-console 说明

## 职责
提供正式桌面控制台窗口，加载 Management Server 内嵌 Web Admin，但不调用 Edge 浏览器进程。

## 当前边界
- Windows 下使用 WebView2 Runtime 渲染页面，这是 Tauri/Clash Verge 同类桌面壳的常规方式。
- 它不是 Edge 浏览器窗口，不会调用 `msedge.exe --app`。
- 直接双击 `WoW-Desktop.exe` 时，如果本机 Management Server 没有响应，会尝试启动同包内 `bin/management-server-core.exe --no-open-browser`。
- Server、Client、Lua、DM 和远程命令协议不放在本模块内实现。

## 当前目录
| 路径 | 职责 |
|------|------|
| `src/main.rs` | 桌面窗口入口、URL 解析、Server 自动启动、错误日志 |

## 运行方式
```powershell
cargo run -p desktop-console -- --url http://127.0.0.1:18080
```

发布包内入口为：

```text
WoW-Desktop.exe
```
