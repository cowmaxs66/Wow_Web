# DM API 测试与上线准入说明

## 目标
上线前必须把“接口是否可用”和“目标窗口是否存在”分开验证，避免把 `World of Warcraft` 未打开误判为 API 故障。

## 测试分层
| 层级 | 脚本/命令 | 是否依赖目标窗口 | 说明 |
|------|-----------|------------------|------|
| L1 基础接口自检 | `scripts/dm_api_selftest.lua` | 否 | 验证 Lua 全局接口、DmBridge 加载、dm COM 初始化、版本、取色、RGB、等待颜色、窗口探测和无效绑定处理 |
| L2 窗口 smoke | `scripts/dm_window_smoke.lua` | 是 | 默认查找标题 `World of Warcraft`；找不到返回 `window=not_found`，找到才绑定和取色 |
| L3 业务脚本验证 | 用户业务 Lua | 是 | 按具体脚本验证图色、键鼠、后台模式和流程控制 |
| L4 高风险输入验证 | 手动安全窗口 | 是 | `left_click` 等会产生真实输入，只允许在安全窗口中手动验证 |

## 上线准入
- L1 必须通过，返回值以 `dm_api_selftest|` 开头。
- L2 在目标游戏未打开时允许返回 `window=not_found`；目标游戏已打开时必须返回 `window=found` 或明确绑定失败原因。
- Web 远程推送示例禁止使用 `dm.find_window_required`，避免目标窗口未开时让 Client monitor 持续失败。
- 业务脚本中只有“目标窗口必须存在，否则任务无意义”时才使用 `dm.find_window_required`。
- `dx` 后台模式必须在 `normal` 或 `gdi` 通过后再验证。

## 当前接口覆盖
| 接口 | L1 自检 | L2 窗口 smoke | 备注 |
|------|---------|---------------|------|
| `log` | 覆盖 | - | 写入脚本日志和回执 |
| `get_config` | 覆盖 | - | 读取 `client.id` |
| `dm.abi_version` | 覆盖 | - | DmBridge ABI |
| `dm.init` / `dm.shutdown` | 覆盖 | 覆盖 | COM 生命周期 |
| `dm.ver` | 覆盖 | 覆盖 | 大漠版本 |
| `dm.set_path` | 覆盖 | - | 使用当前目录 |
| `dm.last_bridge_error` / `dm.last_dm_error` | 覆盖 | 覆盖 | 错误码读取 |
| `dm.find_window` | 覆盖未找到分支 | 覆盖目标窗口分支 | 未找到返回 `0` |
| `dm.find_window_try` | 覆盖未找到分支 | 可业务扩展 | 返回结构化结果 |
| `dm.find_window_required` | 文档约束 | 业务脚本按需 | 强制型接口，不放入默认 smoke |
| `dm.safe_bind_window` | 覆盖无效 hwnd 分支 | 覆盖真实 hwnd 分支 | 推荐绑定入口 |
| `dm.bind_window_try` | 可业务扩展 | 可业务扩展 | 结构化绑定结果 |
| `dm.bind_window` | 不默认覆盖 | 不默认覆盖 | 失败会中断脚本，业务脚本慎用 |
| `dm.unbind_window` | - | 覆盖 | 绑定成功后执行 |
| `dm.get_color` / `dm.get_color_rgb` | 覆盖 | 覆盖 | 图色基础能力 |
| `dm.wait_color` | 覆盖 | 可业务扩展 | 等待当前颜色 |
| `dm.move_to` | 不默认覆盖 | 可手动覆盖 | 会移动鼠标，默认自检不执行 |
| `dm.left_click` | 不默认覆盖 | 仅安全窗口手动覆盖 | 会真实点击，不进入默认自检 |
| `dm.sleep_ms` / `dm.now_ms` | 覆盖 | - | 辅助函数 |

## 用户当前报错解释
```text
FindWindow not found: class= title=World of Warcraft
```

含义：当前桌面没有匹配标题 `World of Warcraft` 的窗口。处理方式：
1. 先运行 `dm_api_selftest.lua`，确认 DM 基础链路。
2. 打开游戏窗口后再运行 `dm_window_smoke.lua`。
3. 如果窗口标题不同，用实际标题替换脚本中的 `target_title`。
