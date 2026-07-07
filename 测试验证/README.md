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
| P12 Client 本机设置与开机启动验证 | HKCU 开机启动、工具脚本、Web 向导命令 | 已通过 |
| P13 正式运行基础验证 | Service、托盘、设置窗口、更新器、远程命令 | 已通过 |
| P14 双击正式入口验证 | 无参数正式入口和维护参数兼容 | 已通过 |
| P15 无控制台正式入口与安装器验证 | GUI launcher、当前用户安装器、打包和包内烟测 | 已通过 |
| P16 自动更新自替换验证 | `update-apply`、打包脚本、包内烟测和敏感文件检查 | 已通过 |
| P17 服务端远程更新与导航补全验证 | Server `update.apply`、在线收敛、Web 导航功能页 | 已通过 |
| P18 服务端上线日志与分包验证 | Server 控制台上线日志、Server/Client 分包、三类 zip 安全检查 | 已通过 |
| P19 客户端直启与远程目标选择修正验证 | Client 分包默认上报、离线上报、Web 目标选择、三类 zip 安全检查 | 已通过 |
| P20 Client 正式直启热修复验证 | 根目录 `client-agent.exe` 真实直启、PowerShell 编码、STA 和隐藏启动链路 | 已通过 |
| P21 Server 托盘与双端图标验证 | Server 托盘真实入口、Client 托盘图标回归、三类 zip 图标资源 | 已通过 |
| P22 产品化控制中心与安装体验验证 | `WoW-Manager.exe` 控制中心入口、脚本语法、三类 zip 生成 | 已通过 |
| P23 Web 使用体验与 DM/Lua 操作流验证 | Client 列表、仪表盘、设置向导、DM/Lua 面板、`script.run_bootstrap` 和三类 zip | 已通过 |
| P24 命令执行回执验证 | Server 回执 API、Client 回执上报、Web 最近回执、三类 zip | 已通过 |
| P25 工程化地基验证 | 共享命令清单、错误类型、app 拆分、CI 配置、Rust/Web 构建 | 已通过 |

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

## P12 Client 本机设置与开机启动验证
- `cargo fmt --all --check`：通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `cargo test --workspace`：通过，client-agent 26 项测试、management-server 20 项测试、shared-types 4 项测试通过。
- `cargo test -p client-agent`：通过，26 项测试覆盖 CLI 解析、启动项命令格式和注册表输出解析。
- `npm run build`：通过，Web Admin 首次设置向导新增开机启动命令后构建成功。
- `cargo run -p client-agent -- --startup-status`：通过，只读查询当前用户开机启动状态，未修改注册表。
- `tools/start-client.ps1 -ClientArch x64 -StartupStatus`：通过，工具脚本可调用只读查询。
- Playwright fallback 桌面视口：1440x1000，通过，页面显示 `Client 開機啟動`、`-StartupStatus`、`-EnableStartup`、`-DisableStartup`，无横向溢出。
- Playwright fallback 移动视口：390x920，通过，开机启动命令区单列展示，无横向溢出。
- v1.6.0 编译包包内烟测：通过，根目录 `management-server.exe` 和 x86 `client-agent.exe --monitor` 可联动，Client 日志包含 `startup package message works`。
- 包内 `client-agent.exe --startup-status`：通过，只读查询输出注册项 `WoW Client Agent`，未修改注册表。
- 编译包敏感文件检查：通过，未包含 `dm.dll`、`RegDll.dll`、CHM/CHW、`.env`、JSONL、PDB、DCU 和 MAP 文件。

## P13 正式运行基础验证
- `cargo fmt --all --check`：通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `cargo test --workspace`：通过，client-agent 33 项测试、management-server 23 项测试、shared-types 5 项测试通过。
- `npm run build`：通过，Web Admin 版本为 `1.7.0`。
- `client-agent.exe --service-status`：通过，未安装时返回明确状态。
- `client-agent.exe --update-check`：通过，可读取 GitHub latest release。
- Server 远程命令烟测：通过，Server 下发 `startup.status`，Client monitor 轮询执行并写入本地日志。
- 命令队列消费检查：通过，Client 拉取后队列清空，避免重启后重复执行旧命令。
- In-app Browser 桌面视口：通过，`远程本机操作` 区显示完整命令列表，选择 `查询 Service` 后可写入命令队列，无控制台错误和横向溢出。
- In-app Browser 移动视口：390x844，通过，远程操作区 DOM 完整，无控制台错误和横向溢出；已修正历史趋势图移动端采样点过密的视觉问题。
- v1.7.0 编译包包内烟测：通过，根目录 `management-server.exe` 内嵌 Web HTTP 200，x86 `client-agent.exe --monitor` 可上报 `v1.7.0` 并执行 `startup.status` 远程命令。
- 包内 `client-agent.exe --service-status`：通过，只读查询输出 `WoWClientAgent` 未安装或不可查询，未修改系统服务表。
- 编译包敏感文件检查：通过，未包含 `dm.dll`、`RegDll.dll`、CHM/CHW、授权文件、`.env`、JSONL、PDB、DCU、MAP 和私有资料。

