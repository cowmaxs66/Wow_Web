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
| P26 Client 远程配置下发验证 | `config.apply`、Client 配置写回、monitor 动态重载、包内闭环 | 已通过 |
| P27 Client 原生设置表单化验证 | 设置窗口表单脚本、Rust/Web 全量验证、本地三类包和包内烟测 | 已通过 |
| P28 DM 实机烟测验证 | DM smoke manifest、x86 Client 分包实机烟测、多机通讯规划 | 已通过 |
| P29 多机器管理验证 | Client 身份模型、分组/标签、远程身份配置、批量确认 | 已通过 |
| P30 通讯效率验证 | monitor jitter、`/api/client/sync`、Client sync 优先链路、包内 sync smoke | 已通过 |
| P31 Client 分页过滤与审计持久化验证 | Server 分页过滤、审计 JSONL、Web 审计面板、本地三类包和 GitHub Release | 已通过 |
| P32 DM 正式包与多选客户端操作验证 | 默认 DM 权限、DM DLL 随包、Web Client 多选操作、三类 zip | 已通过 |
| P33 Lua 热推送与内部测试模式源码验证 | 默认内部测试模式、Lua 热推送、Lua 启停状态、重复下发拦截 | 已通过 |
| P34 DM 绑定诊断与桌面控制台源码验证 | BindWindow 诊断、Lua 常用接口、Server 桌面控制台、Client 日志 UI | 已通过 |
| P35 客户端体验与脚本故障恢复验证 | 日志乱码、DM 自动初始化、Lua 故障恢复、控制台 UI 和 EXE 图标 | 已通过 |
| P36 远程设置与脚本日志回执验证 | 远程设置、Lua 输出回执、回执分类展示、三类编译包和 GitHub Release | 已通过 |
| P37 桌面控制台 UI 体验优化验证 | Web 构建、桌面/移动浏览器 QA、日志导航和截图记录 | 已通过 |

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

## P26 Client 远程配置下发验证
- `cargo fmt --all --check`：通过。
- `cargo test --workspace`：通过，Client 45 个测试、Server 42 个测试、shared-types 8 个测试、launcher 3 个测试全部通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `cargo build --workspace`：通过，已刷新普通 debug exe，避免复用旧版本二进制。
- `npm run build`：通过，Web Admin 版本为 `1.19.0`。
- 共享协议测试：通过，`config.apply` 纳入 `REMOTE_COMMAND_TYPES`，`ClientConfigPatch` 能识别空 payload。
- Server API 测试：通过，`config.apply` 空 payload 被拒绝，合法配置补丁可写入命令队列。
- Client 配置补丁测试：通过，权限去重排序、未知权限拒绝、配置写回后可重新加载。
- 临时目录闭环烟测：通过，Server 临时端口 `/health` 返回 `ok`，Client monitor 上报在线，Server 下发 `config.apply`，Client 写回 TOML 并上报成功回执。
- `tools/package-release.ps1`：通过，三类 Windows zip 已生成。
- 包内闭环烟测：通过，Server 分包临时端口 `/health` 返回 `ok`，Client 分包 monitor 上线，Server 下发 `config.apply`，Client 日志记录执行成功和回执成功，下一轮状态刷新后配置生效。
- 三类 zip 敏感文件检查：通过，未包含 `dm.dll`、`RegDll.dll`、CHM/CHW、授权文件、`.env`、PDB、DCU、MAP。
- 三类 zip SHA-256：
  - 总包 `0b03a0790d240e6b9fdac4b26e2a61c7c8f10094d06087b88b352afe897cbdbd`
  - Server 分包 `29ebe747062e59f7b6895297725ad6db8a27ca92a6aaed516c2984de65ef2ea6`
  - Client 分包 `26341adb915672640a10f75ac5ef01469b7cdcbfeafa48f50e406cc154698755`

