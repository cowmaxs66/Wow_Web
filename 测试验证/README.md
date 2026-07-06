# 测试验证说明

本目录保存编译检查、烟测、测试报告和手动验证记录。

## 当前验证项
| 验证项 | 命令 | 状态 |
|--------|------|------|
| Rust workspace 测试 | `cargo test --workspace` | 已通过 |
| Client 状态输出 | `cargo run -p client-agent` | 已通过 |
| Server 契约输出 | `cargo run -p management-server` | 已通过 |

## P0 验证记录
- `cargo test --workspace`：通过，`shared-types` 单元测试 1 项通过。
- `cargo run -p client-agent`：通过，输出 `schema_version/message_id/message_type/client_id/timestamp_ms/data`。
- `cargo run -p management-server`：通过，服务端入口可复用同一份协议类型。

## P1 验证记录
- `cargo test --workspace`：通过，client-agent 4 项测试通过，shared-types 1 项测试通过。
- `cargo run -p client-agent`：通过，读取 TOML 配置和 `scripts/bootstrap.lua`，输出 `current_script = bootstrap`。
- Lua bootstrap：通过，`log("bootstrap started from file")` 可进入 tracing 日志，`get_config("client.id")` 可读取白名单配置。
- Lua 指令上限：通过，`while true do end` 会触发 `Lua 脚本超过指令上限` 错误。
- 代码结构检查：通过，client-agent 当前最大单文件 119 行，入口文件 32 行；最大文件职责仍限定为 Lua 宿主。