## P14 双击正式入口验证
- `cargo fmt --all --check`：通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `cargo test --workspace`：通过，client-agent 35 项测试、management-server 25 项测试、shared-types 5 项测试通过。
- `npm run build`：通过，Web Admin 版本为 `1.8.0`。
- `cargo run -p client-agent -- --run-once`：通过，维护入口仍输出 `release_version = v1.8.0`、`client_id = local-dev-client`、`current_script = bootstrap`。
- Server `--no-open-browser` 烟测：通过，临时端口 `/health` 返回 `ok`，用于验证自动化场景不弹浏览器。
- CLI 单元测试：通过，`client-agent.exe` 无参数解析为托盘入口，`management-server.exe` 无参数解析为打开浏览器。
- v1.8.0 编译包包内烟测：通过，`management-server.exe --no-open-browser` 内嵌 Web HTTP 200，x86 `client-agent.exe --run-once` 输出 `v1.8.0`，`client-agent.exe --startup-status` 的期望命令不再包含 `--monitor`。
- 编译包敏感文件检查：通过，未包含 `dm.dll`、`RegDll.dll`、CHM/CHW、授权文件、`.env`、JSONL、PDB、DCU、MAP 和私有资料。

## P15 无控制台正式入口与安装器验证
- `cargo fmt --all`：通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `cargo test --workspace`：通过，client-agent 36 项测试、management-server 25 项测试、shared-types 5 项测试、wow-launcher 3 项测试通过。
- `npm run build`：通过，Web Admin 版本为 `1.9.0`。
- `tools/package-release.ps1`：通过，生成 `WoW_Framework_v1.9.0_windows.zip`。
- DmBridge Win32 编译：通过，生成 `target/dm-bridge/Win32/DmBridge.dll`。
- PE 子系统检查：通过，根目录 `management-server.exe`、`client-agent.exe`、`WoW-Manager.exe`、`WoW-Remove.exe` 均为 GUI 子系统，`bin/*-core.exe` 为 Console 子系统。
- Client core 包内烟测：通过，`bin/client-agent-core.exe --run-once` 输出 `v1.9.0` 和 `local-dev-client`。
- 开机启动期望命令烟测：通过，`bin/client-agent-core.exe --startup-status` 的期望命令指向根目录 `client-agent.exe` launcher。
- Server core 包内烟测：通过，`bin/management-server-core.exe --no-open-browser` 临时端口 `/health` 返回 `ok`，内嵌 Web HTTP 200。
- 安装/卸载脚本语法检查：通过，`install-current-user.ps1` 和 `uninstall-current-user.ps1` 解析错误数为 0。
- 编译包敏感文件检查：通过，zip 内未包含 `dm.dll`、`RegDll.dll`、CHM/CHW、授权文件、`.env`、JSONL、PDB、DCU、MAP 和私有资料。

## P16 自动更新自替换验证
- `cargo fmt --all --check`：通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `cargo test --workspace`：通过，client-agent 39 项测试、management-server 25 项测试、shared-types 5 项测试、wow-launcher 3 项测试通过。
- `npm run build`：通过，Web Admin 版本为 `1.10.0`。
- `tools/package-release.ps1`：通过，生成 `WoW_Framework_v1.10.0_windows.zip`。
- DmBridge Win32 编译：通过，生成 `target/dm-bridge/Win32/DmBridge.dll`。
- PE 子系统检查：通过，根目录 GUI launcher 与 `bin` core 维护入口保持 P15 结构。
- Client core 包内烟测：通过，`bin/client-agent-core.exe --run-once` 输出 `v1.10.0` 和 `local-dev-client`。
- `--update-apply` 包内烟测：通过，当前 `v1.10.0` 高于远端 latest `v1.9.0` 时返回 `up_to_date`，不会误下载或误降级。
- Server core 包内烟测：通过，`bin/management-server-core.exe --no-open-browser` 临时端口 `/health` 返回 `ok`，内嵌 Web HTTP 200。
- 编译包敏感文件检查：通过，zip 内未包含 `dm.dll`、`RegDll.dll`、CHM/CHW、授权文件、`.env`、JSONL、PDB、DCU、MAP 和私有资料。