## P27 Client 原生设置表单化验证
- 设置窗口范围：只验证 `client-agent.exe --settings-window` 生成的 WinForms 表单脚本，不改变 Server 协议和远程配置下发。
- `cargo test -p client-agent settings_window`：通过，确认设置窗口不再使用大文本 TOML 编辑器，并包含 Server、Lua、脚本安全门等表单区域。
- PowerShell parser 静态语法检查：通过，生成脚本可被 Windows PowerShell 解析。
- `cargo fmt --all --check`：通过。
- `cargo test --workspace`：通过，Client 46 个测试、Server 42 个测试、shared-types 8 个测试、launcher 3 个测试全部通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `cargo build --workspace`：通过。
- `npm run build`：通过，Web Admin 版本为 `1.20.0`。
- `tools/package-release.ps1`：通过，三类 Windows zip 已生成。
- Client 分包包内烟测：通过，`bin/client-agent-core.exe --run-once` 输出 `release_version = v1.20.0`、`arch = x86`、`current_script = bootstrap`。
- 三类 zip 敏感文件检查：通过，未包含 `dm.dll`、`RegDll.dll`、CHM/CHW、授权文件、`.env`、PDB、DCU、MAP。
- 三类 zip SHA-256：
  - 总包 `33adc868ae1fe5f4b466fa340739fa4d039b4a02f5bb0d2787b36f9871ba5fc3`
  - Server 分包 `146df267022bb8d76feaecd75c71e4dc632a3b2fbcc610fc2db5b099f09907d8`
  - Client 分包 `9ef5aa373c90e553ed6d007aa711cd95cbac6dcb7f87c42c971e4b7d80d1e2ab`

## P28 DM 实机烟测验证
- `cargo fmt --all --check`：通过。
- `cargo test -p client-agent shipped_dm_smoke_manifest_matches_script_and_permissions`：通过。
- `cargo test --workspace`：通过，Client 47 个测试、Server 42 个测试、shared-types 8 个测试、launcher 3 个测试全部通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `cargo build --workspace`：通过。
- `npm run build`：通过，Web Admin 版本为 `1.21.0`。
- `tools/package-release.ps1`：通过，三类 Windows zip 已生成。
- Client 分包 DM smoke 实机烟测：通过，临时复制 Client 分包并切换配置到 `dm-smoke` 后，`bin/client-agent-core.exe --run-once` 输出 `release_version = v1.21.0`、`current_script = dm-smoke`、`arch = x86`。
- 三类 zip 敏感文件检查：通过，未包含 `dm.dll`、`RegDll.dll`、CHM/CHW、授权文件、`.env`、PDB、DCU、MAP。
- 三类 zip SHA-256：
  - 总包 `dc00902da683c924f780a77fc16220be5b234e5d923fb76f0fc485d81dff835a`
  - Server 分包 `17e3f015c38a7f5a7db4c25adf144531a35cda191ef198cd8b2726dba63bb6f0`
  - Client 分包 `6cd35dffe48a82375cab14e1340802caa5fdde8efedfe64e8346b1b6b1579d72`

## P29/P30 多机器管理与通讯效率验证
- `cargo fmt --all --check`：通过。
- `cargo test --workspace`：通过，Client 48 个测试、Server 43 个测试、shared-types 9 个测试、launcher 3 个测试全部通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `cargo build --workspace`：通过。
- `npm run build`：通过，Web Admin 版本为 `1.22.0`。
- `settings_window_script.ps1` PowerShell AST 解析：通过。
- Server sync API 测试：通过，`POST /api/client/sync` 可保存状态、返回消息、取出命令并清空命令队列。
- Client reporter 测试：通过，`sync_client` 会 POST 到 `/api/client/sync` 并解析 ACK、消息和命令。
- Client 配置补丁测试：通过，远程 `config.apply` 可写回显示名、分组和标签，且不会修改 `client.id`。
- `tools/package-release.ps1`：通过，三类 Windows zip 已生成。
- Client 分包 smoke：通过，`bin/client-agent-core.exe --run-once` 输出 `release_version = v1.22.0`、`identity.group = default`、`tags = local,test`。
- Server 分包 sync smoke：通过，包内 Server 临时端口 `/health` 返回 `ok`，`/api/client/sync` 返回 1 条消息和 1 条命令，随后命令队列清空。
- 三类 zip SHA-256：
  - 总包 `0c5aa5df8f17fb2b99f23c37be6b489b5f22fa3b68351acd5fb41587c46c2795`
  - Server 分包 `cbbbffc9e925312eb27a15884f1817da9effd06bd56dc3b41ca0dba713a3c7c6`
  - Client 分包 `59d576f3be4fb98adcec91976366a0bda65dbae0f016abe7bd814bcac563824f`

