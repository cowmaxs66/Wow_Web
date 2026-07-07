# wow-launcher 说明

本模块提供发布包正式入口。

## 职责
- 构建无控制台窗口的 Server launcher。
- 构建无控制台窗口的 Client launcher。
- 构建当前用户安装和卸载 launcher。
- 将用户双击入口与核心维护 exe 分离。

## 边界
- 不实现业务逻辑。
- 不修改 Management Server、Client Agent、DmBridge 和 Web Admin 协议。
- 不直接复制或注册大漠 `dm.dll`。
- 不做系统级 MSI 或管理员全局安装。

## 发布包结构
| 文件 | 职责 |
|------|------|
| `management-server.exe` | 正式 Server 双击入口，隐藏启动 `bin/management-server-core.exe` |
| `client-agent.exe` | 正式 Client 双击入口，隐藏启动 `bin/client-agent-core.exe --tray` |
| `WoW-Manager.exe` | 当前用户安装入口，调用 `installer/install-current-user.ps1` |
| `WoW-Remove.exe` | 当前用户卸载入口，调用 `installer/uninstall-current-user.ps1` |
