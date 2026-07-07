use crate::persistence::{HistoryPersistence, PersistenceError};
use shared_types::{ClientMessage, ClientMessageRequest, ClientStatus, WsEnvelope};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub const CLIENT_HISTORY_LIMIT: usize = 50;
pub const CLIENT_MESSAGE_LIMIT: usize = 100;

#[derive(Debug, Clone, Default)]
pub struct ServerState {
    clients: Arc<RwLock<HashMap<String, WsEnvelope<ClientStatus>>>>,
    histories: Arc<RwLock<HashMap<String, VecDeque<WsEnvelope<ClientStatus>>>>>,
    messages: Arc<RwLock<HashMap<String, VecDeque<ClientMessage>>>>,
    persistence: Option<HistoryPersistence>,
}

impl ServerState {
    pub fn with_persistence(path: PathBuf) -> Result<Self, PersistenceError> {
        let persistence = HistoryPersistence::open(path.clone())?;
        let state = Self {
            persistence: Some(persistence),
            ..Self::default()
        };

        for envelope in HistoryPersistence::load(&path)? {
            state.save_status_in_memory(envelope);
        }

        Ok(state)
    }

    pub fn save_status(&self, envelope: WsEnvelope<ClientStatus>) -> Result<(), PersistenceError> {
        if let Some(persistence) = &self.persistence {
            persistence.append(&envelope)?;
        }

        self.save_status_in_memory(envelope);
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
    }

    pub fn list_statuses(&self) -> Vec<WsEnvelope<ClientStatus>> {
        let clients = self.clients.read().expect("client status lock poisoned");
        let mut statuses: Vec<_> = clients.values().cloned().collect();

        // 列表输出按 client_id 排序，保证 Web 管理端和测试看到稳定顺序。
        // 输入：内存中的最新状态 HashMap。
        // 输出：按 client_id 升序排列的状态数组。
        // 边界：P4 不返回历史记录，只返回每个 Client 最后一条状态。
        statuses.sort_by(|left, right| left.client_id.cmp(&right.client_id));
        statuses
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

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
        let state = ServerState::with_persistence(path.clone()).expect("state must open");

        for index in 0..3 {
            let mut envelope = WsEnvelope::status("client-a", ClientStatus::new("client-a"));
            envelope.timestamp_ms = 2000 + index;
            envelope.message_id = format!("persisted-{index}");
            state.save_status(envelope).expect("status must persist");
        }

        let reloaded = ServerState::with_persistence(path).expect("state must replay");
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

    fn unique_temp_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock must be valid")
            .as_nanos();

        std::env::temp_dir().join(format!("wow-{name}-{}-{nanos}", std::process::id()))
    }
}
