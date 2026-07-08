# DM 绑定诊断与桌面控制台设计

## 阶段目标
P34 解决实机反馈的 `dm_bridge_bind_window` Access violation 提示，并把管理端体验从浏览器标签页推进到桌面控制台形态，同时补 Client 日志 UI 和 Lua 常用接口说明。

目标版本：`v1.26.0`。

## 问题定位
用户脚本在执行：

```lua
dm.bind_window(hwnd, "dx", "windows", "windows", 0)
```

时报错：

```text
DmBridge 调用失败：dm_bridge_bind_window，状态码 -3，消息：Access violation ...
```

该错误来自大漠 COM 内部异常，Bridge 已捕获并返回 `DM_BRIDGE_COM_ERROR = -3`。常见原因：

- `FindWindow` 返回的窗口句柄已失效，目标窗口重建或关闭。
- `"dx"` 后台显示模式与当前窗口、显卡、权限或桌面环境不兼容。
- 目标机器大漠 COM 注册或授权状态异常。
- 直接使用 `dm.bind_window` 会把绑定失败作为 Lua runtime error 抛出，导致 monitor 本轮失败。

## 修复方案
| 模块 | 处理 |
|------|------|
| DmBridge Delphi | `BindWindow` 前校验 `hwnd > 0` 和 `IsWindow(hwnd)` |
| DmBridge Delphi | 绑定失败时记录 `hwnd/display/mouse/keypad/mode/last_dm_error` |
| DmBridge Delphi | `FindWindow` 找不到窗口时写入 class/title 诊断消息 |
| Rust DmBridge | `bind_window` 前校验 hwnd 和模式字符串 |
| Lua API | 新增 `dm.bind_window_try`、`dm.safe_bind_window`、`dm.with_bound_window` |
| Lua API | 新增 `dm.get_color_rgb`、`dm.wait_color`、`dm.sleep_ms`、`dm.now_ms` |
| 文档 | 新增 Lua 接口表与 BindWindow 排查流程 |

## 推荐脚本模式
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

实机测试时先用 `"normal"` 或 `"gdi"`，稳定后再尝试 `"dx"`。

## 桌面控制台
Server 托盘默认不再打开浏览器标签页，改为：

- 优先使用 Microsoft Edge `--app=<serverUrl>` 打开桌面应用窗口。
- 窗口无浏览器标签栏，表现更接近 Clash Verge。
- 托盘仍保留“浏览器打开 Web 管理页”作为排错入口。
- Edge App 模式由系统浏览器内核处理缩放，比 WinForms `WebBrowser` 控件更可靠。

## Client 日志 UI
Client 新增 `--log-window`：

- 托盘菜单新增“查看日志窗口”。
- WinForms 日志窗口按 DPI 缩放。
- 默认读取 `logs/client-agent.log` 最近 800 行。
- 每 3 秒自动刷新，并提供手动刷新和打开日志目录。

## 验证标准
- `cargo fmt --all --check` 通过。
- `cargo test --workspace` 通过。
- `cargo clippy --workspace -- -D warnings` 通过。
- `npm run build` 通过。
- `实现模块/dm-bridge/build.ps1` 通过。
- 发布包生成后，Client 分包可显示 v1.26.0，并包含新的 DmBridge.dll。