## P31 Client 分页过滤与审计持久化验证
- `cargo fmt --all --check`：通过。
- `cargo test --workspace`：通过，Client 48 个测试、Server 49 个测试、shared-types 11 个测试、launcher 3 个测试全部通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `cargo build --workspace`：通过。
- `npm run build`：通过，Web Admin 版本为 `1.23.0`。
- Server 分页过滤 API 测试：通过，`GET /api/client/status-page` 支持分组、标签、在线和分页元数据。
- Server 审计 API 测试：通过，消息、命令和命令回执会生成 `ServerAuditEvent` 并可由 `/api/server/audit` 查询。
- 审计 JSONL 持久化测试：通过，`AuditPersistence` 可追加并回放 JSONL 事件。
- `tools/package-release.ps1`：通过，三类 Windows zip 已生成。
- Client 分包 smoke：通过，`bin/client-agent-core.exe --run-once` 输出 `release_version = v1.23.0`、`identity.group = default`、`tags = local,test`。
- Server 分包分页与审计 smoke：通过，临时端口 `/health` 返回 `ok`，`raid-a` 分页返回 `total = 2`、`total_pages = 2`，`dm` 标签过滤返回 `total = 2`，审计 API 返回 3 条事件，审计 JSONL 写入 3 行。
- 三类 zip SHA-256：
  - 总包 `d450d43db072c35c983689208e098c42959590aebf2175f4e10eac23dd1bf9ab`
  - Server 分包 `f80de951b25743b28d435a8581e93d4267341259e50afa60f7d8e9be83537e02`
  - Client 分包 `76b34c27c67ceb5298f0dddd33c5ed8949337218bf30d7ec4f61823dbe45fc12`

## P32 DM 正式包与多选客户端操作验证
- `cargo fmt --all --check`：通过。
- `cargo test --workspace`：通过，Client 49 个测试、Server 49 个测试、shared-types 11 个测试、launcher 3 个测试全部通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `npm run build`：通过，Web Admin 版本为 `1.24.0`。
- `tools/package-release.ps1` PowerShell AST 解析：通过。
- `tools/package-release.ps1`：通过，三类 Windows zip 已生成。
- Client 分包 smoke：通过，`bin/client-agent-core.exe --run-once` 输出 `release_version = v1.24.0`、`arch = x86`、`allowed_permissions = ["host.log", "config.read", "dm.access"]`。
- Server 分包 smoke：通过，临时端口 `/health` 返回 `{"status":"ok"}`。
- 包内容检查：通过，总包和 Client 分包包含 `DmBridge.dll`、`dm.dll`、`RegDll.dll`；Server 分包不包含这三个 DM 文件。
- 三类 zip SHA-256：
  - 总包 `f13ac447611caef4209865cec2aeaa98a631d59b4a73ee60461d612397d2be4b`
  - Server 分包 `34a26b5a7a0cbe3b2195f7ef91c9e2dd469a7aaf77a4e218b413809bb2866822`
  - Client 分包 `06f0dd76e88a45cda55f69d0a91edc745a4377f6c75fa0925d9790947f4eb29f`

## P33 Lua 热推送与内部测试模式验证
- `cargo fmt --all --check`：通过。
- `cargo test --workspace`：通过，Client 51 个测试、Server 52 个测试、shared-types 11 个测试、launcher 3 个测试全部通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `npm run build`：通过，Web Admin 版本为 `1.25.0`。
- Client 热推送测试：通过，`script.deploy_bundle` 可在无 manifest 的内部测试模式下写入 Lua 文件。
- Client Lua 停止测试：通过，`lua.enabled = false` 时 Client 可生成在线状态但不执行脚本。
- Server 命令校验测试：通过，`script.deploy_bundle` 支持无 manifest 内部测试 payload，并拒绝路径穿越。
- Web Admin 构建验证：通过，Lua 热推送面板、Lua 启停按钮和协议类型均通过 TypeScript 检查。
- `tools/package-release.ps1`：通过，三类 Windows zip 已生成。
- Client 分包 smoke：通过，`bin/client-agent-core.exe --run-once` 输出 `release_version = v1.25.0`、`arch = x86`、`lua.enabled = true`、`script_security.enabled = false` 和 `dm.access`。
- Server 分包 health smoke：通过，临时端口 `/health` 返回 `{"status":"ok"}`。
- Server 分包命令队列 smoke：通过，`script.deploy_bundle` 可写入并取出命令队列。
- 包内容检查：通过，总包和 Client 分包包含 `DmBridge.dll`、`dm.dll`、`RegDll.dll`；Server 分包不包含这三个 DM 文件。
- 三类 zip SHA-256：
  - 总包 `08328ea5fdbc549cd6c884908c9759dda8e4881d74d87344d0e9fe6d29e2f0df`
  - Server 分包 `e834c7890d424e6335ec645447816c638b57b9eb3070036d96e3e392699f3bfa`
  - Client 分包 `cf0bf5c3570e0c8669a493d93c6fc1c266879ab4a29fd95088ff9ca570407938`

