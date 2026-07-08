# Client Lua 脚本说明

## 文件职责
| 文件 | 职责 |
|------|------|
| `bootstrap.lua` | 默认安全启动脚本，只验证 Lua 宿主和配置读取 |
| `bootstrap.manifest.json` | 默认脚本 manifest、hash、权限和签名 |
| `dm_api_selftest.lua` | DM/Lua 上线前基础接口自检，不依赖目标窗口，不点击 |
| `dm_window_smoke.lua` | WoW 窗口实机 smoke，找不到窗口时返回 `window=not_found`，不让 monitor 失败 |
| `dm_bind_probe.lua` | 绑定模式探测脚本，默认测试微信窗口的常见低风险绑定组合 |
| `dm_smoke.lua` | DM 实机烟测脚本，只读取 ABI、版本、屏幕颜色并关闭 Bridge，不点击、不绑定窗口 |
| `dm_smoke.manifest.json` | DM 烟测脚本 manifest，权限限定为 `host.log` 和 `dm.access` |

## Lua 接口说明
完整接口表见：`技术设计/Lua接口表与使用说明.md`。

脚本中推荐优先使用 `dm.find_window` / `dm.find_window_try` 做探测，再使用 `dm.safe_bind_window` 或 `dm.bind_window_try`，避免目标窗口未打开、句柄失效、后台模式不兼容时直接中断整个 Client monitor。

## 上线前测试顺序
1. 先运行 `dm_api_selftest.lua`，确认 DmBridge、dm COM、Lua 全局接口和图色基础接口可用。
2. 再打开目标游戏窗口，运行 `dm_window_smoke.lua`，确认窗口标题、绑定模式和绑定后取色可用。
3. 如果 `safe_bind_window` 失败，再运行 `dm_bind_probe.lua`，确认目标窗口支持哪些绑定组合。
4. 最后再推送业务 Lua。业务 Lua 中只有“目标窗口必须存在”时才使用 `dm.find_window_required`。

## DM 烟测使用方式
在 Client 设置窗口中切换以下字段：
- `Bootstrap 名称`：`dm-smoke`
- `Bootstrap 路径`：`scripts/dm_smoke.lua`
- `Manifest 路径`：`scripts/dm_smoke.manifest.json`
- `Lua 权限`：勾选 `host.log` 和 `dm.access`
- `Ed25519 公钥`：`ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c`

`DmBridge.dll 路径` 使用发布包默认值 `dm-bridge/Win32/DmBridge.dll`。P32 起总包和 Client 分包会随包携带 `dm.dll` 与 `RegDll.dll`；本烟测仍要求本机大漠 COM 已注册并具备授权。

切回默认 `bootstrap.lua` 时，需要把 `Ed25519 公钥` 恢复为默认配置中的 `7f3c8268b3f403594ef6e7e681ca62851e66b07b915c48a523b0cfaf7e54bfc9`，权限保持 `host.log`、`config.read`、`dm.access`。
