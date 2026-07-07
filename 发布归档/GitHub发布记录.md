# GitHub 发布记录

## 当前发布
| 项目 | 内容 |
|------|------|
| Release | v1.15.0 - 产品化控制中心与安装体验 |
| 发布日期 | 2026-07-07 |
| Git tag | `v1.15.0` |
| 指向提交 | `087652262188876fc42313265de0fc85787361de` |
| Release URL | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.15.0 |
| Release ID | `350106593` |
| 编译包 | `WoW_Framework_v1.15.0_windows.zip`、`WoW_Server_v1.15.0_windows.zip`、`WoW_Client_v1.15.0_windows.zip` |
| 编译包 SHA-256 | 总包 `58cbf6ec1c84a2a7760b0a90ebdc28bc7a91d14addb2d0630e18f5917a8427fd`；Server `67bcf1a6f193c7b29dacc7230699030eeccdd16a7f14100a94fbfca695d9c097`；Client `74099573c2226b423ec7457095ad28211a301fe54ade8fb615b6a4e29262ba22` |
| 当前状态 | 已完成 |

## 发布范围
- 发布 P22 已验证源码状态：产品控制中心、安装/修复入口、快捷方式图标、控制中心入口烟测和三类 zip 安全边界。
- Release 内容使用 `发布归档/v1.15.0-发布说明.md`。
- GitHub 自动生成源码包，并额外上传总包、Server 分包和 Client 分包。

## 安全边界
- 未上传 `dm.dll`。
- 未上传 `RegDll.dll`。
- 未上传 CHM/CHW 文档。
- 未上传授权文件、真实账号资料、签名私钥、真实商业脚本。
- 未上传 JSONL 历史文件和 `data/` 运行数据。
- 本次 Release 是本机试运行编译包发布，不代表完整生产部署完成。

## 验证记录
- 远端 `main` 发布前指向 `087652262188876fc42313265de0fc85787361de`。
- 远端 `v1.15.0` tag 已推送。
- GitHub Release 已通过 API 创建。
- 总包 asset ID 为 `468968845`，大小 `4179386` bytes，digest 为 `sha256:58cbf6ec1c84a2a7760b0a90ebdc28bc7a91d14addb2d0630e18f5917a8427fd`。
- Server 分包 asset ID 为 `468968900`，大小 `1351215` bytes，digest 为 `sha256:67bcf1a6f193c7b29dacc7230699030eeccdd16a7f14100a94fbfca695d9c097`。
- Client 分包 asset ID 为 `468968945`，大小 `2586490` bytes，digest 为 `sha256:74099573c2226b423ec7457095ad28211a301fe54ade8fb615b6a4e29262ba22`。
- P22 功能验证沿用 `发布归档/v1.15.0-发布说明.md` 中记录的最终验证结果。

## 历史发布
| 版本 | Release | 指向提交 | 状态 |
|------|---------|----------|------|
| v1.14.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.14.0 | `80c1a0c5542422e010cf3fa49494cb56b2263f4a` | 已完成 |
| v1.13.1 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.13.1 | `3adecdb363931daf39067e1e604c90641461eef6` | 已完成 |
| v1.13.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.13.0 | `8e4230aede57f4546cf8b1b6c135740d398154ff` | 已完成 |
| v1.12.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.12.0 | `a2ce7b5f0e10d7ac9241fe5158952556e9d1b4cf` | 已完成 |
| v1.11.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.11.0 | `0f6cc6420be98de52a4eef983f038216165a8193` | 已完成 |
| v1.10.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.10.0 | `54af35700a9fb2e4d4f352445bad750ec67a6d59` | 已完成 |
| v1.9.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.9.0 | `65c4a2f96f0fcaa5a856e5a187b2bccde6a8683a` | 已完成 |
| v1.8.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.8.0 | `013c3af8f2bcc14f0cc39f13eb8ce5c7ad7f1c20` | 已完成 |
| v1.7.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.7.0 | `bddedf047ebe0336ae182d95972aa8ebdc5415a9` | 已完成 |
| v1.6.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.6.0 | `9444933b622285fd04e87158692805980f7689ae` | 已完成 |
| v1.5.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.5.0 | `96a67f80a541a662952d71a0c6737af7afa88576` | 已完成 |
| v1.4.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.4.0 | `ba2d5ab77cf56ff5fe5b1aa1c0b92ef2a0102fcd` | 已完成 |
| v1.3.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.3.0 | `18543ee7ceb9c166f7d6e55fd4ceb863d117ef76` | 已完成 |
| v1.2.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.2.0 | `41ca63e6b29658fd572d83c4378b6980dbf0bd36` | 已完成 |

## 下一步建议
进入 P23 前先确认优先方向：Server Windows Service 化、MSI/MSIX 安装器、代码签名、命令执行回执、远程命令鉴权审计或发布包签名验证。
