# DM 正式包与多选客户端操作设计

## 阶段范围

本设计覆盖 P32：

- Client 默认配置开启 `dm.access`。
- 总包和 Client 分包随包携带 `dm.dll` 与 `RegDll.dll`。
- Web Admin 远程操作目标从下拉选择改为可勾选 Client 列表。
- 保持源码仓库不提交大漠二进制和授权资料。

## 设计结论

- `script_security.allowed_permissions` 默认包含 `host.log`、`config.read` 和 `dm.access`。
- 默认 `bootstrap.lua` 仍只请求 `host.log/config.read`，真正使用 DM 的 Lua 脚本必须在自己的 manifest 中声明 `dm.access`。
- 打包脚本默认从 `I:\图色\工具\大漠\7.2149` 复制 `dm.dll` 和 `RegDll.dll`，也可用 `WOW_DM_RUNTIME_DIR` 覆盖来源目录。
- DM 运行 DLL 只进入 release 包的 `dm-bridge/Win32/`，不进入 Git 源码。
- Server 分包继续不包含 Client、脚本、DmBridge 和 DM 运行 DLL。
- 远程操作页用复选框控制具体下发 Client，避免“全部客户端”下拉项不直观。

## DM 文件打包边界

进入总包和 Client 分包：

```text
dm-bridge/Win32/DmBridge.dll
dm-bridge/Win32/dm.dll
dm-bridge/Win32/RegDll.dll
```

不进入任何 Git 提交：

- `dm.dll`
- `RegDll.dll`
- 授权文件
- 账号资料
- CHM/CHW 文档
- 大漠私有压缩包

## Web 多选操作

远程操作页的目标选择改为列表框：

- 每台 Client 独立复选框。
- 支持全选和清空。
- 选中 1 台时显示该 Client 最近回执。
- 选中多台时批量写入各自队列，执行结果以每台 Client 的回执为准。

## 风险边界

- Release 包携带 DM DLL 后，不再适合公开分发给无授权用户。
- DM COM 注册和授权状态仍由使用机器负责，本阶段只保证文件随包携带。
- 生产联网前仍必须补登录鉴权、角色权限、操作者审计和审批流。
