# web-admin 说明

## 职责
Web 管理端，后续负责：
- 客户端状态列表。
- 实时日志查看。
- 配置编辑和下发。
- 脚本库、版本和指派管理。

## 当前状态
P4 阶段已完成 Web 管理端 MVP，可读取 Management Server 健康状态和 Client 最新状态。

## 当前目录
| 路径 | 职责 |
|------|------|
| `src/App.vue` | 页面组合入口 |
| `src/composables/useDashboardStatus.ts` | 状态刷新流程与页面状态 |
| `src/api/managementServer.ts` | Management Server HTTP API 客户端 |
| `src/types/protocol.ts` | 与 Rust 协议对齐的前端类型 |
| `src/components/` | App Shell、指标、列表、详情、连接设置 |
| `src/styles/` | 设计 tokens 与全局样式 |

## 当前能力
- 显示 Server 健康状态。
- 显示在线 Client 数量。
- 显示选中 Client 当前脚本和最近上报时间。
- 显示 Client 列表和状态详情。
- 支持修改 Server 地址和 Client ID 后手动刷新。

## 验证命令
```powershell
npm install
npm run build
npm run dev -- --port 5173
```

## 联调前置
先启动 Management Server，并让 Client Agent 上报一次：

```powershell
cargo run -p management-server
$env:CLIENT_AGENT_SERVER_ENABLED='1'
cargo run -p client-agent
```
