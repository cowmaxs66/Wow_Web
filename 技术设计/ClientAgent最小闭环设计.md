# Client Agent 最小闭环设计

## 目标
P1 阶段先让 Client Agent 在本地形成最小闭环：

```text
读取 TOML 配置 -> 初始化 Lua 宿主 -> 注册白名单 API -> 执行 bootstrap -> 输出状态消息
```

## 当前模块拆分
| 文件或目录 | 职责 |
|------------|------|
| `src/main.rs` | 入口流程串接，不承载配置解析或 Lua 细节 |
| `src/config/` | 配置读取、错误类型、默认路径解析 |
| `src/lua_host.rs` | Lua Runtime 初始化、白名单 API 注册、bootstrap 执行 |
| `config/client-agent.toml` | 开发期配置样例 |

## 白名单 API
| API | 输入 | 输出 | 说明 |
|-----|------|------|------|
| `log(message)` | 字符串 | 无 | 输出可追踪 Lua 日志 |
| `get_config(key)` | 字符串 key | 字符串或 nil | 只允许读取明确白名单键 |

## 当前边界
- 不允许 Lua 直接操作文件、进程或系统命令。
- 不在配置文件长期承载业务脚本。
- 下一步将 bootstrap 迁移到独立 Lua 文件。
- 不接入大漠、不接入 WebSocket、不做热重载。
