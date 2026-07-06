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
