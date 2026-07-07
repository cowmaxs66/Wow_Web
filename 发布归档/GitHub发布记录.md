# GitHub 发布记录

## 当前发布
| 项目 | 内容 |
|------|------|
| Release | v1.17.0 - 命令执行回执与审计可视化 |
| 发布日期 | 2026-07-07 |
| Git tag | `v1.17.0` |
| 指向提交 | `73a93f9a9d0ed9332a56ae624c88d08ba1d845a7` |
| Release URL | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.17.0 |
| Release ID | `350159635` |
| 编译包 | `WoW_Framework_v1.17.0_windows.zip`、`WoW_Server_v1.17.0_windows.zip`、`WoW_Client_v1.17.0_windows.zip` |
| 编译包 SHA-256 | 总包 `b33a9f3efc3fb4ad493006cf6f081b1af827714a7d839429d7358fb021ac8ca7`；Server `7ab457c001d493041cd8135e5e9d1edc72baa8e921137b4c30f3ceb7ab64fde0`；Client `d5ad14c5c92c0f1734fdb5f532a2fd3b8dba8d30b91e89af20f7440500fcd448` |
| 当前状态 | 已完成 |

## 发布范围
- 发布 P24 已验证源码状态：命令执行回执协议、Server 回执 API、Client 执行后上报、Web 最近回执和三类 zip 安全边界。
- Release 内容使用 `发布归档/v1.17.0-发布说明.md`。
- GitHub 自动生成源码包，并额外上传总包、Server 分包和 Client 分包。

## 安全边界
- 未上传 `dm.dll`。
- 未上传 `RegDll.dll`。
- 未上传 CHM/CHW 文档。
- 未上传授权文件、真实账号资料、签名私钥、真实商业脚本。
- 未上传 JSONL 历史文件和 `data/` 运行数据。
- 本次 Release 是本机试运行编译包发布，不代表完整生产部署完成。

## 验证记录
- 远端 `main` 发布前指向 `73a93f9a9d0ed9332a56ae624c88d08ba1d845a7`。
- 远端 `v1.17.0` tag 已推送。
- GitHub Release 已通过 API 创建。
- 总包 asset ID 为 `469053696`，大小 `4203379` bytes，digest 为 `sha256:b33a9f3efc3fb4ad493006cf6f081b1af827714a7d839429d7358fb021ac8ca7`。
- Server 分包 asset ID 为 `469053768`，大小 `1367333` bytes，digest 为 `sha256:7ab457c001d493041cd8135e5e9d1edc72baa8e921137b4c30f3ceb7ab64fde0`。
- Client 分包 asset ID 为 `469053809`，大小 `2594442` bytes，digest 为 `sha256:d5ad14c5c92c0f1734fdb5f532a2fd3b8dba8d30b91e89af20f7440500fcd448`。
- P24 功能验证沿用 `发布归档/v1.17.0-发布说明.md` 中记录的最终验证结果。

## 历史发布
| 版本 | Release | 指向提交 | 状态 |
|------|---------|----------|------|
| v1.16.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.16.0 | `cc361bb5427b1700872bbab9d77dc752dacb9fec` | 已完成 |
| v1.15.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.15.0 | `087652262188876fc42313265de0fc85787361de` | 已完成 |
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
进入 P25 前先确认优先方向：持久化审计与鉴权、脚本分发签名流程、真实 DM 场景验证、MSI/MSIX 安装器或代码签名。
