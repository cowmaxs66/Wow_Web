# Lua 接口表与使用说明

## 使用前提
Client Lua 脚本由 `mlua` 宿主执行。启用脚本安全门时，接口是否可用取决于 manifest 中声明的权限；v1.25.0 起内部测试包默认关闭安全门，接口按本机默认权限开放。v1.27.0 起，Lua 首次调用需要 COM 的 DM 函数时会自动初始化 DmBridge，普通脚本不必手写 `dm.init("")`。

## 权限表
| 权限 | 可用接口 |
|------|----------|
| `host.log` | `log(message)` |
| `config.read` | `get_config(key)` |
| `dm.access` | `dm.*` 全部接口 |

## 全局接口
| 接口 | 参数 | 返回 | 说明 |
|------|------|------|------|
| `log(message)` | `message: string` | 无 | 写入 Client 本机日志和 tracing 日志 |
| `get_config(key)` | `key: string` | `string` | 读取白名单配置值，如 `client.id`、`lua.bootstrap_path` |

## DM 生命周期
| 接口 | 参数 | 返回 | 说明 |
|------|------|------|------|
| `dm.abi_version()` | 无 | `number` | 读取 DmBridge ABI 版本 |
| `dm.init(dm_root)` | `dm_root: string` | `true` | 显式初始化大漠 COM，可传空字符串；普通脚本可省略 |
| `dm.shutdown()` | 无 | `true` | 释放 DmBridge Worker 和大漠 COM |
| `dm.ver()` | 无 | `string` | 读取大漠版本 |
| `dm.set_path(path)` | `path: string` | `true` | 调用大漠 `SetPath` |
| `dm.last_bridge_error()` | 无 | `number` | 读取 Bridge 最近错误码 |
| `dm.last_dm_error()` | 无 | `number` | 读取大漠 `GetLastError` |

## 窗口与后台绑定
| 接口 | 参数 | 返回 | 说明 |
|------|------|------|------|
| `dm.find_window(class, title)` | `string, string` | `number` | 探测窗口，找不到返回 `0`，不会中断脚本 |
| `dm.find_window_try(class, title)` | `string, string` | `table` | 探测窗口，返回 `{ ok=true, hwnd=0, error="窗口不存在..." }` 或 `{ ok=false, hwnd=0, error="..." }` |
| `dm.find_window_required(class, title)` | `string, string` | `number` | 找不到窗口时直接给出明确 Lua 错误 |
| `dm.bind_window(hwnd, display, mouse, keypad, mode)` | `number, string, string, string, number` | `number` | 直接绑定窗口，失败会中断脚本 |
| `dm.bind_window_try(hwnd, display, mouse, keypad, mode)` | 同上 | `table` | 不抛错，返回 `{ ok=true, ret=1 }` 或 `{ ok=false, error="..." }` |
| `dm.safe_bind_window(hwnd, display, mouse, keypad, mode)` | 同上 | `boolean, string` | 不抛错，返回 `true, ""` 或 `false, error` |
| `dm.unbind_window()` | 无 | `number` | 解绑当前窗口 |
| `dm.with_bound_window(hwnd, display, mouse, keypad, mode, fn)` | 绑定参数 + 回调函数 | 回调返回值 | 自动绑定、执行回调、最后解绑 |

### 推荐绑定写法
```lua
local hwnd = dm.find_window("Qt51514QWindowIcon", "微信")
if hwnd <= 0 then
  log("window not found")
  return "window not found"
end

local ok, err = dm.safe_bind_window(hwnd, "normal", "windows", "windows", 0)
if not ok then
  log("bind failed: " .. err)
  return "bind failed"
end

local color = dm.get_color(10, 10)
dm.unbind_window()
return "dm color=" .. color
```

`"dx"` 对部分窗口或当前显卡/后台环境可能触发大漠内部异常。遇到 `Access violation` 时，先用 `"normal"` 或 `"gdi"` 验证，再逐步切换后台模式。

## 图色与键鼠
| 接口 | 参数 | 返回 | 说明 |
|------|------|------|------|
| `dm.get_color(x, y)` | `number, number` | `string` | 返回 6 位颜色字符串 |
| `dm.get_color_rgb(x, y)` | `number, number` | `table` | 返回 `{ hex="RRGGBB", r=0, g=0, b=0 }` |
| `dm.wait_color(x, y, color, timeout_ms, interval_ms)` | `number, number, string, number, number` | `boolean, string` | 等待指定坐标颜色变成目标值 |
| `dm.move_to(x, y)` | `number, number` | `number` | 鼠标移动 |
| `dm.left_click()` | 无 | `number` | 鼠标左键点击 |

`dm.left_click()` 会真实点击当前鼠标位置或绑定窗口坐标，上线前测试时只允许在安全测试窗口中执行，不放入默认自检脚本。

## 辅助接口
| 接口 | 参数 | 返回 | 说明 |
|------|------|------|------|
| `dm.sleep_ms(ms)` | `number` | `true` | 阻塞等待，最大 60000ms |
| `dm.now_ms()` | 无 | `number` | 当前 Unix 毫秒时间 |

## 上线前接口测试脚本
| 脚本 | 依赖窗口 | 测试范围 | 通过标准 |
|------|----------|----------|----------|
| `scripts/dm_api_selftest.lua` | 否 | `log`、`get_config`、生命周期、版本、路径、错误码、取色、RGB、等待颜色、窗口探测、无效绑定、安全等待和时间戳 | 返回以 `dm_api_selftest|` 开头的摘要 |
| `scripts/dm_window_smoke.lua` | 是，默认标题 `World of Warcraft` | 窗口查找、`safe_bind_window`、绑定后取色、解绑 | 找到窗口时返回 `window=found`；没开窗口时返回 `window=not_found`，不算接口失败 |
| `scripts/dm_smoke.lua` | 否 | 兼容旧版基础 smoke：ABI、初始化、版本、取色、错误码、关闭 | 返回以 `dm_smoke|` 开头的摘要 |

### 为什么不能把“窗口未找到”算接口失败
`FindWindow not found: class= title=World of Warcraft` 表示当前桌面没有匹配标题的窗口。它是测试前置条件缺失，不是 DmBridge 或 Lua API 崩溃。需要验证接口完整性时先跑 `dm_api_selftest.lua`；需要验证 WoW 绑定时再打开游戏窗口运行 `dm_window_smoke.lua`。

## BindWindow 报错排查
如果看到类似：

```text
dm_bridge_bind_window，状态码 -3，消息：Access violation ...
```

按顺序检查：

1. `hwnd` 是否仍有效，目标窗口是否被关闭或重建。
2. 查找窗口先用 `dm.find_window` 或 `dm.find_window_try`，上线脚本确认必要窗口存在后才使用 `dm.find_window_required`。
3. 先用 `dm.safe_bind_window`，不要直接用 `dm.bind_window`。
4. 先测试 `"normal"` 或 `"gdi"`，再测试 `"dx"`。
5. 确认 Client 使用 x86 分包，并且 `dm-bridge/Win32/DmBridge.dll`、`dm.dll`、`RegDll.dll` 都存在。
6. 确认目标机器已注册并授权大漠 COM。
