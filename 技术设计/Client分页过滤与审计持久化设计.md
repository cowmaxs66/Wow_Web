# Client 分页过滤与审计持久化设计

## 阶段范围

本设计覆盖 P31：

- Server 提供 Client 最新状态分页查询。
- Server 支持按分组、标签、在线状态和关键字过滤。
- Server 对消息、命令下发和命令回执写入可选审计 JSONL。
- Web Admin Client 列表改为 Server 端筛选，远程操作页展示最近审计事件。

## 设计结论

- 保留旧接口 `GET /api/client/status`，新增 `GET /api/client/status-page`，避免破坏旧 Web 或旧脚本。
- 分页只作用于最新状态快照，不替代历史趋势接口。
- 过滤条件放在 Server 执行，减少多机器场景下 Web 一次性拉取全部 Client 的压力。
- 审计事件只保存操作摘要，不保存完整 payload，避免把敏感配置、脚本路径或业务内容长期写入审计文件。
- 审计持久化使用可选 JSONL 文件，未配置时保持内存最近事件，不改变默认启动门槛。

## Server API

### Client 分页查询

```http
GET /api/client/status-page?page=1&page_size=50&group=raid-a&tag=dm&online=true&search=client
```

响应结构：

```json
{
  "page": 1,
  "page_size": 50,
  "total": 2,
  "total_pages": 1,
  "items": []
}
```

字段边界：

| 字段 | 说明 |
|------|------|
| `page` | 页码，小于 1 时按 1 处理 |
| `page_size` | 每页数量，Server 限制在 1 到 100 |
| `group` | 精确匹配 `identity.group`，忽略大小写 |
| `tag` | 精确匹配任一 `identity.tags`，忽略大小写 |
| `online` | 可选布尔值，不传则不过滤在线状态 |
| `search` | 匹配 Client ID、显示名、分组、标签、脚本、版本、架构和上报目标 |

### Server 审计查询

```http
GET /api/server/audit?limit=50
```

响应结构：

```json
{
  "total": 1,
  "limit": 50,
  "items": [
    {
      "id": "client-a-command.created-178...",
      "timestamp_ms": 178...,
      "event_type": "command.created",
      "client_id": "client-a",
      "command_type": "startup.status",
      "success": null,
      "summary": "写入 Client 命令队列：startup.status"
    }
  ]
}
```

## 审计持久化

启用方式：

```powershell
$env:MANAGEMENT_SERVER_AUDIT_PATH='data/server-audit.jsonl'
```

审计事件来源：

| 事件类型 | 触发点 | 说明 |
|----------|--------|------|
| `message.created` | Server 写入 Client 消息队列 | 记录标题摘要 |
| `command.created` | Server 写入 Client 命令队列 | 记录命令类型 |
| `command.receipt` | Client 上报命令回执 | 记录成功/失败和摘要 |

边界：

- 当前不记录操作者身份，因为登录鉴权尚未接入。
- 当前不做 JSONL 轮转，长期运行前需要补保留策略。
- JSONL 文件可能包含 Client ID、命令类型和执行摘要，不得提交到 GitHub。
- 审计写入失败不会阻断消息或命令队列写入，避免日志问题影响控制面可用性。

## Web Admin 调整

- Client 列表新增 Server 端搜索、分组、标签、在线状态和每页数量控件。
- Client 列表显示分页总数和翻页按钮。
- 旧 Server 若没有 `/api/client/status-page`，Web 会回退 `/api/client/status`，保持兼容。
- 远程操作页新增审计面板，展示最近 Server 操作和 Client 回执事件。

## 后续扩展

- 接入登录鉴权后给审计事件增加 `operator_id`。
- 增加审计导出、保留策略和按 Client 查询。
- 为多机器规模测试增加 sync 请求耗时统计和 Server 处理耗时指标。
- 将在线状态阈值改为配置项，并在分页查询中返回失联原因。
