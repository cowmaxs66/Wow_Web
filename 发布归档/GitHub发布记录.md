# GitHub 发布记录

## 当前发布
| 项目 | 内容 |
|------|------|
| Release | v1.4.0 - 一键运行与首次设置向导 |
| 发布日期 | 2026-07-06 |
| Git tag | `v1.4.0` |
| 指向提交 | `ba2d5ab77cf56ff5fe5b1aa1c0b92ef2a0102fcd` |
| Release URL | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.4.0 |
| Release ID | `349571607` |
| 编译包 | `WoW_Framework_v1.4.0_windows.zip` |
| 编译包 SHA-256 | `05ce14b511821514c438813e2e1fdf233bc13f9273f3330ad9a9f3ceb52356a4` |
| 当前状态 | 已完成 |

## 发布范围
- 发布 P10 已验证源码状态：一键运行脚本、Management Server Web 托管、首次设置向导和 x86/x64 编译包。
- Release 内容使用 `发布归档/v1.4.0-发布说明.md`。
- GitHub 自动生成源码包，并额外上传普通 Windows 编译包。

## 安全边界
- 未上传 `dm.dll`。
- 未上传 `RegDll.dll`。
- 未上传 CHM/CHW 文档。
- 未上传授权文件、真实账号资料、签名私钥、真实商业脚本。
- 未上传 JSONL 历史文件和 `data/` 运行数据。
- 本次 Release 是本机试运行编译包发布，不代表完整生产部署完成。

## 验证记录
- 远端 `main` 发布前指向 `ba2d5ab77cf56ff5fe5b1aa1c0b92ef2a0102fcd`。
- 远端 `v1.4.0` tag 已推送。
- GitHub Release 已通过 API 创建。
- 普通编译包 asset ID 为 `468098541`，大小 `3482415` bytes。
- P10 功能验证沿用 `发布归档/v1.4.0-发布说明.md` 中记录的最终验证结果。

## 历史发布
| 版本 | Release | 指向提交 | 状态 |
|------|---------|----------|------|
| v1.3.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.3.0 | `18543ee7ceb9c166f7d6e55fd4ceb863d117ef76` | 已完成 |
| v1.2.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.2.0 | `41ca63e6b29658fd572d83c4378b6980dbf0bd36` | 已完成 |

## 下一步建议
确认 P11 范围：可在数据分析、客户端详细设置、配置落地或服务化运行中选择下一阶段目标。