## P34 DM 绑定诊断与桌面控制台验证
- `cargo fmt --all --check`：通过。
- `cargo test --workspace`：通过，Client 55 个测试、Server 52 个测试、shared-types 11 个测试、launcher 3 个测试全部通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `npm run build`：通过，Web Admin 版本为 `1.26.0`。
- `实现模块/dm-bridge/build.ps1`：通过，Win32 `DmBridge.dll` 重新编译成功。
- Lua 接口测试：通过，`get_color_rgb` 颜色解析单元测试覆盖正常和异常输入。
- Client 日志 UI 测试：通过，日志窗口脚本包含 DPI 缩放和刷新逻辑。
- Web Admin 构建验证：通过，DM/Lua 样例已改为 `dm.safe_bind_window`。
- `tools/package-release.ps1`：通过，三类 Windows zip 已生成。
- Client 分包 smoke：通过，`bin/client-agent-core.exe --run-once` 输出 `release_version = v1.26.0`、`arch = x86`、Lua 启用、脚本安全关闭和 `dm.access`。
- Server 分包 health smoke：通过，临时端口 `/health` 返回 `{"status":"ok"}`。
- Server 分包命令队列 smoke：通过，`script.deploy_bundle` 可写入并取出命令队列。
- 包内容检查：通过，总包和 Client 分包包含 `DmBridge.dll`、`dm.dll`、`RegDll.dll`；Server 分包不包含这三个 DM 文件。
- 三类 zip SHA-256：
  - 总包 `5e3186e36eb12c2d294c63fba37a6066d2768ba56cd37cd2c11df94dcad995bd`
  - Server 分包 `318bd54b7aea947e0101f9926e58ab515724d8b39a3bd206cebf234afadea703`
  - Client 分包 `8eb6e9cb89ba829cde04917c468b52e3652ed7dd5088bfa6865803abcb8d56d1`

## P35 客户端体验与脚本故障恢复验证
- `cargo fmt --all --check`：通过。
- `cargo test --workspace`：通过，Client 55 个测试、Server 52 个测试、shared-types 11 个测试、launcher 3 个测试全部通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `npm run build`：通过，Web Admin 版本为 `1.27.0`。
- 日志窗口测试：通过，脚本包含 UTF-8 读取和毫秒时间戳转换。
- Web Admin 构建验证：通过，远程操作工作台、DM 示例脚本和协议类型均通过 TypeScript 检查。
- `tools/package-release.ps1`：通过，三类 Windows zip 已生成。
- Client 分包 smoke：通过，`bin/client-agent-core.exe --run-once` 输出 `release_version = v1.27.0`、`arch = x86`、Lua 启用、脚本安全关闭和 `dm.access`。
- Server 分包 health smoke：通过，临时端口 `/health` 返回 `{"status":"ok"}`。
- Server 分包命令队列 smoke：通过，`script.deploy_bundle` 可写入并取出命令队列。
- 包内容检查：通过，总包和 Client 分包包含 `DmBridge.dll`、`dm.dll`、`RegDll.dll`；Server 分包不包含这三个 DM 文件。
- EXE 图标 smoke：通过，Client/Server 根 EXE 和 core EXE 均可读取 32x32 关联图标。
- 三类 zip SHA-256：
  - 总包 `36340771a29e1c87cbfe75ae9ee583e9d27aa8c0818f86a36a7e84b15a9c8443`
  - Server 分包 `7e1bad71d16400214c08ac88145cdea5a2b477a6db5cff26aca6090921a0e39b`
  - Client 分包 `105c6c7d840136b91d019ba94b9a6cdbfdeb4aa8b36a13a28b8ac595fc47aaa4`

