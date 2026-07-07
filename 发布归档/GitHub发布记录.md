# GitHub 发布记录

## 当前发布
| 项目 | 内容 |
|------|------|
| Release | v1.10.0 - 自动更新自替换 |
| 发布日期 | 2026-07-07 |
| Git tag | `v1.10.0` |
| 指向提交 | `54af35700a9fb2e4d4f352445bad750ec67a6d59` |
| Release URL | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.10.0 |
| Release ID | `349992273` |
| 编译包 | `WoW_Framework_v1.10.0_windows.zip` |
| 编译包 SHA-256 | `72ecf47084a7290047f55c5cd9fffeae6621ef42fe83a4f112ef5b6231194038` |
| 当前状态 | 已完成 |

## 发布范围
- 发布 P16 已验证源码状态：`--update-apply`、独立 updater 脚本、托盘安装更新入口和更新保留边界。
- Release 内容使用 `发布归档/v1.10.0-发布说明.md`。
- GitHub 自动生成源码包，并额外上传普通 Windows 编译包。

## 安全边界
- 未上传 `dm.dll`。
- 未上传 `RegDll.dll`。
- 未上传 CHM/CHW 文档。
- 未上传授权文件、真实账号资料、签名私钥、真实商业脚本。
- 未上传 JSONL 历史文件和 `data/` 运行数据。
- 本次 Release 是本机试运行编译包发布，不代表完整生产部署完成。

## 验证记录
- 远端 `main` 发布前指向 `54af35700a9fb2e4d4f352445bad750ec67a6d59`。
- 远端 `v1.10.0` tag 已推送。
- GitHub Release 已通过 API 创建。
- 普通编译包 asset ID 为 `468730945`，大小 `4281610` bytes。
- GitHub asset digest 为 `sha256:72ecf47084a7290047f55c5cd9fffeae6621ef42fe83a4f112ef5b6231194038`。
- P16 功能验证沿用 `发布归档/v1.10.0-发布说明.md` 中记录的最终验证结果。

## 历史发布
| 版本 | Release | 指向提交 | 状态 |
|------|---------|----------|------|
| v1.9.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.9.0 | `65c4a2f96f0fcaa5a856e5a187b2bccde6a8683a` | 已完成 |
| v1.8.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.8.0 | `013c3af8f2bcc14f0cc39f13eb8ce5c7ad7f1c20` | 已完成 |
| v1.7.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.7.0 | `bddedf047ebe0336ae182d95972aa8ebdc5415a9` | 已完成 |
| v1.6.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.6.0 | `9444933b622285fd04e87158692805980f7689ae` | 已完成 |
| v1.5.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.5.0 | `96a67f80a541a662952d71a0c6737af7afa88576` | 已完成 |
| v1.4.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.4.0 | `ba2d5ab77cf56ff5fe5b1aa1c0b92ef2a0102fcd` | 已完成 |
| v1.3.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.3.0 | `18543ee7ceb9c166f7d6e55fd4ceb863d117ef76` | 已完成 |
| v1.2.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.2.0 | `41ca63e6b29658fd572d83c4378b6980dbf0bd36` | 已完成 |

## 下一步建议
进入 P17 前先确认优先方向：发布包签名验证、MSI/MSIX 安装器、远程命令鉴权审计、Web 数据分析或 Client 任务策略。
