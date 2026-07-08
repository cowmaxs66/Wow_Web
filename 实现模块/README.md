# 实现模块说明

本目录保存各技术实现模块。

## 模块列表
| 模块 | 类型 | 当前状态 | 职责 |
|------|------|----------|------|
| `shared-types` | Rust library | 已完成 | Client/Server/Web 共用状态、远程命令、热推送和审计协议类型 |
| `client-agent` | Rust binary | 已完成 | 客户端代理入口、Lua 宿主、大漠高层 API、状态上报、Lua 热推送、脚本安全门和运行详情摘要 |
| `management-server` | Rust binary | 已完成 | HTTP 状态接收、Client 查询、命令队列、Lua 热推送校验、审计和 Web Admin 读取 API |
| `wow-launcher` | Rust binary | 进行中 | 无控制台正式入口、安装器入口和卸载入口 |
| `dm-bridge` | Delphi DLL | 已完成 | 大漠 COM 桥接最小链路 |
| `web-admin` | Vue frontend | 已完成 | Web 管理端，展示健康、快照分析、历史趋势、Client 详情、本地设置、远程操作和 Lua 热推送 |
