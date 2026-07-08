use crate::persistence::{AuditPersistence, HistoryPersistence, PersistenceError};
use shared_types::{
    AdminRealtimeMessage, ClientCommand, ClientCommandReceipt, ClientCommandReceiptRequest,
    ClientCommandRequest, ClientMessage, ClientMessageRequest, ClientStatus, ClientStatusPage,
    ServerAuditEvent, ServerRealtimeMessage, WsEnvelope,
};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc::UnboundedSender;

pub const CLIENT_HISTORY_LIMIT: usize = 50;
pub const CLIENT_MESSAGE_LIMIT: usize = 100;
pub const CLIENT_COMMAND_LIMIT: usize = 100;
pub const CLIENT_COMMAND_RECEIPT_LIMIT: usize = 100;
pub const CLIENT_ONLINE_STALE_MS: u128 = 30_000;
pub const SERVER_AUDIT_LIMIT: usize = 500;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ClientStatusPageFilter {
    pub page: usize,
    pub page_size: usize,
    pub group: Option<String>,
    pub tag: Option<String>,
    pub online: Option<bool>,
    pub search: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ServerState {
    clients: Arc<RwLock<HashMap<String, WsEnvelope<ClientStatus>>>>,
    histories: Arc<RwLock<HashMap<String, VecDeque<WsEnvelope<ClientStatus>>>>>,
    messages: Arc<RwLock<HashMap<String, VecDeque<ClientMessage>>>>,
    commands: Arc<RwLock<HashMap<String, VecDeque<ClientCommand>>>>,
    command_receipts: Arc<RwLock<HashMap<String, VecDeque<ClientCommandReceipt>>>>,
    audit_events: Arc<RwLock<VecDeque<ServerAuditEvent>>>,
    realtime_clients: Arc<RwLock<HashMap<String, ClientRealtimeConnection>>>,
    realtime_admins: Arc<RwLock<HashMap<u64, UnboundedSender<AdminRealtimeMessage>>>>,
    next_realtime_connection_id: Arc<AtomicU64>,
    persistence: Option<HistoryPersistence>,
    audit_persistence: Option<AuditPersistence>,
}

#[derive(Debug, Clone)]
struct ClientRealtimeConnection {
    connection_id: u64,
    sender: UnboundedSender<ServerRealtimeMessage>,
}

impl ServerState {
    pub fn from_paths(
        history_path: Option<PathBuf>,
        audit_path: Option<PathBuf>,
    ) -> Result<Self, PersistenceError> {
        let persistence = history_path
            .as_ref()
            .map(|path| HistoryPersistence::open(path.clone()))
            .transpose()?;
        let audit_persistence = audit_path
            .as_ref()
            .map(|path| AuditPersistence::open(path.clone()))
            .transpose()?;
        let state = Self {
            persistence,
            audit_persistence,
            ..Self::default()
        };

        if let Some(path) = history_path {
            for envelope in HistoryPersistence::load(&path)? {
                state.save_status_in_memory(envelope);
            }
        }

        if let Some(path) = audit_path {
            for event in AuditPersistence::load(&path)? {
                state.save_audit_in_memory(event);
            }
        }

        Ok(state)
    }

    pub fn save_status(&self, envelope: WsEnvelope<ClientStatus>) -> Result<(), PersistenceError> {
        if let Some(persistence) = &self.persistence {
            persistence.append(&envelope)?;
        }

        self.save_status_in_memory(envelope.clone());
        self.broadcast_admin(AdminRealtimeMessage::ClientStatus {
            status: Box::new(envelope),
        });
        Ok(())
    }

    fn save_status_in_memory(&self, envelope: WsEnvelope<ClientStatus>) {
        let client_id = envelope.client_id.clone();

        let mut clients = self.clients.write().expect("client status lock poisoned");
        // 最新状态仍然单独保存，保证 P3/P4/P7 的查询接口语义不变。
        // 输入：Client 上报的状态信封。
        // 输出：内存中的最新状态快照。
        // 边界：未配置持久化时进程退出即丢失；配置持久化时启动会从 JSONL 回放。
        clients.insert(client_id.clone(), envelope.clone());
        drop(clients);

        let mut histories = self
            .histories
            .write()
            .expect("client history lock poisoned");
        let history = histories.entry(client_id).or_default();
        history.push_back(envelope);

        while history.len() > CLIENT_HISTORY_LIMIT {
            history.pop_front();
        }
    }

    pub fn get_status(&self, client_id: &str) -> Option<WsEnvelope<ClientStatus>> {
        self.clients
            .read()
            .expect("client status lock poisoned")
            .get(client_id)
            .cloned()
            .map(mark_stale_status)
    }

    pub fn list_statuses(&self) -> Vec<WsEnvelope<ClientStatus>> {
        let clients = self.clients.read().expect("client status lock poisoned");
        let mut statuses: Vec<_> = clients.values().cloned().map(mark_stale_status).collect();

        // 列表输出按 client_id 排序，保证 Web 管理端和测试看到稳定顺序。
        // 输入：内存中的最新状态 HashMap。
        // 输出：按 client_id 升序排列的状态数组。
        // 边界：P17 只在最新状态查询层收敛在线状态，不改写历史样本。
        statuses.sort_by(|left, right| left.client_id.cmp(&right.client_id));
        statuses
    }

    pub fn list_status_page(&self, filter: ClientStatusPageFilter) -> ClientStatusPage {
        let page = filter.page.max(1);
        let page_size = filter.page_size.clamp(1, 100);
        let filtered: Vec<_> = self
            .list_statuses()
            .into_iter()
            .filter(|status| status_matches_filter(status, &filter))
            .collect();
        let total = filtered.len();
        let start = page.saturating_sub(1).saturating_mul(page_size);
        let items = filtered.into_iter().skip(start).take(page_size).collect();

        ClientStatusPage::new(page, page_size, total, items)
    }

    pub fn get_history(&self, client_id: &str) -> Vec<WsEnvelope<ClientStatus>> {
        self.histories
            .read()
            .expect("client history lock poisoned")
            .get(client_id)
            .map(|history| history.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn push_message(&self, client_id: &str, request: ClientMessageRequest) -> ClientMessage {
        let message = ClientMessage::new(client_id.to_string(), request);
        let mut messages = self.messages.write().expect("client message lock poisoned");
        let queue = messages.entry(client_id.to_string()).or_default();

        // P11 消息队列只做本机试运行内存队列。
        // 输入：Server 创建的 ClientMessage。
        // 输出：每个 Client 最近 100 条消息。
        // 边界：Server 重启后消息丢失；生产持久化和确认机制后续阶段单独设计。
        queue.push_back(message.clone());
        while queue.len() > CLIENT_MESSAGE_LIMIT {
            queue.pop_front();
        }

        let _ = self.record_audit_event(ServerAuditEvent::new(
            "message.created",
            client_id,
            None,
            None,
            format!("写入 Server 消息：{}", message.title),
        ));
        self.send_realtime_message(&message);
        self.broadcast_admin(AdminRealtimeMessage::ClientMessage {
            message: Box::new(message.clone()),
        });
        message
    }

    pub fn list_messages(&self, client_id: &str) -> Vec<ClientMessage> {
        self.messages
            .read()
            .expect("client message lock poisoned")
            .get(client_id)
            .map(|messages| messages.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn push_command(&self, client_id: &str, request: ClientCommandRequest) -> ClientCommand {
        let command_type = request.command_type.clone();
        let command = ClientCommand::new(client_id.to_string(), request);
        let mut commands = self.commands.write().expect("client command lock poisoned");
        let queue = commands.entry(client_id.to_string()).or_default();

        // P13 命令队列独立于文本消息队列。
        // 输入：Server 创建的 ClientCommand。
        // 输出：每个 Client 最近 100 条待轮询命令。
        // 边界：当前为内存队列；鉴权、审计、持久化和确认后续继续补齐。
        queue.push_back(command.clone());
        while queue.len() > CLIENT_COMMAND_LIMIT {
            queue.pop_front();
        }

        let _ = self.record_audit_event(ServerAuditEvent::new(
            "command.created",
            client_id,
            Some(command_type.clone()),
            None,
            format!("写入 Client 命令队列：{command_type}"),
        ));
        self.send_realtime_command(&command);
        self.broadcast_admin(AdminRealtimeMessage::CommandQueued {
            command: Box::new(command.clone()),
        });
        command
    }

    pub fn take_commands(&self, client_id: &str) -> Vec<ClientCommand> {
        self.commands
            .write()
            .expect("client command lock poisoned")
            .get_mut(client_id)
            .map(|commands| commands.drain(..).collect())
            .unwrap_or_default()
    }

    pub fn pending_commands(&self, client_id: &str) -> Vec<ClientCommand> {
        self.commands
            .read()
            .expect("client command lock poisoned")
            .get(client_id)
            .map(|commands| commands.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn push_command_receipt(
        &self,
        client_id: &str,
        request: ClientCommandReceiptRequest,
    ) -> ClientCommandReceipt {
        let receipt = ClientCommandReceipt::new(client_id.to_string(), request);
        let mut receipts = self
            .command_receipts
            .write()
            .expect("client command receipt lock poisoned");
        let queue = receipts.entry(client_id.to_string()).or_default();

        // P24 回执队列只保存最近结果，解决“命令是否执行”不可见的问题。
        // 输入：Client 回传的执行结果。
        // 输出：每个 Client 最近 100 条命令执行记录。
        // 边界：当前仍是内存审计，Server 重启后丢失；生产持久化后续单独设计。
        queue.push_back(receipt.clone());
        while queue.len() > CLIENT_COMMAND_RECEIPT_LIMIT {
            queue.pop_front();
        }

        let _ = self.record_audit_event(ServerAuditEvent::new(
            "command.receipt",
            client_id,
            Some(receipt.command_type.clone()),
            Some(receipt.success),
            receipt.summary.clone(),
        ));
        self.remove_pending_command(client_id, &receipt.command_id);
        self.broadcast_admin(AdminRealtimeMessage::CommandReceipt {
            receipt: Box::new(receipt.clone()),
        });
        receipt
    }

    pub fn list_command_receipts(&self, client_id: &str) -> Vec<ClientCommandReceipt> {
        self.command_receipts
            .read()
            .expect("client command receipt lock poisoned")
            .get(client_id)
            .map(|receipts| receipts.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn record_audit_event(&self, event: ServerAuditEvent) -> Result<(), PersistenceError> {
        // 审计事件先写入内存，再尝试落盘。
        // 输入：Server 操作或 Client 回执生成的事件。
        // 输出：Web 能立即看到最近事件；配置文件路径有效时追加 JSONL。
        // 边界：落盘失败会返回错误，但调用方可选择不阻断主流程。
        self.save_audit_in_memory(event.clone());

        if let Some(persistence) = &self.audit_persistence {
            persistence.append(&event)?;
        }

        Ok(())
    }

    fn save_audit_in_memory(&self, event: ServerAuditEvent) {
        let mut audit_events = self
            .audit_events
            .write()
            .expect("server audit lock poisoned");
        audit_events.push_back(event);

        while audit_events.len() > SERVER_AUDIT_LIMIT {
            audit_events.pop_front();
        }
    }

    pub fn list_audit_events(&self, limit: usize) -> Vec<ServerAuditEvent> {
        let limit = limit.clamp(1, SERVER_AUDIT_LIMIT);
        let audit_events = self
            .audit_events
            .read()
            .expect("server audit lock poisoned");

        audit_events.iter().rev().take(limit).cloned().collect()
    }

    pub fn register_realtime_client(
        &self,
        client_id: impl Into<String>,
        sender: UnboundedSender<ServerRealtimeMessage>,
    ) -> u64 {
        let client_id = client_id.into();
        let connection_id = self.next_realtime_id();
        let connection = ClientRealtimeConnection {
            connection_id,
            sender,
        };

        // 同一个 Client 只能保留最新实时连接，避免旧连接断开时继续收到命令。
        // 输入：Client ID、当前 WebSocket writer 对应的 channel。
        // 输出：connection_id，用于断线时只注销本连接。
        // 边界：旧连接的关闭事件不能删除新连接，注销时会校验 connection_id。
        self.realtime_clients
            .write()
            .expect("realtime client lock poisoned")
            .insert(client_id.clone(), connection);
        self.broadcast_admin(AdminRealtimeMessage::ClientConnected { client_id });
        connection_id
    }

    pub fn unregister_realtime_client(&self, client_id: &str, connection_id: u64) {
        let removed = {
            let mut clients = self
                .realtime_clients
                .write()
                .expect("realtime client lock poisoned");
            let should_remove = clients
                .get(client_id)
                .is_some_and(|connection| connection.connection_id == connection_id);
            if should_remove {
                clients.remove(client_id);
            }
            should_remove
        };

        if removed {
            self.broadcast_admin(AdminRealtimeMessage::ClientDisconnected {
                client_id: client_id.to_string(),
            });
        }
    }

    pub fn register_realtime_admin(&self, sender: UnboundedSender<AdminRealtimeMessage>) -> u64 {
        let connection_id = self.next_realtime_id();
        self.realtime_admins
            .write()
            .expect("realtime admin lock poisoned")
            .insert(connection_id, sender);
        connection_id
    }

    pub fn unregister_realtime_admin(&self, connection_id: u64) {
        self.realtime_admins
            .write()
            .expect("realtime admin lock poisoned")
            .remove(&connection_id);
    }

    pub fn list_realtime_clients(&self) -> Vec<String> {
        let mut client_ids: Vec<_> = self
            .realtime_clients
            .read()
            .expect("realtime client lock poisoned")
            .keys()
            .cloned()
            .collect();
        client_ids.sort();
        client_ids
    }

    pub fn realtime_admin_count(&self) -> usize {
        self.realtime_admins
            .read()
            .expect("realtime admin lock poisoned")
            .len()
    }

    fn send_realtime_command(&self, command: &ClientCommand) -> bool {
        let sender = self
            .realtime_clients
            .read()
            .expect("realtime client lock poisoned")
            .get(&command.client_id)
            .map(|connection| connection.sender.clone());

        sender.is_some_and(|sender| {
            sender
                .send(ServerRealtimeMessage::Command {
                    command: command.clone(),
                })
                .is_ok()
        })
    }

    fn send_realtime_message(&self, message: &ClientMessage) -> bool {
        let sender = self
            .realtime_clients
            .read()
            .expect("realtime client lock poisoned")
            .get(&message.client_id)
            .map(|connection| connection.sender.clone());

        sender.is_some_and(|sender| {
            sender
                .send(ServerRealtimeMessage::Message {
                    message: message.clone(),
                })
                .is_ok()
        })
    }

    fn broadcast_admin(&self, message: AdminRealtimeMessage) {
        let mut admins = self
            .realtime_admins
            .write()
            .expect("realtime admin lock poisoned");

        // Admin WS 是 UI 加速通道，发送失败只说明该页面已断开。
        // 输入：状态、命令、回执等事件。
        // 输出：仍在线的 Admin 连接继续保留，已断开的 sender 被清理。
        // 边界：前端还会走 HTTP 防抖刷新，因此这里不阻断业务写入。
        admins.retain(|_, sender| sender.send(message.clone()).is_ok());
    }

    fn remove_pending_command(&self, client_id: &str, command_id: &str) {
        if command_id.trim().is_empty() {
            return;
        }

        let mut commands = self.commands.write().expect("client command lock poisoned");
        if let Some(queue) = commands.get_mut(client_id) {
            queue.retain(|command| command.id != command_id);
        }
    }

    fn next_realtime_id(&self) -> u64 {
        self.next_realtime_connection_id
            .fetch_add(1, Ordering::Relaxed)
            + 1
    }
}

fn status_matches_filter(
    envelope: &WsEnvelope<ClientStatus>,
    filter: &ClientStatusPageFilter,
) -> bool {
    if let Some(online) = filter.online
        && envelope.data.online != online
    {
        return false;
    }

    if let Some(group) = trimmed_filter(&filter.group)
        && !text_eq(&envelope.data.identity.group, group)
    {
        return false;
    }

    if let Some(tag) = trimmed_filter(&filter.tag)
        && !envelope
            .data
            .identity
            .tags
            .iter()
            .any(|value| text_eq(value, tag))
    {
        return false;
    }

    if let Some(search) = trimmed_filter(&filter.search)
        && !status_search_text(envelope).contains(&search.to_lowercase())
    {
        return false;
    }

    true
}

fn trimmed_filter(value: &Option<String>) -> Option<&str> {
    value
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn text_eq(left: &str, right: &str) -> bool {
    left.trim().eq_ignore_ascii_case(right.trim())
}

fn status_search_text(envelope: &WsEnvelope<ClientStatus>) -> String {
    [
        envelope.client_id.as_str(),
        envelope.data.identity.display_name.as_str(),
        envelope.data.identity.group.as_str(),
        &envelope.data.identity.tags.join(" "),
        envelope.data.current_script.as_deref().unwrap_or_default(),
        envelope.data.runtime.release_version.as_str(),
        envelope.data.runtime.arch.as_str(),
        envelope.data.server.report_target.as_str(),
    ]
    .join("\n")
    .to_lowercase()
}

fn mark_stale_status(mut envelope: WsEnvelope<ClientStatus>) -> WsEnvelope<ClientStatus> {
    let now_ms = current_timestamp_ms();

    // Client 上报是轮询心跳模型，Server 不能长期相信旧快照仍然在线。
    // 输入：内存中最后一条状态信封。
    // 输出：查询结果中的 online 字段；历史记录不被改写。
    // 边界：客户端主动上报 offline 时保持 offline；本机时间早于上报时间时不误判离线。
    if envelope.data.online && is_stale_timestamp(envelope.timestamp_ms, now_ms) {
        envelope.data.online = false;
    }

    envelope
}

fn is_stale_timestamp(timestamp_ms: u128, now_ms: u128) -> bool {
    now_ms.saturating_sub(timestamp_ms) > CLIENT_ONLINE_STALE_MS
}

fn current_timestamp_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be later than UNIX_EPOCH")
        .as_millis()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};
    use tokio::sync::mpsc::unbounded_channel;

    #[test]
    fn state_keeps_latest_client_status() {
        let state = ServerState::default();
        let status = ClientStatus::new("client-a");
        let envelope = WsEnvelope::status("client-a", status);

        state
            .save_status(envelope.clone())
            .expect("status must save");

        assert_eq!(state.get_status("client-a"), Some(envelope));
        assert_eq!(state.get_status("missing"), None);
    }

    #[test]
    fn state_lists_client_statuses_in_stable_order() {
        let state = ServerState::default();
        state
            .save_status(WsEnvelope::status(
                "client-b",
                ClientStatus::new("client-b"),
            ))
            .expect("client-b must save");
        state
            .save_status(WsEnvelope::status(
                "client-a",
                ClientStatus::new("client-a"),
            ))
            .expect("client-a must save");

        let statuses = state.list_statuses();

        assert_eq!(statuses.len(), 2);
        assert_eq!(statuses[0].client_id, "client-a");
        assert_eq!(statuses[1].client_id, "client-b");
    }

    #[test]
    fn state_pages_and_filters_client_statuses() {
        let state = ServerState::default();

        for (client_id, group, tags) in [
            ("client-a", "raid-a", vec!["dm".to_string()]),
            ("client-b", "raid-a", vec!["farm".to_string()]),
            ("client-c", "raid-b", vec!["dm".to_string()]),
        ] {
            let mut status = ClientStatus::new(client_id);
            status.identity.group = group.to_string();
            status.identity.tags = tags;
            state
                .save_status(WsEnvelope::status(client_id, status))
                .expect("status must save");
        }

        let page = state.list_status_page(ClientStatusPageFilter {
            page: 1,
            page_size: 1,
            group: Some("raid-a".to_string()),
            tag: None,
            online: Some(true),
            search: None,
        });

        assert_eq!(page.total, 2);
        assert_eq!(page.total_pages, 2);
        assert_eq!(page.items.len(), 1);
        assert_eq!(page.items[0].client_id, "client-a");

        let page = state.list_status_page(ClientStatusPageFilter {
            page: 1,
            page_size: 10,
            group: None,
            tag: Some("dm".to_string()),
            online: None,
            search: None,
        });

        assert_eq!(page.total, 2);
    }

    #[test]
    fn state_marks_stale_latest_status_offline() {
        let state = ServerState::default();
        let mut envelope = WsEnvelope::status("client-a", ClientStatus::new("client-a"));
        envelope.timestamp_ms = current_timestamp_ms() - CLIENT_ONLINE_STALE_MS - 1;

        state.save_status(envelope).expect("status must save");

        let latest = state.get_status("client-a").expect("latest must exist");
        assert!(!latest.data.online);

        let statuses = state.list_statuses();
        assert!(!statuses[0].data.online);
    }

    #[test]
    fn state_keeps_recent_latest_status_online() {
        let state = ServerState::default();
        let mut envelope = WsEnvelope::status("client-a", ClientStatus::new("client-a"));
        envelope.timestamp_ms = current_timestamp_ms();

        state.save_status(envelope).expect("status must save");

        let latest = state.get_status("client-a").expect("latest must exist");
        assert!(latest.data.online);
    }

    #[test]
    fn state_keeps_bounded_client_history() {
        let state = ServerState::default();

        for index in 0..(CLIENT_HISTORY_LIMIT + 2) {
            let mut envelope = WsEnvelope::status("client-a", ClientStatus::new("client-a"));
            envelope.timestamp_ms = index as u128;
            envelope.message_id = format!("client-a-{index}");
            state.save_status(envelope).expect("status must save");
        }

        let history = state.get_history("client-a");

        assert_eq!(history.len(), CLIENT_HISTORY_LIMIT);
        assert_eq!(history[0].message_id, "client-a-2");
        assert_eq!(
            history[CLIENT_HISTORY_LIMIT - 1].message_id,
            format!("client-a-{}", CLIENT_HISTORY_LIMIT + 1)
        );
        assert!(state.get_history("missing").is_empty());
    }

    #[test]
    fn state_replays_persisted_history_on_startup() {
        let dir = unique_temp_dir("state-replay");
        let path = dir.join("status-history.jsonl");
        let state = ServerState::from_paths(Some(path.clone()), None).expect("state must open");

        for index in 0..3 {
            let mut envelope = WsEnvelope::status("client-a", ClientStatus::new("client-a"));
            envelope.timestamp_ms = 2000 + index;
            envelope.message_id = format!("persisted-{index}");
            state.save_status(envelope).expect("status must persist");
        }

        let reloaded = ServerState::from_paths(Some(path), None).expect("state must replay");
        let history = reloaded.get_history("client-a");

        assert_eq!(history.len(), 3);
        assert_eq!(history[0].message_id, "persisted-0");
        assert_eq!(
            reloaded
                .get_status("client-a")
                .expect("latest must replay")
                .message_id,
            "persisted-2"
        );

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn state_keeps_bounded_client_messages() {
        let state = ServerState::default();

        for index in 0..(CLIENT_MESSAGE_LIMIT + 2) {
            state.push_message(
                "client-a",
                ClientMessageRequest {
                    title: format!("title-{index}"),
                    body: "body".to_string(),
                },
            );
        }

        let messages = state.list_messages("client-a");

        assert_eq!(messages.len(), CLIENT_MESSAGE_LIMIT);
        assert_eq!(messages[0].title, "title-2");
        assert!(state.list_messages("missing").is_empty());
    }

    #[test]
    fn state_keeps_bounded_client_commands() {
        let state = ServerState::default();

        for index in 0..(CLIENT_COMMAND_LIMIT + 2) {
            state.push_command(
                "client-a",
                ClientCommandRequest {
                    command_type: "startup.status".to_string(),
                    payload: serde_json::json!({ "index": index }),
                },
            );
        }

        let commands = state.take_commands("client-a");

        assert_eq!(commands.len(), CLIENT_COMMAND_LIMIT);
        assert_eq!(commands[0].payload["index"], serde_json::json!(2));
        assert!(state.take_commands("client-a").is_empty());
        assert!(state.take_commands("missing").is_empty());
    }

    #[test]
    fn state_keeps_bounded_client_command_receipts() {
        let state = ServerState::default();

        for index in 0..(CLIENT_COMMAND_RECEIPT_LIMIT + 2) {
            state.push_command_receipt(
                "client-a",
                ClientCommandReceiptRequest {
                    command_id: format!("cmd-{index}"),
                    command_type: "startup.status".to_string(),
                    success: true,
                    summary: format!("summary-{index}"),
                },
            );
        }

        let receipts = state.list_command_receipts("client-a");

        assert_eq!(receipts.len(), CLIENT_COMMAND_RECEIPT_LIMIT);
        assert_eq!(receipts[0].command_id, "cmd-2");
        assert!(state.list_command_receipts("missing").is_empty());
    }

    #[test]
    fn state_records_recent_audit_events() {
        let state = ServerState::default();

        state.push_command(
            "client-a",
            ClientCommandRequest {
                command_type: "startup.status".to_string(),
                payload: serde_json::json!({}),
            },
        );
        state.push_message(
            "client-a",
            ClientMessageRequest {
                title: "hello".to_string(),
                body: "body".to_string(),
            },
        );

        let events = state.list_audit_events(10);

        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event_type, "message.created");
        assert_eq!(events[1].event_type, "command.created");
    }

    #[test]
    fn realtime_client_registration_does_not_remove_new_connection() {
        let state = ServerState::default();
        let (first_sender, _first_receiver) = unbounded_channel::<ServerRealtimeMessage>();
        let first_id = state.register_realtime_client("client-a", first_sender);
        let (second_sender, _second_receiver) = unbounded_channel::<ServerRealtimeMessage>();
        let second_id = state.register_realtime_client("client-a", second_sender);

        state.unregister_realtime_client("client-a", first_id);
        assert_eq!(state.list_realtime_clients(), vec!["client-a".to_string()]);

        state.unregister_realtime_client("client-a", second_id);
        assert!(state.list_realtime_clients().is_empty());
    }

    #[test]
    fn realtime_command_push_keeps_queue_until_receipt() {
        let state = ServerState::default();
        let (sender, mut receiver) = unbounded_channel::<ServerRealtimeMessage>();
        state.register_realtime_client("client-a", sender);

        let command = state.push_command(
            "client-a",
            ClientCommandRequest {
                command_type: "startup.status".to_string(),
                payload: serde_json::json!({}),
            },
        );

        let frame = receiver.try_recv().expect("command frame must arrive");
        match frame {
            ServerRealtimeMessage::Command { command: pushed } => {
                assert_eq!(pushed.id, command.id);
            }
            other => panic!("unexpected realtime frame: {other:?}"),
        }
        assert_eq!(state.pending_commands("client-a").len(), 1);

        state.push_command_receipt(
            "client-a",
            ClientCommandReceiptRequest {
                command_id: command.id,
                command_type: "startup.status".to_string(),
                success: true,
                summary: "ok".to_string(),
            },
        );

        assert!(state.pending_commands("client-a").is_empty());
    }

    #[test]
    fn realtime_admin_receives_status_events() {
        let state = ServerState::default();
        let (sender, mut receiver) = unbounded_channel::<AdminRealtimeMessage>();
        state.register_realtime_admin(sender);

        state
            .save_status(WsEnvelope::status(
                "client-a",
                ClientStatus::new("client-a"),
            ))
            .expect("status must save");

        let frame = receiver.try_recv().expect("admin frame must arrive");
        match frame {
            AdminRealtimeMessage::ClientStatus { status } => {
                assert_eq!(status.client_id, "client-a");
            }
            other => panic!("unexpected admin frame: {other:?}"),
        }
    }

    fn unique_temp_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock must be valid")
            .as_nanos();

        std::env::temp_dir().join(format!("wow-{name}-{}-{nanos}", std::process::id()))
    }
}
