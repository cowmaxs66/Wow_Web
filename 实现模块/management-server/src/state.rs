use shared_types::{ClientStatus, WsEnvelope};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Default)]
pub struct ServerState {
    clients: Arc<RwLock<HashMap<String, WsEnvelope<ClientStatus>>>>,
}

impl ServerState {
    pub fn save_status(&self, envelope: WsEnvelope<ClientStatus>) {
        let client_id = envelope.client_id.clone();
        let mut clients = self.clients.write().expect("client status lock poisoned");

        // P3 只保存每个 Client 的最新状态，避免过早引入数据库和历史表。
        // 输入：Client 上报的状态信封。
        // 输出：内存中的最新状态快照。
        // 边界：进程退出即丢失，P4/P5 前再接入持久化。
        clients.insert(client_id, envelope);
    }

    pub fn get_status(&self, client_id: &str) -> Option<WsEnvelope<ClientStatus>> {
        self.clients
            .read()
            .expect("client status lock poisoned")
            .get(client_id)
            .cloned()
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
}