## P17 服务端远程更新与导航补全验证
- `cargo fmt --all --check`：通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `cargo test --workspace`：通过，client-agent 39 项测试、management-server 28 项测试、shared-types 5 项测试、wow-launcher 3 项测试通过。
- `npm run build`：通过，Web Admin 版本为 `1.11.0`。
- Server 命令白名单测试：通过，`update.apply` 可创建并被 Client 拉取。
- Server 在线收敛测试：通过，最后上报超过 60 秒的最新状态查询会显示离线。
- Web Admin 构建测试：通过，左侧导航页面、远程操作组件和脚本配置组件均通过 TypeScript 检查。
- `tools/package-release.ps1`：通过，生成 `WoW_Framework_v1.11.0_windows.zip`，SHA-256 为 `9027d9bd52e4b5d21aed82908280185914bd070599c0bd96ba8f2993a62b7b97`。
- DmBridge Win32 编译：通过。
- PE 子系统检查：通过，根目录 GUI launcher 与 `bin` core 维护入口保持 P15 结构。
- Client core 包内烟测：通过，`bin/client-agent-core.exe --run-once` 输出 `v1.11.0` 和 `local-dev-client`。
- `--update-apply` 包内烟测：通过，当前 `v1.11.0` 高于远端 latest `v1.10.0` 时返回 `up_to_date`，不会误下载或误降级。
- Server core 包内烟测：通过，临时端口 `/health` 返回 `ok`。
- Server 远程命令包内烟测：通过，`update.apply` 可写入并取出命令队列。
- 编译包敏感文件检查：通过，zip 内未包含 `dm.dll`、`RegDll.dll`、CHM/CHW、授权文件、`.env`、JSONL、PDB、DCU、MAP 和私有资料。

## P18 服务端上线日志与分包验证
- `cargo fmt --all --check`：通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `cargo test --workspace`：通过，client-agent 39 项测试、management-server 31 项测试、shared-types 5 项测试、wow-launcher 3 项测试通过。
- `npm run build`：通过，Web Admin 版本为 `1.12.0`。
- `tools/package-release.ps1`：通过，生成三类 zip。
- 总包：`WoW_Framework_v1.12.0_windows.zip`，SHA-256 为 `af2825aeb58c1663d97d17ac1c34a53cf29bd939bcbb9fc3f57e4868fb2db95f`，大小 `4289058` bytes。
- Server 分包：`WoW_Server_v1.12.0_windows.zip`，SHA-256 为 `f1db6f39930afdb4e2c27e465ce56ed381a3d8864e197a057ac91b2ac389a843`，大小 `1393362` bytes。
- Client 分包：`WoW_Client_v1.12.0_windows.zip`，SHA-256 为 `16ddb34939c7404c5d38874db859c2785cb80eb7b70517248068a26aff09abe1`，大小 `2652867` bytes。
- Server 控制台上线日志烟测：通过，Server 分包 core 收到 Client 分包上报后输出 `[server] Client 上线: client_id=local-dev-client online=true script=bootstrap release_version=v1.12.0 ...`。
- 分包边界检查：通过，Server 分包不含 `client-agent.exe`、Client core、Client config、scripts、DmBridge；Client 分包不含 `management-server.exe` 和 Server core。
- 三类 zip 敏感文件检查：通过，均未包含 `dm.dll`、`RegDll.dll`、CHM/CHW、授权文件、`.env`、JSONL、PDB、DCU、MAP 和私有资料。

## P19 客户端直启与远程目标选择修正验证
- `cargo fmt --all --check`：通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `cargo test --workspace`：通过。
- `npm run build`：通过，Web Admin 版本为 `1.13.0`。
- `tools/package-release.ps1`：通过，生成总包、Server 分包和 Client 分包。
- Client 分包默认上报烟测：通过，未设置 `CLIENT_AGENT_SERVER_ENABLED` 时，Client 分包可向 Server 上报 `v1.13.0`。
- 离线上报烟测：通过，正常停止 monitor 后可上报 `online = false`。
- Web Admin 目标选择：通过，远程操作支持单台 Client 或全部已上报 Client。
- 三类 zip 敏感文件检查：通过，均未包含 `dm.dll`、`RegDll.dll`、CHM/CHW、授权文件、`.env`、JSONL、PDB、DCU、MAP 和私有资料。

