# 测试验证说明

本目录保存编译检查、烟测、测试报告和手动验证记录。

## 当前验证项
| 验证项 | 命令 | 状态 |
|--------|------|------|
| Rust 格式化 | `cargo fmt --all` | 已通过 |
| Rust Clippy | `cargo clippy --workspace -- -D warnings` | 已通过 |
| Rust workspace 测试 | `cargo test --workspace` | 已通过 |
| Client 状态输出 | `cargo run -p client-agent` | 已通过 |
| Server 契约输出 | `cargo run -p management-server` | 已通过 |
| DmBridge Win32 编译 | `.\实现模块\dm-bridge\build.ps1` | 已通过 |
| DmBridge 导出符号检查 | `tdump -ee target\dm-bridge\Win32\DmBridge.dll` | 已通过 |
| DmBridge 32 位 ABI 测试 | `cargo test -p client-agent --target i686-pc-windows-msvc dm_bridge_loads_abi_version_from_env_when_available` | 已通过 |
| DmBridge 32 位 COM 烟测 | `cargo test -p client-agent --target i686-pc-windows-msvc dm_bridge_com_ver_and_color_smoke_when_enabled` | 已通过 |
| Lua dm 32 位 COM 烟测 | `cargo test -p client-agent --target i686-pc-windows-msvc lua_dm_api_com_ver_and_color_smoke_when_enabled` | 已通过 |
| P3 本地通讯烟测 | 启动 Server、Client 上报、GET 查询状态 | 已通过 |
| Web Admin 生产构建 | `npm run build` | 已通过 |
| P4 Web Admin 浏览器烟测 | Playwright fallback 桌面/移动视口 | 已通过 |
| P5 脚本安全测试 | hash、签名、权限、Lua API 拒绝测试 | 已通过 |
| P6 最终发布验证 | 前后端构建、DmBridge 编译、Server/Client 烟测 | 已通过 |
| P7 Web 信息扩展验证 | 协议扩展、前端构建、浏览器桌面/移动烟测 | 已通过 |
| P8 历史趋势验证 | 历史 API、Web 趋势面板、浏览器桌面/移动烟测 | 已通过 |
| P9 持久化与编译包验证 | JSONL 重启恢复、release build、普通编译包包内烟测 | 已通过 |
| P10 一键运行与首次设置向导验证 | tools 脚本、Web 托管、向导联动、x86/x64 包内烟测 | 已通过 |
| P11 单 exe 与客户端监控验证 | 内嵌 Web、Server 消息、Client monitor、日志和浏览器烟测 | 已通过 |

## P0 验证记录
- `cargo test --workspace`：通过，`shared-types` 单元测试 1 项通过。
- `cargo run -p client-agent`：通过，输出 `schema_version/message_id/message_type/client_id/timestamp_ms/data`。
- `cargo run -p management-server`：通过，服务端入口可复用同一份协议类型。

## P1 验证记录
- `cargo test --workspace`：通过，client-agent 11 项测试通过，shared-types 1 项测试通过。
- `cargo run -p client-agent`：通过，读取 TOML 配置和 `scripts/bootstrap.lua`，输出 `current_script = bootstrap`。
- Lua bootstrap：通过，`log("bootstrap started from file")` 可进入 tracing 日志，`get_config("client.id")` 可读取白名单配置。
- Lua 指令上限：通过，`while true do end` 会触发 `Lua 脚本超过指令上限` 错误。
- 代码结构检查：通过，client-agent 当前最大单文件 169 行，入口文件 34 行；大漠 API 已拆到 `src/dm_bridge/` 与 `src/lua_dm.rs`。

## P2 验证记录
- `.\实现模块\dm-bridge\build.ps1`：通过，Win32 `DmBridge.dll` 编译成功。
- `tdump -ee target\dm-bridge\Win32\DmBridge.dll`：通过，`dm_bridge_*` 导出符号完整。
- Delphi 代码结构检查：通过，Worker 已拆分为 Types、Request、Thread、门面四个单元；最大单文件 174 行。
- `dm_bridge_abi_version`：通过，32 位 P/Invoke 和 32 位 Rust 均返回 `1`。
- `dm_bridge_init -> dm_bridge_ver -> dm_bridge_get_color -> dm_bridge_move_to -> dm_bridge_shutdown`：通过，返回大漠版本 `7.2149`，取色返回 `000000`，`MoveTo` 返回 `1`。
- Lua `dm.init -> dm.ver -> dm.get_color -> dm.move_to -> dm.shutdown`：通过，Lua 高层 API 可穿透到 DmBridge。
- 安全说明：自动烟测未执行 `LeftClick`，避免误点击；`LeftClick` 已完成导出和 Rust/Lua 封装。

