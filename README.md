# WoW 自动化框架

## 项目定位
本项目用于沉淀一套 Windows 自动化代理框架：Rust 负责稳定核心，Lua 负责业务脚本，大漠插件通过独立桥接层接入，管理端后续提供集中监控、配置和脚本下发。

## 当前阶段
- 当前阶段：P33 Lua 热推送与内部测试模式
- 当前版本：v1.25.0
- 当前目标：v1.25.0 已完成默认内部测试模式、管理端 Lua 热推送、Lua 启动/停止/状态命令和重复下发拦截，等待打包发布后实机测试

## 第一里程碑
已完成 P0-P33 第一轮源码、Web 信息扩展、短期历史分析、持久化、一键运行、客户端监控、本机开机启动、正式运行基础、双击正式入口、无控制台发布入口、自动更新自替换、服务端远程更新入口、部署分包、正式体验修正、Client 直启热修复、Server 托盘、双端图标、产品控制中心、Web 使用体验与 DM/Lua 操作流、命令执行回执、工程化地基修补、Client 远程配置下发、Client 设置表单化、DM smoke 脚本入口、多机器通讯优化、Server 查询审计能力、DM 正式包能力和 Lua 热推送能力：

1. Client Agent 能执行 Lua bootstrap。
2. DmBridge 能通过 Rust/Lua 调用大漠最小链路。
3. Management Server 能接收和查询 Client 状态。
4. Web Admin 能在浏览器查看 Server 健康和 Client 最新状态。
5. Client Agent 执行 Lua 前会校验 manifest、Ed25519 签名、SHA-256 和权限白名单。
6. 发布归档已补齐交接、部署、安全边界和最终验证资料。
7. Web Admin 能展示在线比例、脚本分布、Agent 运行详情、脚本安全配置和 Server 上报目标。
8. Management Server 能保存短期状态历史，Web Admin 能展示真实历史趋势。
9. Management Server 可选启用 JSONL 历史持久化，重启后可恢复最新状态和短期历史。
10. Management Server 可托管 Web Admin，工具脚本可一键启动本机 Server/Client，Web Admin 提供首次设置向导。
11. Management Server 可内嵌 Web Admin，Client Agent 可常驻监控、记录日志、轮询 Server 消息并弹出通知。
12. Client Agent 可管理当前用户开机启动，Web Admin 可生成对应本机设置命令。
13. Client Agent 可通过 Service、托盘、设置窗口、更新器和 Server 远程白名单命令进入正式运行形态。
14. `management-server.exe` 无参数启动并打开 Web 管理页，`client-agent.exe` 无参数启动托盘常驻 UI。
15. 发布包根目录入口为无控制台 GUI launcher，维护 core exe 放入 `bin`，并提供当前用户安装/卸载入口。
16. Client 支持 `--update-apply`，可下载 GitHub Release 并安排独立 updater 替换安装目录。
17. Web Admin 左侧导航已补齐总览、客户端、脚本、远程操作、设置，Server 可下发 `update.apply` 触发 Client 自替换更新。
18. Management Server 控制台可显示 Client 上线/刷新/离线日志，发布脚本可生成总包、Server 分包和 Client 分包。
19. Client 分包根目录 `client-agent.exe` 可正式直启，正常退出会上报离线，Web Admin 远程操作可选择单台或全部 Client。
20. Client 分包根目录 `client-agent.exe` 真实双击入口已修复，可启动托盘宿主和 x86 monitor 并向 Server 上报在线。
21. Server 分包根目录 `management-server.exe` 可启动右下角托盘，提供启动、关闭、重启、打开 Web 和日志目录快捷操作；Server/Client 托盘均使用独立图标。
22. `WoW-Manager.exe` 已升级为本机控制中心，可安装/修复、启动双端、打开 Web、打开日志、打开目录和卸载。
23. Web Admin 已增强 Client 列表、快照分析、设置向导、DM/Lua 面板和 `script.run_bootstrap` 远程白名单命令。
24. 远程白名单命令执行后，Client 会向 Server 回传成功或失败回执，Web Admin 可查看单台 Client 最近执行结果。
25. 远程命令清单已收敛到 `shared-types`，Server app 主文件已拆分，Client 命令错误已类型化，并新增 GitHub Actions CI。
26. Web Admin 可对指定 Client 下发受控本机设置，Client 写回 TOML 并在 monitor 下一轮自动重载。
27. Client 本机设置窗口已改为表单 UI，可通过输入框、复选框和权限勾选保存设置。
28. Client 提供 `dm_smoke.lua` 实机烟测脚本，可验证 Lua、DmBridge 和大漠 COM 的最小链路；多机器管理和通讯效率优化已形成 P28 规划。
29. Client 状态、设置窗口和 Web Admin 已支持显示名、分组和标签，批量消息/命令下发前必须显式确认。
30. Client monitor 已支持 jitter 和 `/api/client/sync` 合并同步，减少多机器轮询请求量，并保留旧接口回退链路。
31. Server 已支持 Client 状态分页、分组、标签、在线状态和关键字查询，并把消息、命令和命令回执写入可选 JSONL 审计文件；Web Admin 已接入 Server 端筛选和审计面板。
32. Client 默认开启 `dm.access`，总包和 Client 分包携带 `dm.dll/RegDll.dll`，Web 远程操作目标改为可勾选 Client 列表。
33. Client 默认进入内部测试模式：Lua 开启、脚本安全校验默认关闭、已知权限默认全开；Web Admin 可向勾选 Client 热推送 Lua，并下发 Lua 启动、停止和状态查询命令。

## 目录说明
| 目录 | 职责 |
|------|------|
| `计划报告/` | 阶段计划、步骤记录、进度、风险、变更 |
| `技术设计/` | 架构设计、MVP 收敛方案、协议设计 |
| `实现模块/` | Rust、Delphi、Web 等实现模块 |
| `测试验证/` | 编译检查、烟测、测试记录 |
| `发布归档/` | 版本发布、交付资料、归档说明 |
| `tools/` | 一键运行和本机试运行脚本 |

## 验证命令
```powershell
cargo test --workspace
cargo clippy --workspace -- -D warnings
cd 实现模块/web-admin
npm run build
```

## 发布归档
当前 v1.25.0 正式测试包待发布；上一版 v1.24.0 已发布：https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.24.0
