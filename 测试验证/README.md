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
