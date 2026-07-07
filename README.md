# WoW 自动化框架

## 项目定位
本项目用于沉淀一套 Windows 自动化代理框架：Rust 负责稳定核心，Lua 负责业务脚本，大漠插件通过独立桥接层接入，管理端后续提供集中监控、配置和脚本下发。

## 当前阶段
- 当前阶段：P18 服务端上线日志与分包
- 当前版本：v1.12.0
- 当前目标：让 Server 控制台显示 Client 上线/刷新日志，并输出 Server/Client 独立分包

## 第一里程碑
已完成 P0-P18 第一轮源码、Web 信息扩展、短期历史分析、持久化、一键运行、客户端监控、本机开机启动、正式运行基础、双击正式入口、无控制台发布入口、自动更新自替换、服务端远程更新入口和部署分包：

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
v1.12.0 发布资料已归档，GitHub Release：https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.12.0
