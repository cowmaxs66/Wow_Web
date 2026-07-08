# Lua 接口表与使用说明

## 使用前提
Client Lua 脚本由 `mlua` 宿主执行。启用脚本安全门时，接口是否可用取决于 manifest 中声明的权限；v1.25.0 起内部测试包默认关闭安全门，接口按本机默认权限开放。

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
| `dm.init(dm_root)` | `dm_root: string` | `true` | 初始化大漠 COM，可传空字符串 |
| `dm.shutdown()` | 无 | `true` | 释放 DmBridge Worker 和大漠 COM |
| `dm.ver()` | 无 | `string` | 读取大漠版本 |
| `dm.set_path(path)` | `path: string` | `true` | 调用大漠 `SetPath` |
| `dm.last_bridge_error()` | 无 | `number` | 读取 Bridge 最近错误码 |
| `dm.last_dm_error()` | 无 | `number` | 读取大漠 `GetLastError` |

## 窗口与后台绑定
| 接口 | 参数 | 返回 | 说明 |
|------|------|------|------|
| `dm.find_window(class, title)` | `string, string` | `number` | 调用大漠 `FindWindow`，找不到会报错并说明 class/title |
| `dm.find_window_required(class, title)` | `string, string` | `number` | 找不到窗口时直接给出明确 Lua 错误 |
| `dm.bind_window(hwnd, display, mouse, keypad, mode)` | `number, string, string, string, number` | `number` | 直接绑定窗口，失败会中断脚本 |
| `dm.bind_window_try(hwnd, display, mouse, keypad, mode)` | 同上 | `table` | 不抛错，返回 `{ ok=true, ret=1 }` 或 `{ ok=false, error="..." }` |
| `dm.safe_bind_window(hwnd, display, mouse, keypad, mode)` | 同上 | `boolean, string` | 不抛错，返回 `true, ""` 或 `false, error` |
| `dm.unbind_window()` | 无 | `number` | 解绑当前窗口 |
| `dm.with_bound_window(hwnd, display, mouse, keypad, mode, fn)` | 绑定参数 + 回调函数 | 回调返回值 | 自动绑定、执行回调、最后解绑 |

### 推荐绑定写法
```lua
local hwnd = dm.find_window_required("Qt51514QWindowIcon", "微信")
local ok, err = dm.safe_bind_window(hwnd, "normal", "windows", "windows", 0)
if not ok then
  log("bind failed: " .. err)
  dm.shutdown()
  return "bind failed"
end

local color = dm.get_color(10, 10)
dm.unbind_window()
dm.shutdown()
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

## 辅助接口
| 接口 | 参数 | 返回 | 说明 |
|------|------|------|------|
| `dm.sleep_ms(ms)` | `number` | `true` | 阻塞等待，最大 60000ms |
| `dm.now_ms()` | 无 | `number` | 当前 Unix 毫秒时间 |

## BindWindow 报错排查
如果看到类似：

```text
dm_bridge_bind_window，状态码 -3，消息：Access violation ...
```

按顺序检查：

1. `hwnd` 是否仍有效，目标窗口是否被关闭或重建。
2. 先用 `dm.safe_bind_window`，不要直接用 `dm.bind_window`。
3. 先测试 `"normal"` 或 `"gdi"`，再测试 `"dx"`。
4. 确认 Client 使用 x86 分包，并且 `dm-bridge/Win32/DmBridge.dll`、`dm.dll`、`RegDll.dll` 都存在。
5. 确认目标机器已注册并授权大漠 COM。
