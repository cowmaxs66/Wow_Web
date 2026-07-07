# GitHub 自动化说明

本目录保存 GitHub 侧自动化配置。

## 当前内容
| 目录 | 职责 |
|------|------|
| `workflows/` | GitHub Actions 工作流 |

## 边界
- CI 只运行公开源码可验证的 Rust 和 Web 检查。
- DmBridge Delphi 编译依赖本机 `C:\RAD13\bin\dcc32.exe`，不放入 GitHub runner。
- 发布包上传仍由本机完成，避免把大漠相关私有环境带入远端。