## P36 远程设置与脚本日志回执验证
- `cargo fmt --all --check`：通过。
- `cargo test --workspace`：通过，Client 55 个测试、Server 52 个测试、shared-types 11 个测试、launcher 3 个测试全部通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `npm run build`：通过，Web Admin 版本为 `1.28.0`。
- Lua 宿主测试：通过，`log()` 会进入 `ScriptRunReport.log_messages`，并能生成 `[script.result]` / `[script.log]` 回执行。
- Web Admin 构建验证：通过，远程操作页 Client 设置面板、快捷模板和回执分类展示均通过 TypeScript 检查。
- `tools/package-release.ps1`：通过，三类 Windows zip 已生成。
- Client 分包 smoke：通过，`bin/client-agent-core.exe --run-once` 输出 `release_version = v1.28.0`、`arch = x86`、Lua 启用、脚本安全关闭和 `dm.access`。
- Server 分包 health smoke：通过，临时端口 `/health` 返回 `{"status":"ok"}`。
- Server 分包命令队列 smoke：通过，`script.deploy_bundle` 可写入并取出命令队列。
- 包内容检查：通过，总包和 Client 分包包含 `DmBridge.dll`、`dm.dll`、`RegDll.dll`；Server 分包不包含这三个 DM 文件。
- EXE 图标 smoke：通过，Client/Server 根 EXE 和 core EXE 均可读取 32x32 关联图标。
- 三类 zip SHA-256：
  - 总包 `d5f29ffd5a95a0fbec4a4b024b0b263fa7fa9c2adbc0cd58a168e1bd51a7a3f6`
  - Server 分包 `45cb5bcac244d7ac10f3d0c2a27f77178463f862467d85577157e0b0a6a949ef`
  - Client 分包 `feb507de5754dfb0330408e773f483308bfb6a8583401ca05843f07b9898121a`

## P37 桌面控制台 UI 体验优化验证
- `npm run build`：通过，Web Admin 版本为 `1.29.0`。
- `cargo test --workspace`：通过，Client 55 项、Server 52 项、shared-types 11 项、launcher 3 项测试全部通过。
- `tools/package-release.ps1`：通过，三类 Windows zip 已生成。
- 浏览器桌面 QA：1280px 视口通过，页面显示总览、顶部当前目标卡片、Client 列表优先布局，页面无横向溢出。
- 浏览器日志页 QA：日志导航可进入，页面显示 `Server 審計` 和远程操作回执入口。
- 浏览器移动 QA：390px 视口通过，移动导航为 3 列两行，中文未竖排，页面无横向溢出。
- 浏览器控制台错误检查：通过，错误数为 0。
- QA 截图：`测试验证/P37-UI-QA/desktop-overview.png`、`测试验证/P37-UI-QA/mobile-logs.png`。
- Client 分包 smoke：通过，`bin/client-agent-core.exe --run-once` 输出 `release_version = v1.29.0`、`arch = x86`、Lua 启用、脚本安全关闭和 `dm.access`。
- Server 分包 smoke：通过，`/health = ok`、内嵌首页 HTTP 200、`script.deploy_bundle` 命令可写入。
- 三类 zip SHA-256：
  - 总包 `e9595b7b823fcec5790a4adc574d9339df851eb8db98b0c1b39f9a93d789f5f9`
  - Server 分包 `52557f87c26a9378db8a8619c99d1c0c68147e3461d4e21d01072f39c622c746`
  - Client 分包 `551c801f05bdaa38d80bc25ae3c32fc0efc4cc85ab5e08159e61a7e3fc47953d`

## P38 原生桌面控制台入口验证
- `cargo fmt --all --check`：通过。
- `cargo test -p desktop-console`：通过，URL 解析和默认绑定转换测试全部通过。
- `cargo test -p management-server tray`：通过，托盘脚本包含 `WoW-Desktop.exe`，不包含 `msedge.exe` 和 `--app=$serverUrl`。
- `cargo test --workspace`：通过，Client 55 项、desktop-console 4 项、Server 53 项、shared-types 11 项、launcher 3 项测试全部通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `npm run build`：通过，Web Admin 版本为 `1.30.0`。
- `tools/package-release.ps1`：修复 Windows PowerShell 5.1 `Path.GetRelativePath` 兼容问题后通过，三类 Windows zip 已生成。
- 包内容检查：完整包和 Server 分包根目录包含 `WoW-Desktop.exe`，Client 分包不包含 Server 桌面控制台。
- Server 分包 smoke：通过，`/health` 返回 HTTP 200，内嵌首页返回 HTTP 200。
- Client 分包 smoke：通过，`bin/client-agent-core.exe --run-once` 输出 `release_version = v1.30.0`、`arch = x86`、Lua 启用、脚本安全关闭和 `dm.access`。
- 桌面壳错误日志 smoke：通过，非法 URL 会写入 `logs/desktop-console-error.log`。
- 三类 zip SHA-256：
  - 总包 `5fdf01203fd979b6bb7f64b7377d43d1f58fd3a3e51e4f515083ed85e1cb0edb`
  - Server 分包 `b14bafffe7747f8457b6cc026c945beb6b57968da463edfc7d6eb4b0300b0742`
  - Client 分包 `6b55d00cd19534cd7733279f69c96aa420df7361b0afc07ddebbeff7b98ed897`

