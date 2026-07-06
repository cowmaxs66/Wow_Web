use shared_types::{ClientStatus, WsEnvelope};

fn main() {
    let client_id = "local-dev-client";
    let status = ClientStatus::new(client_id);
    let envelope = WsEnvelope::status(client_id, status);

    // 当前阶段只输出标准 JSON，验证协议类型与客户端入口可运行。
    // 输入：本地固定 client_id。
    // 输出：符合 shared-types 的状态消息。
    // 边界：P1 接入配置文件后，client_id 不再硬编码。
    let json = serde_json::to_string_pretty(&envelope).expect("status envelope must serialize");
    println!("{json}");
}