## P20 Client 正式直启热修复验证
- `cargo fmt --all --check`：通过。
- `cargo test -p client-agent`：通过，41 项测试通过。
- `cargo test --workspace`：通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `npm run build`：通过，Web Admin 版本为 `1.13.1`。
- `tools/package-release.ps1`：通过，三类 Windows zip 已生成。
- DmBridge Win32 编译：通过，生成 `target/dm-bridge/Win32/DmBridge.dll`。
- Client 分包根目录 exe 真实直启烟测：通过，直接启动 `WoW_Client_v1.13.1_windows/client-agent.exe` 后产生 `powershell.exe -STA ... tray.ps1` 和 `client-agent-core.exe --monitor` 两个进程。
- Server 状态查询：通过，`local-dev-client` 返回 `online = true`、`release_version = v1.13.1`、`arch = x86`。
- 托盘错误日志：通过，`logs/tray-error.log` 长度为 0。
- 三类 zip SHA-256：
  - 总包 `3ece2cafba9063ff122a5393179b8fe5cdaf8b7c25a431ff3653cfe3ccaf137a`
  - Server 分包 `00b63001a781d6c19bc9ef85b47c03677a803574081d7a74939296ea0667e6f3`
  - Client 分包 `85206e3bcf2079f75443794bff7685a9660159017ec0218bfeb42ad4d4dfa292`

## P21 Server 托盘与双端图标验证
- `cargo fmt --all`：通过。
- `cargo test --workspace`：通过，management-server 新增 Server 托盘 CLI 和 URL 测试，wow-launcher Server 启动计划测试通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `npm run build`：通过，Web Admin 版本为 `1.14.0`。
- `tools/package-release.ps1`：通过，三类 Windows zip 已生成。
- Server 分包根目录 exe 真实入口烟测：通过，直接启动 `WoW_Server_v1.14.0_windows/management-server.exe` 后产生 `powershell.exe -STA ... wow-management-server/tray.ps1` 和 `management-server-core.exe --no-open-browser` 两个进程。
- Server 健康检查：通过，测试端口 `/health` 返回 `status = ok`。
- Server 托盘错误日志：通过，`logs/server-tray-error.log` 长度为 0。
- Server 图标资源：通过，Server 分包包含 `assets/icons/server.ico` 和 `assets/icons/lua_ai_server_icon.svg`。
- Client 分包根目录 exe 回归烟测：通过，`client-agent.exe` 启动托盘宿主和 x86 monitor，Server 查询 `online = true`、`release_version = v1.14.0`、`arch = x86`。
- Client 图标资源：通过，Client 分包包含 `assets/icons/client.ico` 和 `assets/icons/lua_ai_client_icon.svg`。
- 三类 zip SHA-256：
  - 总包 `2bea315e31595fa4df8c1e54e459b175524085ed7901f990af6520f6e204a942`
  - Server 分包 `fa5a85231e01a83fd5dadc9d49aa202a08fddb8f715a79b6f48b65e8bb8c4236`
  - Client 分包 `73130bf34be397a911d08b2dc332392d4e1f48bfc375332f68dd373cc2b56bd1`

## P22 产品化控制中心与安装体验验证
- `cargo fmt --all --check`：通过。
- `cargo test --workspace`：通过，wow-launcher Manager 脚本启动计划测试通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `npm run build`：通过，Web Admin 版本为 `1.15.0`。
- PowerShell 脚本语法检查：通过，`manager-current-user.ps1`、安装脚本、卸载脚本、打包脚本和 Server 启动脚本均 0 语法错误。
- `tools/package-release.ps1`：通过，三类 Windows zip 已生成。
- 控制中心入口烟测：通过，`WoW-Manager.exe` 启动 `powershell.exe -STA ... manager-current-user.ps1 -ShowMessage`。
- 控制中心入口错误日志：通过，`launcher-error.log` 未生成错误内容。
- 总包 `RUNNING.md`：通过，已说明 `WoW-Manager.exe` 是 control center。
- 三类 zip SHA-256：
  - 总包 `58cbf6ec1c84a2a7760b0a90ebdc28bc7a91d14addb2d0630e18f5917a8427fd`
  - Server 分包 `67bcf1a6f193c7b29dacc7230699030eeccdd16a7f14100a94fbfca695d9c097`
  - Client 分包 `74099573c2226b423ec7457095ad28211a301fe54ade8fb615b6a4e29262ba22`

