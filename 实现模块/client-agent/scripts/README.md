# Client Lua 脚本说明

## 文件职责
| 文件 | 职责 |
|------|------|
| `bootstrap.lua` | 默认安全启动脚本，只验证 Lua 宿主和配置读取 |
| `bootstrap.manifest.json` | 默认脚本 manifest、hash、权限和签名 |
| `dm_smoke.lua` | DM 实机烟测脚本，只读取 ABI、版本、屏幕颜色并关闭 Bridge，不点击、不绑定窗口 |
| `dm_smoke.manifest.json` | DM 烟测脚本 manifest，权限限定为 `host.log` 和 `dm.access` |

## DM 烟测使用方式
在 Client 设置窗口中切换以下字段：
- `Bootstrap 名称`：`dm-smoke`
- `Bootstrap 路径`：`scripts/dm_smoke.lua`
- `Manifest 路径`：`scripts/dm_smoke.manifest.json`
- `Lua 权限`：勾选 `host.log` 和 `dm.access`
- `Ed25519 公钥`：`ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c`

`DmBridge.dll 路径` 使用发布包默认值 `dm-bridge/Win32/DmBridge.dll`。本烟测仍要求本机大漠 COM 已可用；发布包不会包含 `dm.dll`、`RegDll.dll`、授权文件或账号资料。

切回默认 `bootstrap.lua` 时，需要把 `Ed25519 公钥` 恢复为默认配置中的 `7f3c8268b3f403594ef6e7e681ca62851e66b07b915c48a523b0cfaf7e54bfc9`，并把权限恢复为 `host.log`、`config.read`。
