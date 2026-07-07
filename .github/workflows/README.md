# GitHub Actions 工作流说明

## 当前文件
| 文件 | 职责 |
|------|------|
| `ci.yml` | 在 push 和 pull request 时运行 Rust 与 Web 基础验证 |

## 验证范围
- `cargo fmt --all --check`
- `cargo clippy --workspace -- -D warnings`
- `cargo test --workspace`
- `npm ci`
- `npm run build`

## 不包含项
- 不编译 DmBridge Delphi DLL。
- 不生成正式发布包。
- 不上传 Release 资产。
