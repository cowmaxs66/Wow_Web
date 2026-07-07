# web-admin 说明

## 职责
Web 管理端，后续负责：
- 客户端状态列表。
- 实时日志查看。
- 配置编辑和下发。
- 脚本库、版本和指派管理。

## 当前状态
P23 阶段已完成 Web 使用体验与 DM/Lua 操作流收敛。当前页面可读取 Management Server 健康状态、Client 最新状态、短期历史、快照分析、Agent 运行详情、脚本安全配置、本地连接设置，可发送 Server 消息，可下发白名单本机操作，并可通过 `script.run_bootstrap` 让 Client 重新执行本机 Lua bootstrap。

## 当前目录
| 路径 | 职责 |
|------|------|
| `src/App.vue` | 页面组合入口 |
| `src/composables/useSetupWizard.ts` | 首次设置向导状态、命令生成和本地持久化 |
| `src/composables/useDashboardStatus.ts` | 状态刷新流程与页面状态 |
| `src/api/managementServer.ts` | Management Server HTTP API 客户端 |
| `src/types/protocol.ts` | 与 Rust 协议对齐的前端类型 |
| `src/components/SetupWizardPanel.vue` | 首次设置向导面板 |
| `src/components/SnapshotAnalytics.vue` | 基于当前状态快照展示健康分、在线比例、脚本分布、架构分布、DM 权限和风险提示 |
| `src/components/HistoryTrendPanel.vue` | 基于真实历史状态展示样本数、在线样本、趋势线和最近记录 |
| `src/components/ClientDetail.vue` | 展示选中 Client 的基础状态、运行详情、脚本设置和 Server 上报 |
| `src/components/ClientSettingsPanel.vue` | 管理 Web Admin 本地 Server URL 和查询 Client ID |
| `src/components/DmLuaGuidePanel.vue` | 展示 Client 如何使用 Lua、manifest、安全门和 DM 权限 |
| `src/components/` | App Shell、指标、列表、详情、分析和设置组件 |
| `src/styles/` | 设计 tokens 与全局样式 |

## 当前能力
- 显示 Server 健康状态。
- 显示在线 Client 数量。
- 显示选中 Client 当前脚本和最近上报时间。
- 显示当前状态快照分析：在线比例、健康分、脚本分布、架构分布、最新上报、安全门、DM 权限和风险提示。
- 显示选中 Client 最近历史趋势和最近记录。
- 显示可搜索、可筛选的 Client 列表和运行详情。
- 支持修改 Web Admin 本地 Server 地址和查询 Client ID 后手动刷新。
- 支持首次设置向导，保存 Server 地址、历史文件路径、Web 静态目录和 Client 模式，普通界面不再直接展示命令行。
- 支持把 Management Server、Client Agent 和开机启动命令隐藏到进阶排错区域。
- 支持区分 `x64` 核心模式和 `x86` 大漠模式，避免误把 32 位 `dm.dll` 放入 64 位进程。
- 支持解释 Lua bootstrap、manifest、签名、hash、权限白名单和 `dm.access` 如何套用。
- 支持在 Client 详情中发送 Server 消息，供 `client-agent --monitor` 轮询、记录日志和弹出通知。
- 支持在 Client 详情中下发白名单本机操作：script、startup、service、update、settings、log、tray。

## P10 首次设置向导
- 向导配置只保存在浏览器 `localStorage`，不上传到 Server。
- `x64` 核心模式用于普通状态上报和 Web 联调。
- `x86` 大漠模式用于加载 32 位 `DmBridge.dll` 和本机大漠插件。
- 向导主界面只显示标准操作流程；命令只用于进阶排错，正式部署前仍需补齐鉴权、TLS 和运维策略。
- P12 起向导额外生成开机启动命令；浏览器只负责展示命令，真正修改本机设置仍由 `client-agent.exe` 或 `tools/start-client.ps1` 执行。
- P13 起 Client 详情可下发白名单远程命令；当前仍为本机试运行队列，生产联网前必须补鉴权、审计和送达确认。

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
