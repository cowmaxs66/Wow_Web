# DmBridge C ABI 契约设计

## 目标
`DmBridge.dll` 负责把大漠 COM 接口收敛成稳定 C ABI，供 Rust 通过 `libloading` 调用。

本契约只覆盖 P2 第一版桥接范围：

```text
Rust client-agent -> libloading -> DmBridge.dll -> dm.dll COM -> 大漠能力
```

## 不做事项
- 不导出大漠全量 830 个 Delphi 方法。
- 不暴露内存、汇编、防护盾、变速、降 CPU 等高风险接口。
- 不让 Lua 直接调用 DmBridge C ABI。
- 不跨 DLL 分配/释放字符串内存。
- 不把授权码、注册码、真实脚本配置写入仓库。

## ABI 基本规则
| 项目 | 规则 |
|------|------|
| 调用约定 | Windows 下统一 `stdcall` |
| 函数前缀 | `dm_bridge_` |
| 整数类型 | `i32` 对应 Delphi `Integer`，Rust `i32` |
| 浮点类型 | `f64` 对应 Delphi `Double`，Rust `f64` |
| 字符串输入 | UTF-16 NUL 结尾，C 类型为 `const wchar_t*`，Delphi 为 `PWideChar` |
| 字符串输出 | 调用方提供 UTF-16 buffer，DLL 只写入，不分配 |
| 布尔值 | 使用 `i32`，`0 = false`，`1 = true` |
| 句柄 | 使用 `i32` 保存大漠接口中的 `hwnd` |
| 坐标 | 使用 `i32` |

## 返回值规则
所有导出函数返回 Bridge 状态码，不直接等同于大漠原始返回值。

| 状态码 | 名称 | 含义 |
|--------|------|------|
| `1` | `DM_BRIDGE_OK` | Bridge 调用成功，已执行大漠方法 |
| `0` | `DM_BRIDGE_DM_FAILED` | 大漠方法执行了，但返回失败 |
| `-1` | `DM_BRIDGE_NOT_INITIALIZED` | 未初始化 |
| `-2` | `DM_BRIDGE_INVALID_ARG` | 参数为空、长度不合法、坐标范围错误 |
| `-3` | `DM_BRIDGE_COM_ERROR` | COM 创建或调用异常 |
| `-4` | `DM_BRIDGE_BUFFER_TOO_SMALL` | 调用方字符串 buffer 不够 |
| `-5` | `DM_BRIDGE_THREAD_ERROR` | 调用线程不符合 STA 规则 |
| `-6` | `DM_BRIDGE_UNSUPPORTED` | 当前 ABI 未开放该能力 |

## 结果结构
Rust 侧使用 `#[repr(C)]` 对齐。Delphi 侧使用 `packed record` 或明确字段顺序。

```c
typedef struct DmBridgePoint {
    int32_t x;
    int32_t y;
} DmBridgePoint;

typedef struct DmBridgeRect {
    int32_t x1;
    int32_t y1;
    int32_t x2;
    int32_t y2;
} DmBridgeRect;

typedef struct DmBridgeSize {
    int32_t width;
    int32_t height;
} DmBridgeSize;

typedef struct DmBridgeFindResult {
    int32_t dm_ret;
    int32_t x;
    int32_t y;
} DmBridgeFindResult;
```

## 字符串输出规则
所有字符串输出统一使用：

```c
int32_t dm_bridge_xxx(
    wchar_t* out_buf,
    uint32_t out_capacity,
    uint32_t* out_len
);
```

规则：
- `out_capacity` 表示 `wchar_t` 数量，不是字节数。
- `out_len` 返回实际需要的字符数，不含结尾 NUL。
- 如果 buffer 不够，返回 `DM_BRIDGE_BUFFER_TOO_SMALL`，同时写入 `out_len`。
- 如果输出为空字符串，`out_len = 0`，buffer 写入 NUL。
- Rust 侧负责分配 buffer 并重试。

## 线程模型
P2 第一版采用 **Bridge 内部 STA Worker 模式**：

1. `dm_bridge_init` 创建一个专用 STA 线程。
2. STA 线程中执行 COM 初始化，并创建 `dm.dmsoft` 对象。
3. 所有导出函数把请求投递到 STA 线程，同步等待结果。
4. `dm_bridge_shutdown` 释放 COM 对象并退出 STA 线程。

这样 Rust 可以从任意线程调用 DmBridge，但 Bridge 内部保证所有大漠 COM 调用都在同一个 STA 线程完成。