## P3 验证记录
- `cargo fmt --all`：通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `cargo test --workspace`：通过，client-agent 13 项测试、management-server 4 项测试、shared-types 2 项测试通过。
- `GET /health`：通过，返回 `status = ok`。
- `POST /api/client/status`：通过，Client 上报 `local-dev-client` 状态后 Server 返回 ACK。
- `GET /api/client/status/local-dev-client`：通过，返回 `message_type = status`、`current_script = bootstrap`、`online = true`。
- `cargo run -p client-agent`：默认不上报 Server，仍可独立输出状态 JSON。
- 上报模块拆分后最终烟测：通过，直接启动 `management-server.exe`，运行 `client-agent.exe` 上报到 `127.0.0.1:18082`。
- 烟测残留：已删除 `target/p3-smoke` 临时日志目录。

## P4 验证记录
- `npm run build`：通过，`vue-tsc --noEmit` 与 `vite build` 均成功。
- `cargo fmt --all`：通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `cargo test --workspace`：通过，client-agent 13 项测试、management-server 6 项测试、shared-types 2 项测试通过。
- P4 API 烟测：通过，`GET /api/client/status` 返回 `local-dev-client`，`current_script = bootstrap`。
- Browser/IAB：当前会话未暴露直接浏览器控制工具，使用 Playwright fallback。
- Playwright 桌面视口：1440x920，通过，页面显示 `Server 正常`、`local-dev-client`、`bootstrap`，无横向溢出。
- Playwright 移动视口：390x844，通过，客户端表格改为字段卡片，无文字截断和页面横向溢出。
- 概念图对照：已用生成概念图和最终实现截图做 `view_image` 检查。

## P5 验证记录
- `cargo fmt --all`：通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `cargo test --workspace`：通过，client-agent 20 项测试、management-server 6 项测试、shared-types 2 项测试通过。
- `cargo run -p client-agent`：通过，默认启用 manifest、签名、hash、权限校验后仍输出 `current_script = bootstrap`。
- P5 Server 上报烟测：通过，安全门启用后 Client 仍可上报到 Management Server。
- Web Admin 构建回归：通过，`npm run build` 成功。
- hash 拒绝测试：通过，manifest hash 与 Lua 文件不匹配时拒绝加载。
- 签名拒绝测试：通过，manifest 签名错误时拒绝加载。
- 权限拒绝测试：通过，manifest 请求 `dm.access` 但配置未授权时拒绝加载。
- Lua API 拒绝测试：通过，缺少 `config.read` 时 `get_config` 不注册。

## P6 验证记录
- `npm run build`：通过，`vue-tsc --noEmit` 与 `vite build` 成功。
- `cargo fmt --all --check`：通过，无格式化差异。
- `cargo clippy --workspace -- -D warnings`：通过。
- `cargo test --workspace`：通过，client-agent 20 项测试、management-server 6 项测试、shared-types 2 项测试通过。
- `cargo run -p client-agent`：通过，默认启用脚本安全门后仍输出 `current_script = bootstrap`。
- P6 Server 上报烟测：通过，`GET /health` 返回 `ok`，`GET /api/client/status/local-dev-client` 返回 `online = true`、`current_script = bootstrap`。
- DmBridge Win32 编译：通过，`.\实现模块\dm-bridge\build.ps1` 生成 `target/dm-bridge/Win32/DmBridge.dll`。
- DmBridge 导出符号检查：通过，`tdump -ee target\dm-bridge\Win32\DmBridge.dll` 可看到 `dm_bridge_*` 导出函数。
- 文档烟测修正：部署指南已改用 `CLIENT_AGENT_SERVER_HOST` 和 `CLIENT_AGENT_SERVER_PORT`，避免错误使用不存在的 `CLIENT_AGENT_SERVER_ADDR`。

## P7 验证记录
- `cargo fmt --all --check`：通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `cargo test --workspace`：通过，client-agent 21 项测试、management-server 6 项测试、shared-types 2 项测试通过。
- `npm run build`：通过，`vue-tsc --noEmit` 与 `vite build` 成功。
- `cargo build --workspace`：通过，重建普通二进制后执行 Server/Client 烟测。
- P7 Server 上报烟测：通过，API 返回 `release_version = v1.1.0`、`os = windows`、`arch = x86_64`、`security_enabled = true`、`report_target = 127.0.0.1:18087/api/client/status`。
- Playwright fallback 桌面视口：1440x920，通过，页面显示快照分析、`local-dev-client`、`v1.1.0`、本地设置、脚本安全门，无横向溢出。
- Playwright fallback 移动视口：390x844，通过，分析、列表、设置、详情按单列展示，无横向溢出。
- 发现项：直接运行旧 `target/debug/client-agent.exe` 会得到旧协议；发布烟测前必须先执行 `cargo build --workspace`。

