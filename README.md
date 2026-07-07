# WoW 自动化框架

## 项目定位
本项目用于沉淀一套 Windows 自动化代理框架：Rust 负责稳定核心，Lua 负责业务脚本，大漠插件通过独立桥接层接入，管理端后续提供集中监控、配置和脚本下发。

## 当前阶段
- 当前阶段：P11 单 exe 与客户端监控基础
- 当前版本：v1.5.0
- 当前目标：让 Server/Client 形成更接近真实产品的双 exe 试运行形态，并支持 Server 消息、Client 日志和通知

## 第一里程碑
已完成 P0-P11 第一轮源码、Web 信息扩展、短期历史分析、持久化、一键运行与客户端监控闭环：

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
v1.5.0 发布资料见：`发布归档/README.md`。
