# GitHub 备份策略

## 当前状态
- 本机已安装 Git。
- 当前未安装 GitHub CLI `gh`。
- 当前工作区已绑定远端仓库：`https://github.com/cowmaxs66/Wow_Web.git`。
- 本地 `main` 已推送到 GitHub。

## 推荐策略
默认建议使用 private GitHub 仓库保存开发进度，避免自动化框架、脚本和配置过早公开。当前用户指定仓库 `cowmaxs66/Wow_Web` 为 public，后续不要提交敏感配置、密钥、真实脚本账号信息或商业数据。

## 接入方式
### 方式 A：使用已有仓库
用户提供：

```text
owner/repo
```

随后执行：

```powershell
git remote add origin https://github.com/owner/repo.git
git push -u origin main
```

当前已执行：

```powershell
git remote add origin https://github.com/cowmaxs66/Wow_Web.git
git push -u origin main
```

### 方式 B：安装并登录 GitHub CLI
安装并登录后执行：

```powershell
gh auth login
gh repo create wow-automation-framework --private --source . --remote origin --push
```

## 版本更新规则
- 每个可验证阶段完成后提交一次。
- 每个阶段收口后推送一次到 `origin/main`。
- 版本号记录在 `VERSION`。
- 阶段完成时更新 `计划报告/项目总阶段预览表.md`。
- 重要变更记录到 `计划报告/变更记录.md`。
