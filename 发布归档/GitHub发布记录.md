# GitHub 发布记录

## 当前发布
| 项目 | 内容 |
|------|------|
| Release | v1.5.0 - 单 exe 与客户端监控基础 |
| 发布日期 | 2026-07-07 |
| Git tag | `v1.5.0` |
| 指向提交 | `96a67f80a541a662952d71a0c6737af7afa88576` |
| Release URL | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.5.0 |
| Release ID | `349961863` |
| 编译包 | `WoW_Framework_v1.5.0_windows.zip` |
| 编译包 SHA-256 | `b58bd9202b1d54ccbec4f2c4170d9190cc33c48e32b9d3e5e565a5fb52cde541` |
| 当前状态 | 已完成 |

## 发布范围
- 发布 P11 已验证源码状态：Server 内嵌 Web、Client monitor、本地日志、Server 消息、通知气泡和双 exe 编译包。
- Release 内容使用 `发布归档/v1.5.0-发布说明.md`。
- GitHub 自动生成源码包，并额外上传普通 Windows 编译包。

## 安全边界
- 未上传 `dm.dll`。
- 未上传 `RegDll.dll`。
- 未上传 CHM/CHW 文档。
- 未上传授权文件、真实账号资料、签名私钥、真实商业脚本。
- 未上传 JSONL 历史文件和 `data/` 运行数据。
- 本次 Release 是本机试运行编译包发布，不代表完整生产部署完成。

## 验证记录
- 远端 `main` 发布前指向 `96a67f80a541a662952d71a0c6737af7afa88576`。
- 远端 `v1.5.0` tag 已推送。
- GitHub Release 已通过 API 创建。
- 普通编译包 asset ID 为 `468649845`，大小 `2608385` bytes。
- P11 功能验证沿用 `发布归档/v1.5.0-发布说明.md` 中记录的最终验证结果。

## 历史发布
| 版本 | Release | 指向提交 | 状态 |
|------|---------|----------|------|
| v1.4.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.4.0 | `ba2d5ab77cf56ff5fe5b1aa1c0b92ef2a0102fcd` | 已完成 |
| v1.3.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.3.0 | `18543ee7ceb9c166f7d6e55fd4ceb863d117ef76` | 已完成 |
| v1.2.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.2.0 | `41ca63e6b29658fd572d83c4378b6980dbf0bd36` | 已完成 |

## 下一步建议
进入 P12 前先确认优先方向：持久托盘右键菜单、原生设置窗口、Windows Service 或 Web 数据分析。
