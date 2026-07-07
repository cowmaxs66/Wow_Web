# 图标资源说明

本目录保存 Server 和 Client 的公开图标资源。

## 文件说明
| 文件 | 职责 |
|------|------|
| `lua_ai_server_icon.svg` | Server 图标源文件 |
| `lua_ai_client_icon.svg` | Client 图标源文件 |
| `server.ico` | Server 托盘和发布包运行时图标 |
| `client.ico` | Client 托盘和发布包运行时图标 |

## 使用方式
- 发布包会复制对应 `.ico` 到 `assets/icons/`。
- 托盘程序优先读取 `.ico`；文件缺失时回退系统默认图标。
