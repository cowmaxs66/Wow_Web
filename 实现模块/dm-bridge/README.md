# dm-bridge 说明

## 职责
大漠插件桥接层，后续使用 Delphi DLL 通过 C ABI 暴露稳定函数给 Rust 调用。

## 当前状态
P2 已完成 DmBridge 最小桥接链路，Win32 DLL 可由 32 位 Rust `libloading` 加载，并已通过 Rust/Lua COM 烟测。

当前只实现 P2 烟测链路：
- `dm_bridge_abi_version`
- `dm_bridge_init`
- `dm_bridge_shutdown`
- `dm_bridge_get_last_bridge_error`
- `dm_bridge_get_last_dm_error`
- `dm_bridge_get_last_message`
- `dm_bridge_ver`
- `dm_bridge_set_path`
- `dm_bridge_find_window`
- `dm_bridge_bind_window`
- `dm_bridge_get_color`
- `dm_bridge_move_to`
- `dm_bridge_left_click`
- `dm_bridge_unbind_window`

## 编译命令
```powershell
.\实现模块\dm-bridge\build.ps1
```

输出文件位于：

```text
target/dm-bridge/Win32/DmBridge.dll
```

该输出目录已被 `.gitignore` 排除，不进入 GitHub。

## P2 前必须明确
- 字符串编码。
- 内存分配与释放归属。
- COM 初始化和释放规则。
- STA 单线程调用队列。
- 错误码与错误消息结构。

## 计划目录排版
后续 Delphi 工程必须按职责拆分，禁止把所有导出函数写在一个文件中。

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
| `DmBridge.Worker.pas` | Worker 对外门面，负责初始化、调用和释放 |
| `DmBridge.Api.Common.pas` | 导出函数共用小工具 |
| `DmBridge.Api.Abi.pas` | ABI 版本导出函数 |
| `DmBridge.Api.Lifecycle.pas` | 初始化、释放、错误查询导出函数 |
| `DmBridge.Api.Basic.pas` | 版本、路径等基本导出函数 |
| `DmBridge.Api.Window.pas` | 窗口查找导出函数 |
| `DmBridge.Api.Bind.pas` | 后台绑定和解绑导出函数 |
| `DmBridge.Api.Color.pas` | 图色导出函数 |
| `DmBridge.Api.Input.pas` | 键鼠导出函数 |

## 当前限制
- 当前已允许外部线程调用导出函数，但 DLL 内部仍串行投递到单一 STA 线程执行大漠 COM。
- 当前不实现大漠全量接口，只实现烟测链路。
- 自动烟测验证 `MoveTo`，不自动执行 `LeftClick`，避免误点击。
- 当前不复制 `dm.dll`、`RegDll.dll`、CHM、授权资料到仓库。

## 契约文档
详见：`技术设计/DmBridge_C_ABI契约设计.md`
