# Server 托盘与双端图标设计

## 目标
让 Server 与 Client 都具备正式托盘入口，并使用项目图标资源，减少用户直接面对控制台或无区分默认图标的情况。

## 影响评估
- 不改变 Server API、Web Admin API 和 Client 状态协议。
- 不改变 `bin/*-core.exe` 维护入口。
- 根目录 `management-server.exe` 从直接启动 Server 改为启动 Server 托盘，托盘负责拉起 `management-server-core.exe --no-open-browser`。
- Client 托盘只增加图标加载，不改变 monitor、Service、设置、更新菜单语义。

## 图标资源
- 图标源文件归档到 `assets/icons/`。
- 运行时托盘读取 `.ico`：
  - Server：`assets/icons/server.ico`
  - Client：`assets/icons/client.ico`
- 如果图标文件缺失，托盘回退到系统默认应用图标，避免启动失败。

## Server 托盘行为
Server 托盘由 `management-server-core.exe --tray` 生成 PowerShell STA 托盘宿主。

菜单项：
- 启动 Server
- 关闭 Server
- 重启 Server
- 打开 Web 管理页
- 打开日志目录
- 仅退出托盘
- 退出托盘并关闭 Server

## 日志边界
Server 正式入口没有控制台，因此托盘启动的 Server 会将输出重定向到：
- `logs/management-server.log`
- `logs/management-server-error.log`
- `logs/server-tray-error.log`

## 非目标
- 本阶段不新增 Server Windows Service。
- 本阶段不新增服务安装器、服务恢复策略或系统级守护。
- 本阶段不新增登录鉴权、命令审计和命令执行回执。
