# WoW 自动化框架

## 项目定位
本项目用于沉淀一套 Windows 自动化代理框架：Rust 负责稳定核心，Lua 负责业务脚本，大漠插件通过独立桥接层接入，管理端后续提供集中监控、配置和脚本下发。

## 当前阶段
- 当前阶段：P8 历史状态与趋势分析
- 当前版本：v1.2.0
- 当前目标：保存短期状态历史并在 Web Admin 展示真实趋势

## 第一里程碑
已完成 P0-P8 第一轮源码、Web 信息扩展与短期历史分析闭环：

1. Client Agent 能执行 Lua bootstrap。
2. DmBridge 能通过 Rust/Lua 调用大漠最小链路。
3. Management Server 能接收和查询 Client 状态。
4. Web Admin 能在浏览器查看 Server 健康和 Client 最新状态。
5. Client Agent 执行 Lua 前会校验 manifest、Ed25519 签名、SHA-256 和权限白名单。
6. 发布归档已补齐交接、部署、安全边界和最终验证资料。
7. Web Admin 能展示在线比例、脚本分布、Agent 运行详情、脚本安全配置和 Server 上报目标。
8. Management Server 能保存短期状态历史，Web Admin 能展示真实历史趋势。

## 目录说明
| 目录 | 职责 |
|------|------|
| `计划报告/` | 阶段计划、步骤记录、进度、风险、变更 |
| `技术设计/` | 架构设计、MVP 收敛方案、协议设计 |
| `实现模块/` | Rust、Delphi、Web 等实现模块 |
| `测试验证/` | 编译检查、烟测、测试记录 |
| `发布归档/` | 版本发布、交付资料、归档说明 |

## 验证命令
```powershell
cargo test --workspace
cargo clippy --workspace -- -D warnings
cd 实现模块/web-admin
npm run build
```

## 发布归档
v1.2.0 发布资料见：`发布归档/README.md`。
