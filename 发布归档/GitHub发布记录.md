# GitHub 发布记录

## 当前发布
| 项目 | 内容 |
|------|------|
| Release | v1.3.0 - 生产持久化 MVP |
| 发布日期 | 2026-07-06 |
| Git tag | `v1.3.0` |
| 指向提交 | `18543ee7ceb9c166f7d6e55fd4ceb863d117ef76` |
| Release URL | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.3.0 |
| Release ID | `349552553` |
| 编译包 | `WoW_Framework_v1.3.0_windows_x64.zip` |
| 编译包 SHA-256 | `8e1bc61b70348db81d7fbb7741fc18cb8c1306a1f1ab3968d5c34464c1bd9813` |
| 当前状态 | 已完成 |

## 发布范围
- 发布 P9 已验证源码状态：JSONL 历史持久化、启动回放、打包路径兼容和普通编译包。
- Release 内容使用 `发布归档/v1.3.0-发布说明.md`。
- GitHub 自动生成源码包，并额外上传普通 Windows x64 编译包。

## 安全边界
- 未上传 `dm.dll`。
- 未上传 `RegDll.dll`。
- 未上传 CHM/CHW 文档。
- 未上传授权文件、真实账号资料、签名私钥、真实商业脚本。
- 未上传 JSONL 历史文件和 `data/` 运行数据。
- 本次 Release 是普通编译包发布，不代表完整生产部署完成。

## 验证记录
- 远端 `main` 发布前指向 `18543ee7ceb9c166f7d6e55fd4ceb863d117ef76`。
- 远端 `v1.3.0` tag 已推送。
- GitHub Release 已通过 API 创建。
- 普通编译包 asset ID 为 `468073738`，大小 `2035481` bytes。
- P9 功能验证沿用 `发布归档/v1.3.0-发布说明.md` 中记录的最终验证结果。

## 历史发布
| 版本 | Release | 指向提交 | 状态 |
|------|---------|----------|------|
| v1.2.0 | https://github.com/cowmaxs66/Wow_Web/releases/tag/v1.2.0 | `41ca63e6b29658fd572d83c4378b6980dbf0bd36` | 已完成 |

## 下一步建议
进入 P10：数据库持久化升级，补 SQLite/Postgres 真实落地、迁移脚本、保留策略自动化和备份恢复演练。
