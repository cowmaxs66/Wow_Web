use shared_types::{ClientStatus, WsEnvelope};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};

pub const CLIENT_HISTORY_LIMIT: usize = 50;

#[derive(Debug, Clone, Default)]
pub struct ServerState {
    clients: Arc<RwLock<HashMap<String, WsEnvelope<ClientStatus>>>>,
    histories: Arc<RwLock<HashMap<String, VecDeque<WsEnvelope<ClientStatus>>>>>,
}

impl ServerState {
    pub fn save_status(&self, envelope: WsEnvelope<ClientStatus>) {
        let client_id = envelope.client_id.clone();

        let mut clients = self.clients.write().expect("client status lock poisoned");
        // 最新状态仍然单独保存，保证 P3/P4/P7 的查询接口语义不变。
        // 输入：Client 上报的状态信封。
        // 输出：内存中的最新状态快照。
        // 边界：进程退出即丢失，历史持久化后续阶段单独设计。
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_keeps_latest_client_status() {
        let state = ServerState::default();
        let status = ClientStatus::new("client-a");
        let envelope = WsEnvelope::status("client-a", status);

        state.save_status(envelope.clone());

        assert_eq!(state.get_status("client-a"), Some(envelope));
        assert_eq!(state.get_status("missing"), None);
    }

    #[test]
    fn state_lists_client_statuses_in_stable_order() {
        let state = ServerState::default();
        state.save_status(WsEnvelope::status(
            "client-b",
            ClientStatus::new("client-b"),
        ));
        state.save_status(WsEnvelope::status(
            "client-a",
            ClientStatus::new("client-a"),
        ));

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
            state.save_status(envelope);
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
}
