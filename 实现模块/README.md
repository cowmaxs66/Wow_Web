# 实现模块说明

本目录保存各技术实现模块。

## 模块列表
| 模块 | 类型 | 当前状态 | 职责 |
|------|------|----------|------|
| `shared-types` | Rust library | 进行中 | Client/Server 共用协议类型 |
| `client-agent` | Rust binary | 进行中 | 客户端代理入口、Lua 宿主、大漠高层 API、状态上报 |
| `management-server` | Rust binary | 进行中 | HTTP 状态接收、查询和 Web Admin 读取 API |
| `dm-bridge` | Delphi DLL | 进行中 | 大漠 COM 桥接 |
| `web-admin` | Vue frontend | 进行中 | Web 管理端 MVP，展示 Server 健康和 Client 状态 |