## 生命周期函数
| C ABI | 用途 |
|-------|------|
| `int32_t dm_bridge_abi_version(void)` | 返回 ABI 版本，当前固定返回 `1` |
| `int32_t dm_bridge_init(const wchar_t* dm_root)` | 初始化 Bridge、STA 线程和大漠 COM 对象 |
| `int32_t dm_bridge_shutdown(void)` | 释放大漠对象并停止 STA 线程 |
| `int32_t dm_bridge_get_last_bridge_error(void)` | 获取 Bridge 内部错误码 |
| `int32_t dm_bridge_get_last_dm_error(int32_t* out_error)` | 调用大漠 `GetLastError` |
| `int32_t dm_bridge_get_last_message(wchar_t* out_buf, uint32_t out_capacity, uint32_t* out_len)` | 获取 Bridge 内部错误消息 |

## 基本设置导出
| C ABI | 对应大漠接口 | 说明 |
|-------|--------------|------|
| `dm_bridge_ver(out_buf, cap, out_len)` | `Ver` | 获取版本，第一烟测 |
| `dm_bridge_set_path(path)` | `SetPath` | 设置图片资源目录 |
| `dm_bridge_get_path(out_buf, cap, out_len)` | `GetPath` | 获取当前资源目录 |
| `dm_bridge_reg(code, ver)` | `Reg` | 授权注册，参数只能来自私有配置 |
| `dm_bridge_reg_ex(code, ver, ip)` | `RegEx` | 扩展注册，参数只能来自私有配置 |
| `dm_bridge_enable_pic_cache(enable)` | `EnablePicCache` | 图片缓存 |
| `dm_bridge_set_enum_window_delay(delay_ms)` | `SetEnumWindowDelay` | 枚举窗口延迟 |
| `dm_bridge_set_show_error_msg(show)` | `SetShowErrorMsg` | 调试期错误弹窗控制 |

## 窗口导出
| C ABI | 对应大漠接口 | 输出 |
|-------|--------------|------|
| `dm_bridge_find_window(class_name, title_name, out_hwnd)` | `FindWindow` | `hwnd` |
| `dm_bridge_find_window_ex(parent, class_name, title_name, out_hwnd)` | `FindWindowEx` | `hwnd` |
| `dm_bridge_enum_window(parent, title, class_name, filter, out_buf, cap, out_len)` | `EnumWindow` | 窗口列表字符串 |
| `dm_bridge_get_window_title(hwnd, out_buf, cap, out_len)` | `GetWindowTitle` | 标题 |
| `dm_bridge_get_window_class(hwnd, out_buf, cap, out_len)` | `GetWindowClass` | 类名 |
| `dm_bridge_get_window_rect(hwnd, out_rect)` | `GetWindowRect` | `DmBridgeRect` |
| `dm_bridge_get_client_rect(hwnd, out_rect)` | `GetClientRect` | `DmBridgeRect` |
| `dm_bridge_get_client_size(hwnd, out_size)` | `GetClientSize` | `DmBridgeSize` |
| `dm_bridge_client_to_screen(hwnd, inout_point)` | `ClientToScreen` | 修改坐标 |
| `dm_bridge_screen_to_client(hwnd, inout_point)` | `ScreenToClient` | 修改坐标 |
| `dm_bridge_get_foreground_window(out_hwnd)` | `GetForegroundWindow` | `hwnd` |
| `dm_bridge_get_point_window(x, y, out_hwnd)` | `GetPointWindow` | `hwnd` |
| `dm_bridge_set_window_state(hwnd, flag, out_dm_ret)` | `SetWindowState` | 大漠返回值 |