## P8 验证记录
- `cargo fmt --all --check`：通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `cargo test --workspace`：通过，client-agent 21 项测试、management-server 8 项测试、shared-types 3 项测试通过。
- `cargo build --workspace`：通过，重建普通二进制后执行 Server/Client 历史烟测。
- `npm run build`：通过，`vue-tsc --noEmit` 与 `vite build` 成功。
- P8 Server/Client 历史 API 烟测：通过，连续三次真实上报后，`GET /api/client/history/local-dev-client` 返回 `history_total = 3`、`history_limit = 50`、`release_version = v1.2.0`。
- Playwright fallback 桌面视口：1440x920，通过，页面显示 `歷史趨勢`、`3/50`、`v1.2.0`、`local-dev-client`，无横向溢出。
- Playwright fallback 移动视口：390x844，通过，历史趋势、快照、列表、设置、详情单列展示，无横向溢出。
- `view_image` 截图复查：发现并修复历史趋势面板 SVG 选择器过宽导致图标放大的问题。

## P9 验证记录
- `cargo fmt --all --check`：通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `cargo test --workspace`：通过，client-agent 21 项测试、management-server 13 项测试、shared-types 3 项测试通过。
- `cargo build --workspace`：通过。
- `cargo build --workspace --release`：通过，生成 Windows release 可执行文件。
- `npm run build`：通过，`vue-tsc --noEmit` 与 `vite build` 成功，Web Admin 版本为 `1.3.0`。
- P9 持久化烟测：通过，Server 启用 `MANAGEMENT_SERVER_HISTORY_PATH` 后连续接收两次 Client 上报，JSONL 文件写入 2 行。
- P9 重启恢复烟测：通过，Server 停止后使用同一 JSONL 文件重启，`GET /api/client/history/local-dev-client` 仍返回 `history_total = 2`。
- P9 版本烟测：通过，恢复后的最新状态返回 `release_version = v1.3.0`、`current_script = bootstrap`。
- 普通编译包包内烟测：通过，从包根目录运行 `bin/management-server.exe` 和 `bin/client-agent.exe`，历史 API 返回 `history_total = 1`。
- 编译包敏感文件检查：通过，未包含 `dm.dll`、`RegDll.dll`、CHM/CHW、`.env` 和 JSONL 历史文件。

## P10 验证记录
- `cargo fmt --all --check`：通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `cargo test --workspace`：通过，client-agent 21 项测试、management-server 15 项测试、shared-types 3 项测试通过。
- `npm run build`：通过，`vue-tsc --noEmit` 与 `vite build` 成功，Web Admin 版本为 `1.4.0`。
- `pwsh -File tools/start-client.ps1 -DisableReport`：通过，源码脚本会先重建 Client，状态输出 `release_version = v1.4.0`。
- P10 Server/Web/API 联调：通过，`MANAGEMENT_SERVER_WEB_DIR` 托管 Web Admin，`GET /health = ok`，Client 上报后 API 返回 `release_version = v1.4.0`，首页 HTTP 200。
- Playwright fallback 桌面视口：1440x1000，通过，页面显示 `首次設定向導`、`Server 啟動命令`、`客戶端狀態`，无页面横向溢出。
- Playwright fallback 移动视口：390x920，通过，移动导航和向导单列展示，无页面横向溢出。
- 首次设置向导联动：通过，端口从 `18080` 改为 `18130` 后点击 `套用並完成`，看板自动刷新并显示 `local-dev-client`、`1/1`、`v1.4.0`。
- v1.4.0 编译包包内烟测：通过，包内脚本启动 Server，x64 Client 上报成功，历史文件写入 1 行。
- x86 Client 包内烟测：通过，`tools/start-client.ps1 -ClientArch x86 -DisableReport` 输出 `arch = x86`、`release_version = v1.4.0`。
- 编译包敏感文件检查：通过，未包含 `dm.dll`、`RegDll.dll`、CHM/CHW、`.env`、JSONL、PDB、DCU 和 MAP 文件。

## P11 验证记录
- `cargo fmt --all --check`：通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `cargo test --workspace`：通过，client-agent 22 项测试、management-server 20 项测试、shared-types 4 项测试通过。
- `npm run build`：通过，Web Admin 版本为 `1.5.0`。
- Server 内嵌 Web 烟测：通过，不设置 `MANAGEMENT_SERVER_WEB_DIR` 时，`management-server.exe` 首页 HTTP 200。
- Server 消息 API 烟测：通过，`POST /api/client/messages/local-dev-client` 创建消息，`GET` 返回 `total = 1`。
- Client monitor 烟测：通过，`client-agent.exe --monitor` 周期上报状态并写入 `logs/status-history.jsonl`。
- Client 消息日志烟测：通过，monitor 收到 Server 消息后写入 `logs/client-agent.log`。
- Playwright fallback 桌面视口：1440x1000，通过，页面显示 `Server 消息` 表单、`local-dev-client`、`v1.5.0`，无横向溢出。
- Playwright fallback 移动视口：390x920，通过，消息表单和详情面板单列展示，无横向溢出。
- v1.5.0 编译包包内烟测：通过，根目录 `management-server.exe` 和 x86 `client-agent.exe --monitor` 可联动，Client 日志包含 `package message works`。
- 编译包敏感文件检查：通过，未包含 `dm.dll`、`RegDll.dll`、CHM/CHW、`.env`、JSONL、PDB、DCU 和 MAP 文件。
