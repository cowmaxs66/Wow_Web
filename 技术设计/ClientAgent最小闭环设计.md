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
| `src/script/` | Lua 文件读取和脚本路径解析 |
| `src/lua_host.rs` | Lua Runtime 初始化、白名单 API 注册、指令上限和脚本执行 |
| `src/lua_dm.rs` | Lua `dm` 高层 API 注册 |
| `src/dm_bridge/` | Rust DmBridge 安全封装和 UTF-16 buffer 工具 |
| `src/status.rs` | Lua 执行报告到 Client 状态的映射 |
| `src/logging.rs` | 本地结构化日志初始化 |
| `config/client-agent.toml` | 开发期配置样例 |
| `scripts/bootstrap.lua` | 开发期 bootstrap 脚本 |

## 白名单 API
| API | 输入 | 输出 | 说明 |
|-----|------|------|------|
| `log(message)` | 字符串 | 无 | 输出可追踪 Lua 日志 |
| `get_config(key)` | 字符串 key | 字符串或 nil | 只允许读取明确白名单键 |
| `dm.abi_version()` | 无 | 整数 | 读取 DmBridge ABI 版本 |
| `dm.init(dm_root)` | 字符串 | true 或错误 | 初始化 DmBridge 和大漠 COM |
| `dm.ver()` | 无 | 字符串 | 读取大漠版本 |
| `dm.get_color(x, y)` | 整数坐标 | RGB 字符串 | 读取屏幕或绑定窗口颜色 |
| `dm.move_to(x, y)` | 整数坐标 | 大漠返回值 | 鼠标移动 |

## 当前边界
- 不允许 Lua 直接操作文件、进程或系统命令。
- 不在配置文件承载业务脚本，配置只保存脚本名称、路径和运行限制。
- Lua 宿主已设置指令上限，用于拦截开发期无限循环脚本。
- Lua 只能调用 Rust 注册的 `dm` 高层 API，不能直接访问 DmBridge C ABI。
- 当前不接入 WebSocket、不做热重载。
