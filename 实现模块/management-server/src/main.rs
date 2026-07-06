use shared_types::{ClientStatus, WsEnvelope};

fn main() {
    let sample_status = ClientStatus::new("server-contract-check");
    let envelope = WsEnvelope::status("server-contract-check", sample_status);

    // 服务端在 P0 阶段只复用协议类型，确认 Client/Server 使用同一份契约。
    // 输入：开发期样例状态。
    // 输出：可序列化的服务端契约检查消息。
    // 边界：P3 接入 Axum 后，此入口会替换为真实 HTTP/WebSocket 服务。
    let json = serde_json::to_string(&envelope).expect("server contract sample must serialize");
    println!("{json}");
}