## 后台设置导出
| C ABI | 对应大漠接口 | 说明 |
|-------|--------------|------|
| `dm_bridge_bind_window(hwnd, display, mouse, keypad, mode, out_dm_ret)` | `BindWindow` | 第一优先 |
| `dm_bridge_bind_window_ex(hwnd, display, mouse, keypad, public_desc, mode, out_dm_ret)` | `BindWindowEx` | 第二步验证 |
| `dm_bridge_unbind_window(out_dm_ret)` | `UnBindWindow` | 必需清理 |
| `dm_bridge_is_bind(hwnd, out_dm_ret)` | `IsBind` | 绑定状态 |
| `dm_bridge_get_bind_window(out_hwnd)` | `GetBindWindow` | 当前绑定窗口 |
| `dm_bridge_switch_bind_window(hwnd, out_dm_ret)` | `SwitchBindWindow` | 多窗口预留 |
| `dm_bridge_lock_display(lock, out_dm_ret)` | `LockDisplay` | 后台稳定性 |
| `dm_bridge_lock_input(lock, out_dm_ret)` | `LockInput` | 谨慎使用 |
| `dm_bridge_lock_mouse_rect(x1, y1, x2, y2, out_dm_ret)` | `LockMouseRect` | 防误操作 |
| `dm_bridge_enable_fake_active(enable, out_dm_ret)` | `EnableFakeActive` | 后台兼容 |
| `dm_bridge_enable_ime(enable, out_dm_ret)` | `EnableIme` | 输入法兼容 |
| `dm_bridge_enable_mouse_sync(enable, timeout_ms, out_dm_ret)` | `EnableMouseSync` | 鼠标同步 |
| `dm_bridge_enable_keypad_sync(enable, timeout_ms, out_dm_ret)` | `EnableKeypadSync` | 键盘同步 |
| `dm_bridge_set_display_delay(delay_ms, out_dm_ret)` | `SetDisplayDelay` | 图色稳定性 |
| `dm_bridge_set_input_dm(input_dm, rx, ry, out_dm_ret)` | `SetInputDm` | 输入模式 |

## 算法导出
算法接口只处理大漠返回的坐标字符串，不触碰窗口或输入。

| C ABI | 对应大漠接口 | 输出 |
|-------|--------------|------|
| `dm_bridge_exclude_pos(all_pos, type, x1, y1, x2, y2, out_buf, cap, out_len)` | `ExcludePos` | 坐标字符串 |
| `dm_bridge_find_nearest_pos(all_pos, type, x, y, out_buf, cap, out_len)` | `FindNearestPos` | 坐标字符串 |
| `dm_bridge_sort_pos_distance(all_pos, type, x, y, out_buf, cap, out_len)` | `SortPosDistance` | 坐标字符串 |

## 图色导出
| C ABI | 对应大漠接口 | 输出 |
|-------|--------------|------|
| `dm_bridge_capture(x1, y1, x2, y2, file_name, out_dm_ret)` | `Capture` | 大漠返回值 |
| `dm_bridge_capture_png(x1, y1, x2, y2, file_name, out_dm_ret)` | `CapturePng` | 大漠返回值 |
| `dm_bridge_get_color(x, y, out_buf, cap, out_len)` | `GetColor` | RGB 字符串 |
| `dm_bridge_get_color_bgr(x, y, out_buf, cap, out_len)` | `GetColorBGR` | BGR 字符串 |
| `dm_bridge_cmp_color(x, y, color, sim, out_dm_ret)` | `CmpColor` | 大漠返回值 |
| `dm_bridge_find_color(x1, y1, x2, y2, color, sim, dir, out_result)` | `FindColor` | `DmBridgeFindResult` |
| `dm_bridge_find_color_ex(x1, y1, x2, y2, color, sim, dir, out_buf, cap, out_len)` | `FindColorEx` | 多坐标字符串 |
| `dm_bridge_find_multi_color(x1, y1, x2, y2, first_color, offset_color, sim, dir, out_result)` | `FindMultiColor` | `DmBridgeFindResult` |
| `dm_bridge_find_multi_color_ex(x1, y1, x2, y2, first_color, offset_color, sim, dir, out_buf, cap, out_len)` | `FindMultiColorEx` | 多坐标字符串 |
| `dm_bridge_find_pic(x1, y1, x2, y2, pic_name, delta_color, sim, dir, out_result)` | `FindPic` | `DmBridgeFindResult` |
| `dm_bridge_find_pic_ex(x1, y1, x2, y2, pic_name, delta_color, sim, dir, out_buf, cap, out_len)` | `FindPicEx` | 多坐标字符串 |
| `dm_bridge_get_pic_size(pic_name, out_size)` | `GetPicSize` | `DmBridgeSize` |
| `dm_bridge_load_pic(pic_name, out_dm_ret)` | `LoadPic` | 大漠返回值 |
| `dm_bridge_free_pic(pic_name, out_dm_ret)` | `FreePic` | 大漠返回值 |
| `dm_bridge_set_pic_pwd(pwd, out_dm_ret)` | `SetPicPwd` | 大漠返回值 |
| `dm_bridge_enable_get_color_by_capture(enable, out_dm_ret)` | `EnableGetColorByCapture` | 大漠返回值 |
| `dm_bridge_enable_find_pic_multithread(enable, out_dm_ret)` | `EnableFindPicMultithread` | 大漠返回值 |
| `dm_bridge_set_find_pic_multithread_count(count, out_dm_ret)` | `SetFindPicMultithreadCount` | 大漠返回值 |