## P38-H01 桌面控制台直启热修复验证
- `cargo fmt --all --check`：通过。
- `cargo test -p desktop-console`：通过，9 项测试全部通过。
- `cargo test --workspace`：通过，Client 55 项、desktop-console 9 项、Server 53 项、shared-types 11 项、launcher 3 项测试全部通过。
- `npm run build`：通过，Web Admin 版本为 `1.30.1`。
- `tools/package-release.ps1`：通过，三类 Windows zip 已生成。
- 直启 smoke：通过，未提前启动 Server 时，从 Server 分包启动 `WoW-Desktop.exe --url http://127.0.0.1:18134`，可自动启动同包内 Server core，`/health` 返回 HTTP 200。
- 三类 zip SHA-256：
  - 总包 `ceaa6d95f968c4928e4d849103584d98f6fc7a19ebada1a3cfbebfd1a339e4f8`
  - Server 分包 `16ded0bc0527faf9b3f0545d5521cb38d8a9313734c025b1277fa82ddacf2cb0`
  - Client 分包 `d8f39ad3a9fdb91ba39dfd1b2cd2d5808d1e63f585fc92d5d42cd8f6507e40d8`

## P39 DM API 测试与上线准入验证
- `cargo fmt --all --check`：通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `cargo test --workspace`：通过，Client 58 项、desktop-console 9 项、Server 53 项、shared-types 11 项、launcher 3 项测试全部通过。
- `npm run build`：通过，Web Admin 版本为 `1.31.0`。
- `tools/package-release.ps1`：通过，三类 Windows zip 已生成。
- Client 分包 DM API 自检 smoke：通过，临时包执行 `scripts/dm_api_selftest.lua` 成功，输出状态包含 `release_version = v1.31.0`、`arch = x86`、`script_security.enabled = false` 和 `dm.access`。
- Client 分包窗口 smoke：通过，临时包执行 `scripts/dm_window_smoke.lua` 成功；未打开 `World of Warcraft` 时不会让 Client monitor 报错中断。
- 包内容检查：通过，Client 分包包含 `dm_api_selftest.lua`、`dm_window_smoke.lua`、`DmBridge.dll`、`dm.dll`、`RegDll.dll`。
- 三类 zip SHA-256：
  - 总包 `4e03b6ac9b65b118a254c4197eda6d35efdfc8900b3f75fceba6a8f0ceac9e95`
  - Server 分包 `644f17810866667238496511e34ea9eed69bb2b0a5e81d762cb4ab61b5005771`
  - Client 分包 `e805bf15521adacac64db4afa9e59086851697f525f13ded2661c53de22d7b41`

## P39-H01 绑定探测热修复验证
- `cargo fmt --all --check`：通过。
- `cargo test -p client-agent shipped_dm_bind_probe_uses_safe_binding_modes`：通过。
- `cargo test --workspace`：通过，Client 59 项、desktop-console 9 项、Server 53 项、shared-types 11 项、launcher 3 项测试全部通过。
- `cargo clippy --workspace -- -D warnings`：通过。
- `npm run build`：通过，Web Admin 版本为 `1.31.1`。
- `tools/package-release.ps1`：通过，三类 Windows zip 已生成。
- Client 分包绑定探测 smoke：通过，临时包执行 `scripts/dm_bind_probe.lua` 成功，输出状态包含 `release_version = v1.31.1`、`arch = x86` 和 `dm.access`。
- 包内容检查：通过，Client 分包包含 `dm_bind_probe.lua`、`dm_api_selftest.lua`、`dm_window_smoke.lua`、`DmBridge.dll`、`dm.dll`、`RegDll.dll`。
- 包内 README 检查：通过，Client 分包 README 显示 `v1.31.1` 和绑定探测说明。
- 三类 zip SHA-256：
  - 总包 `520a28e0d3ecd85eedeff4669fb477735eb8c4962f023edeeab6acb44016513a`
  - Server 分包 `7dd2b0900e2c4d1437765d21196547a26f155ba8f27f146a2c637d515232d5b1`
  - Client 分包 `efaf93cbb15e76d76e78556afff06a2cbcab0196df0f75b6dc545b8012c267aa`
