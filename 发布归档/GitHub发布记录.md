# GitHub 发布记录

## 当前发布
| 项目 | 内容 |
|------|------|
| Release | v1.19.0 - Client 远程配置下发 |
| 发布日期 | 2026-07-07 |
| Git tag | `v1.19.0` |
| 指向提交 | `9b12fa26d76eb65f03c43f28a7c986abe0d37436` |
| Release URL | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.19.0 |
| Release ID | `350280292` |
| 编译包 | `WoW_Framework_v1.19.0_windows.zip`、`WoW_Server_v1.19.0_windows.zip`、`WoW_Client_v1.19.0_windows.zip` |
| 编译包 SHA-256 | 总包 `0b03a0790d240e6b9fdac4b26e2a61c7c8f10094d06087b88b352afe897cbdbd`；Server `29ebe747062e59f7b6895297725ad6db8a27ca92a6aaed516c2984de65ef2ea6`；Client `26341adb915672640a10f75ac5ef01469b7cdcbfeafa48f50e406cc154698755` |
| 当前状态 | 已完成 |

## 发布范围
- 发布 P26 已验证源码状态：`config.apply`、Client 配置补丁写回、monitor 动态重载、Web 远程配置面板和三类 zip 安全边界。
- Release 内容使用 `发布归档/v1.19.0-发布说明.md`。
- GitHub 自动生成源码包，并额外上传总包、Server 分包和 Client 分包。

## 待发布本地包
| 项目 | 内容 |
|------|------|
| 版本 | v1.21.0 |
| 范围 | DM 实机烟测与多机通讯规划 |
| 本地路径 | `target/release-package/` |
| 编译包 | `WoW_Framework_v1.21.0_windows.zip`、`WoW_Server_v1.21.0_windows.zip`、`WoW_Client_v1.21.0_windows.zip` |
| 编译包 SHA-256 | 总包 `dc00902da683c924f780a77fc16220be5b234e5d923fb76f0fc485d81dff835a`；Server `17e3f015c38a7f5a7db4c25adf144531a35cda191ef198cd8b2726dba63bb6f0`；Client `6cd35dffe48a82375cab14e1340802caa5fdde8efedfe64e8346b1b6b1579d72` |
| 当前状态 | 本地包已生成，GitHub Release 未创建 |

## 安全边界
- 未上传 `dm.dll`。
- 未上传 `RegDll.dll`。
- 未上传 CHM/CHW 文档。
- 未上传授权文件、真实账号资料、签名私钥、真实商业脚本。
- 未上传 JSONL 历史文件和 `data/` 运行数据。
- 本次 Release 是本机试运行编译包发布，不代表完整生产部署完成。

## 验证记录
- 远端 `main` 发布前指向 `9b12fa26d76eb65f03c43f28a7c986abe0d37436`。
- 远端 `v1.19.0` tag 已推送。
- GitHub Release 已通过 API 创建。
- 总包 asset ID 为 `469172817`，大小 `4303908` bytes，digest 为 `sha256:0b03a0790d240e6b9fdac4b26e2a61c7c8f10094d06087b88b352afe897cbdbd`。
- Server 分包 asset ID 为 `469172865`，大小 `1388023` bytes，digest 为 `sha256:29ebe747062e59f7b6895297725ad6db8a27ca92a6aaed516c2984de65ef2ea6`。
- Client 分包 asset ID 为 `469172904`，大小 `2674521` bytes，digest 为 `sha256:26341adb915672640a10f75ac5ef01469b7cdcbfeafa48f50e406cc154698755`。
- P26 功能验证沿用 `发布归档/v1.19.0-发布说明.md` 中记录的最终验证结果。

## 历史发布
| 版本 | Release | 指向提交 | 状态 |
|------|---------|----------|------|
| v1.18.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.18.0 | `93153cbea2709ae4e6d52856a57d093b3de30303` | 已完成 |
| v1.17.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.17.0 | `73a93f9a9d0ed9332a56ae624c88d08ba1d845a7` | 已完成 |
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
优先进入 P29 多机器管理数据模型、Client 分组/标签和批量操作安全边界；随后进入 P30 通讯效率优化，增加轮询 jitter 和 `/api/client/sync` 合并同步接口。