## 键鼠导出
| C ABI | 对应大漠接口 | 说明 |
|-------|--------------|------|
| `dm_bridge_move_to(x, y, out_dm_ret)` | `MoveTo` | 移动鼠标 |
| `dm_bridge_move_r(rx, ry, out_dm_ret)` | `MoveR` | 相对移动 |
| `dm_bridge_move_to_ex(x, y, w, h, out_buf, cap, out_len)` | `MoveToEx` | 返回随机点字符串 |
| `dm_bridge_left_click(out_dm_ret)` | `LeftClick` | 左键单击 |
| `dm_bridge_left_double_click(out_dm_ret)` | `LeftDoubleClick` | 左键双击 |
| `dm_bridge_left_down(out_dm_ret)` | `LeftDown` | 左键按下 |
| `dm_bridge_left_up(out_dm_ret)` | `LeftUp` | 左键抬起 |
| `dm_bridge_right_click(out_dm_ret)` | `RightClick` | 右键单击 |
| `dm_bridge_right_down(out_dm_ret)` | `RightDown` | 右键按下 |
| `dm_bridge_right_up(out_dm_ret)` | `RightUp` | 右键抬起 |
| `dm_bridge_wheel_up(out_dm_ret)` | `WheelUp` | 滚轮向上 |
| `dm_bridge_wheel_down(out_dm_ret)` | `WheelDown` | 滚轮向下 |
| `dm_bridge_key_press(vk, out_dm_ret)` | `KeyPress` | 按键 |
| `dm_bridge_key_down(vk, out_dm_ret)` | `KeyDown` | 按下 |
| `dm_bridge_key_up(vk, out_dm_ret)` | `KeyUp` | 抬起 |
| `dm_bridge_key_press_char(key_str, out_dm_ret)` | `KeyPressChar` | 字符按键 |
| `dm_bridge_key_press_str(key_str, delay_ms, out_dm_ret)` | `KeyPressStr` | 字符串按键 |
| `dm_bridge_get_cursor_pos(out_point)` | `GetCursorPos` | 鼠标位置 |
| `dm_bridge_get_key_state(vk, out_state)` | `GetKeyState` | 按键状态 |
| `dm_bridge_set_mouse_delay(type, delay_ms, out_dm_ret)` | `SetMouseDelay` | 鼠标延迟 |
| `dm_bridge_set_keypad_delay(type, delay_ms, out_dm_ret)` | `SetKeypadDelay` | 键盘延迟 |
| `dm_bridge_set_mouse_speed(speed, out_dm_ret)` | `SetMouseSpeed` | 鼠标速度 |
| `dm_bridge_wait_key(vk, timeout_ms, out_dm_ret)` | `WaitKey` | 等待按键 |

## P2 最小烟测链路
第一轮实现只需要跑通这条链路：

```text
dm_bridge_abi_version
dm_bridge_init
dm_bridge_ver
dm_bridge_set_path
dm_bridge_find_window
dm_bridge_bind_window
dm_bridge_get_color
dm_bridge_move_to
dm_bridge_left_click
dm_bridge_unbind_window
dm_bridge_shutdown
```

验收标准：
- 不崩溃。
- 能返回大漠版本。
- 能设置资源路径。
- 能定位窗口。
- 能绑定和解绑窗口。
- 能取色。
- 能执行一次鼠标移动和点击。
- 失败时 Rust 能拿到 Bridge 错误和大漠 `GetLastError`。

## P2-S03 实现备注
P2-S03 已建立最小 Delphi DLL 工程。该阶段实现为“直接 STA 最小模式”：

- `dm_bridge_init` 在线程内执行 COM 初始化并创建 `dm.dmsoft`。
- 后续导出函数要求在同一线程调用。
- 如果跨线程调用，返回 `DM_BRIDGE_THREAD_ERROR`。
- 该模式只用于验证 Delphi 工程、导出函数、UTF-16 buffer、COM 创建和最小链路。

## P2-S04 实现备注
P2-S04 已升级为内部 STA Worker 队列：

