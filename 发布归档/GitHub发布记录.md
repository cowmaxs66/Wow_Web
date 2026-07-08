# GitHub 发布记录

## 当前发布
| 项目 | 内容 |
|------|------|
| Release | v1.24.0 - DM 正式包与多选客户端操作 |
| 发布日期 | 2026-07-08 |
| Git tag | `v1.24.0` |
| 指向提交 | `ce772f8855ca83cdeb137579dcd8c8417d67a58b` |
| Release URL | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.24.0 |
| Release ID | `350645580` |
| 编译包 | `WoW_Framework_v1.24.0_windows.zip`、`WoW_Server_v1.24.0_windows.zip`、`WoW_Client_v1.24.0_windows.zip` |
| 编译包 SHA-256 | 总包 `f13ac447611caef4209865cec2aeaa98a631d59b4a73ee60461d612397d2be4b`；Server `34a26b5a7a0cbe3b2195f7ef91c9e2dd469a7aaf77a4e218b413809bb2866822`；Client `06f0dd76e88a45cda55f69d0a91edc745a4377f6c75fa0925d9790947f4eb29f` |
| 当前状态 | 已完成 |

## 发布范围
- 发布 P32 已验证源码状态：默认 DM 权限、DM DLL 随包、Web Client 复选框多选操作和三类 zip 安全边界。
- Release 内容使用 `发布归档/v1.24.0-发布说明.md`。
- GitHub 自动生成源码包，并额外上传总包、Server 分包和 Client 分包。

## 待发布本地包

| 项目 | 内容 |
|------|------|
| 状态 | 无 |

## 安全边界
- 总包和 Client 分包包含 `dm-bridge/Win32/dm.dll` 与 `dm-bridge/Win32/RegDll.dll`。
- Server 分包未上传 `dm.dll`、`RegDll.dll` 或 `DmBridge.dll`。
- 未上传 CHM/CHW 文档。
- 未上传授权文件、真实账号资料、签名私钥、真实商业脚本。
- 未上传 JSONL 历史文件和 `data/` 运行数据。
- 本次 Release 是正式测试包发布，不代表已完成授权分发、代码签名、登录鉴权和生产部署。
- 公开分发前必须确认大漠插件授权与再分发边界；授权不允许时应改用私有 Release 或用户本机安装 DM。

## 验证记录
- 远端 `main` 发布前指向 `ce772f8855ca83cdeb137579dcd8c8417d67a58b`。
- 远端 `v1.24.0` tag 已推送。
- GitHub Release 已通过 API 创建。
- 总包 asset ID 为 `469743934`，大小 `7847357` bytes，digest 为 `sha256:f13ac447611caef4209865cec2aeaa98a631d59b4a73ee60461d612397d2be4b`。
- Server 分包 asset ID 为 `469744009`，大小 `1503257` bytes，digest 为 `sha256:34a26b5a7a0cbe3b2195f7ef91c9e2dd469a7aaf77a4e218b413809bb2866822`。
- Client 分包 asset ID 为 `469744043`，大小 `6095347` bytes，digest 为 `sha256:06f0dd76e88a45cda55f69d0a91edc745a4377f6c75fa0925d9790947f4eb29f`。
- P32 功能验证沿用 `发布归档/v1.24.0-发布说明.md` 中记录的最终验证结果。

## 历史发布
| 版本 | Release | 指向提交 | 状态 |
|------|---------|----------|------|
| v1.23.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.23.0 | `8a587cae57d10624f6ae3171c005f3fe9d797582` | 已完成 |
| v1.19.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.19.0 | `9b12fa26d76eb65f03c43f28a7c986abe0d37436` | 已完成 |
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
优先测试 v1.24.0 GitHub Release 下载包；实机通过后建议补 DM 注册/授权检测向导、Client 批量操作结果筛选和发布包签名验证。