## P23 Web 使用体验与 DM/Lua 操作流验证
- `cargo fmt --all --check`：通过。
- `cargo test --workspace`：通过，Client、Server、shared-types、launcher 共 86 个测试全部通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `npm run build`：通过，Web Admin 版本为 `1.16.0`。
- `tools/package-release.ps1`：通过，三类 Windows zip 已生成。
- 浏览器桌面验证：通过，Client 列表、脚本页、设置页、筛选和进阶排错折叠正常，控制台无错误。
- 浏览器移动端验证：通过，Client 列表卡片布局和设置页无横向溢出，控制台无错误。
- Server 分包烟测：通过，临时端口 `/health` 返回 `ok`，内嵌 Web HTTP 200，`script.run_bootstrap` 可写入并取出命令队列。
- Client 分包烟测：通过，`client-agent-core.exe --run-once` 能执行本机 Lua bootstrap，输出 `release_version = v1.16.0`、`current_script = bootstrap`。
- 三类 zip SHA-256：
  - 总包 `1e9c5e8d3355f9ad072a50ce9eb47a5f0607d91136e705bff2717d0d0769078d`
  - Server 分包 `7e5b720eeb476ded8ac112de92f1289a221cce12e1b320ab70dc109b734b9edb`
  - Client 分包 `399d325a437f029d4c315bbfd141018be5ea86f4166e586a00528a4604dd2930`

## P24 命令执行回执验证
- `cargo fmt --all --check`：通过。
- `cargo test --workspace`：通过，Client、Server、shared-types、launcher 测试全部通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `npm run build`：通过，Web Admin 版本为 `1.17.0`。
- Server 回执 API 单元测试：通过，`POST/GET /api/client/command-receipts/{client_id}` 可创建并查询回执。
- Client reporter 单元测试：通过，`report_command_receipt` 会 POST 到正确路径并解析 Server 响应。
- Web Admin 构建验证：通过，远程操作页新增“最近执行回执”列表，无 TypeScript 错误。
- 浏览器烟测：通过，远程操作页显示“最近执行回执”和 `P24 浏览器烟测回执：startup.status 已执行`，控制台错误数为 0。
- 包内闭环烟测：通过，Server 分包临时端口 `/health` 返回 `ok`，Client 分包 monitor 上报 `v1.17.0`，下发 `startup.status` 后收到成功回执。
- 三类 zip 敏感文件检查：通过，未包含 `dm.dll`、`RegDll.dll`、CHM/CHW、授权文件、`.env`、PDB、DCU、MAP。
- 三类 zip SHA-256：
  - 总包 `b33a9f3efc3fb4ad493006cf6f081b1af827714a7d839429d7358fb021ac8ca7`
  - Server 分包 `7ab457c001d493041cd8135e5e9d1edc72baa8e921137b4c30f3ceb7ab64fde0`
  - Client 分包 `d5ad14c5c92c0f1734fdb5f532a2fd3b8dba8d30b91e89af20f7440500fcd448`

## P25 工程化地基验证
- `cargo fmt --all --check`：首次发现 `app_tests.rs` 文件头空行，已执行 `cargo fmt --all` 修正。
- `cargo test --workspace`：通过，Client、Server、shared-types、launcher 测试全部通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `npm run build`：通过，Web Admin 版本为 `1.18.0`。
- 共享命令清单测试：通过，`REMOTE_COMMAND_TYPES` 接受已知命令并拒绝 `shell.exec`。
- Server app 拆分验证：通过，原 app 路由测试迁移到 `app_tests.rs` 后全部通过。
- CI 配置范围：已新增 `.github/workflows/ci.yml`，远端将验证 Rust 和 Web，不包含 Delphi 打包。
- `tools/package-release.ps1`：通过，三类 Windows zip 已生成。
- 包内闭环烟测：通过，Server 分包临时端口 `/health` 返回 `ok`，Client 分包 monitor 上报 `v1.18.0`，下发 `startup.status` 后收到成功回执。
- 三类 zip 敏感文件检查：通过，未包含 `dm.dll`、`RegDll.dll`、CHM/CHW、授权文件、`.env`、PDB、DCU、MAP。
- 三类 zip SHA-256：
  - 总包 `0a266237a70a88583d9d01bc13b75cc9a62d93405202e832304c5bb4c1a761f1`
  - Server 分包 `1ea56c01b1115109ed6b8c2b07584b8371b1ebecce293e942611ea2c2e7813a7`
  - Client 分包 `6e4a926e0f081eb7f20822bdb8f73a50861f7c4c9d0c8c4f56494d2674eb6762`