- `dm_bridge_init` 创建 `TDmWorkerThread`。
- `TDmWorkerThread` 内部执行 `CoInitializeEx(nil, COINIT_APARTMENTTHREADED)`。
- `TDmWorkerThread` 内部创建并持有 `TDmsoftHost`。
- 所有导出函数通过 `WorkerInvoke` 同步投递请求。
- `dm_bridge_shutdown` 停止 Worker 并释放 COM 对象。
- 导出函数名称和 ABI 参数未变化。

## P2-S05 至 P2-S07 实现备注
P2 Rust 与 Lua 链路已完成最小闭环：

- Rust `src/dm_bridge/` 使用 `libloading` 加载 Win32 `DmBridge.dll`。
- Rust 加载前读取 PE Machine 字段，64 位进程加载 32 位 DLL 时会返回明确错误。
- Rust 统一使用 UTF-16 NUL 输入和调用方 buffer 输出。
- Lua 通过 `src/lua_dm.rs` 暴露 `dm` 高层 API，不暴露 C ABI 指针。
- 已验证 `dm_bridge_abi_version`、`init`、`ver`、`get_color`、`move_to`、`shutdown`。
- 自动烟测不执行 `left_click`，避免误点击；点击能力只完成导出和封装。

## P2 修复记录
- Delphi 标识符不区分大小写，内部常量不能命名为 `DM_BRIDGE_ABI_VERSION`，否则会与导出函数 `dm_bridge_abi_version` 冲突并递归调用自身。
- `TDmWorkerThread.Start` 必须在对象构造完成后由 `WorkerInit` 调用，不能在构造函数内部调用。
- 大漠无参 COM 方法在 Delphi `OleVariant` 下使用 `FObj.Ver` 形式，不能写成 `FObj.Ver()`。

## Delphi 工程目录排版建议
P2-S03 实现时按以下职责拆分，禁止把代码堆在单个 `.pas` 文件：

| 文件 | 职责 |
|------|------|
| `DmBridge.dpr` | DLL 工程入口和 exports |
| `DmBridge.Types.pas` | C ABI record、常量、状态码 |
| `DmBridge.Strings.pas` | UTF-16 输入输出和 buffer 写入 |
| `DmBridge.Errors.pas` | Bridge 错误码和错误消息 |
| `DmBridge.Dmsoft.pas` | 大漠 COM 对象封装 |
| `DmBridge.Worker.Types.pas` | Worker 调用闭包类型 |
| `DmBridge.Worker.Request.pas` | 同步请求对象和完成事件 |
| `DmBridge.Worker.Thread.pas` | STA 线程、COM 初始化和请求队列 |
| `DmBridge.Worker.pas` | Worker 对外门面 |
| `DmBridge.Api.Abi.pas` | ABI 版本导出实现 |
| `DmBridge.Api.Basic.pas` | 基本设置导出实现 |
| `DmBridge.Api.Window.pas` | 窗口导出实现 |
| `DmBridge.Api.Bind.pas` | 后台绑定导出实现 |
| `DmBridge.Api.Color.pas` | 图色导出实现 |
| `DmBridge.Api.Input.pas` | 键鼠导出实现 |
| `DmBridge.Api.Algorithm.pas` | 算法导出实现 |

## Rust 侧目录排版建议
P2-S04 实现时按以下职责拆分：

| 文件 | 职责 |
|------|------|
| `src/dm_bridge/mod.rs` | 对外安全封装入口 |
| `src/dm_bridge/ffi.rs` | `libloading` 原始函数指针和 `repr(C)` 类型 |
| `src/dm_bridge/error.rs` | Bridge 错误类型 |
| `src/dm_bridge/strings.rs` | UTF-16 buffer 调用工具 |
| `src/dm_bridge/path.rs` | DmBridge 路径解析和 PE 位数检查 |
| `src/dm_bridge/window.rs` | 窗口安全封装 |
| `src/dm_bridge/bind.rs` | 后台绑定安全封装 |
| `src/dm_bridge/color.rs` | 图色安全封装 |
| `src/dm_bridge/input.rs` | 键鼠安全封装 |
| `src/dm_bridge/algorithm.rs` | 算法安全封装 |

## 公开仓库注意
当前 GitHub 仓库为 public。以下内容禁止提交：

- `dm.dll`
- `RegDll.dll`
- `*.chm`
- `*.chw`
- 大漠注册码
- 真实窗口标题、账号、脚本配置
- 真实游戏或业务图片资源
