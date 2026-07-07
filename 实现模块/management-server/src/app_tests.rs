use super::*;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use tower::ServiceExt;

#[tokio::test]
async fn status_report_can_be_queried_back() {
    let app = build_router_with_web_dir(ServerState::default(), None);
    let status = ClientStatus::new("client-a");
    let envelope = WsEnvelope::status("client-a", status);
    let body = serde_json::to_vec(&envelope).expect("status must serialize");

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/client/status")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/client/status/client-a")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let saved: WsEnvelope<ClientStatus> =
        serde_json::from_slice(&body).expect("saved status must deserialize");
    assert_eq!(saved.client_id, "client-a");
}

#[tokio::test]
async fn mismatched_client_id_is_rejected() {
    let app = build_router_with_web_dir(ServerState::default(), None);
    let mut envelope = WsEnvelope::status("client-a", ClientStatus::new("client-b"));
    envelope.client_id = "client-a".to_string();
    let body = serde_json::to_vec(&envelope).expect("status must serialize");

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/client/status")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn status_list_returns_all_latest_clients() {
    let app = build_router_with_web_dir(ServerState::default(), None);

    for client_id in ["client-b", "client-a"] {
        let envelope = WsEnvelope::status(client_id, ClientStatus::new(client_id));
        let body = serde_json::to_vec(&envelope).expect("status must serialize");

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/client/status")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/client/status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let statuses: Vec<WsEnvelope<ClientStatus>> =
        serde_json::from_slice(&body).expect("status list must deserialize");

    assert_eq!(statuses.len(), 2);
    assert_eq!(statuses[0].client_id, "client-a");
    assert_eq!(statuses[1].client_id, "client-b");
}

#[tokio::test]
async fn status_history_returns_samples_for_client() {
    let app = build_router_with_web_dir(ServerState::default(), None);

    for index in 0..3 {
        let mut envelope = WsEnvelope::status("client-a", ClientStatus::new("client-a"));
        envelope.timestamp_ms = 1000 + index;
        envelope.message_id = format!("client-a-{index}");
        let body = serde_json::to_vec(&envelope).expect("status must serialize");

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/client/status")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/client/history/client-a")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let history: ClientStatusHistory =
        serde_json::from_slice(&body).expect("history must deserialize");

    assert_eq!(history.client_id, "client-a");
    assert_eq!(history.limit, CLIENT_HISTORY_LIMIT);
    assert_eq!(history.total, 3);
    assert_eq!(history.items[0].message_id, "client-a-0");
    assert_eq!(history.items[2].message_id, "client-a-2");
}

#[tokio::test]
async fn static_web_dir_serves_index_when_configured() {
    let web_dir = unique_temp_dir("web-dir");
    fs::create_dir_all(&web_dir).expect("web dir must exist");
    fs::write(web_dir.join("index.html"), "<main>WoW Control</main>").expect("index must write");
    let app = build_router_with_web_dir(ServerState::default(), Some(web_dir.clone()));

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    assert!(String::from_utf8_lossy(&body).contains("WoW Control"));

    let _ = fs::remove_dir_all(web_dir);
}

#[tokio::test]
async fn client_message_can_be_created_and_listed() {
    let app = build_router_with_web_dir(ServerState::default(), None);
    let body = serde_json::to_vec(&ClientMessageRequest {
        title: "测试消息".to_string(),
        body: "hello client".to_string(),
    })
    .expect("message request must serialize");

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/client/messages/client-a")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/client/messages/client-a")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let messages: ClientMessageList =
        serde_json::from_slice(&body).expect("message list must deserialize");

    assert_eq!(messages.client_id, "client-a");
    assert_eq!(messages.total, 1);
    assert_eq!(messages.items[0].title, "测试消息");
}

#[tokio::test]
async fn empty_client_message_is_rejected() {
    let app = build_router_with_web_dir(ServerState::default(), None);
    let body = serde_json::to_vec(&ClientMessageRequest {
        title: "".to_string(),
        body: "hello".to_string(),
    })
    .expect("message request must serialize");

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/client/messages/client-a")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn client_command_can_be_created_and_taken() {
    let app = build_router_with_web_dir(ServerState::default(), None);
    let body = serde_json::to_vec(&ClientCommandRequest {
        command_type: "startup.status".to_string(),
        payload: serde_json::json!({}),
    })
    .expect("command request must serialize");

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/client/commands/client-a")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/client/commands/client-a")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let commands: ClientCommandList =
        serde_json::from_slice(&body).expect("command list must deserialize");

    assert_eq!(commands.client_id, "client-a");
    assert_eq!(commands.total, 1);
    assert_eq!(commands.items[0].command_type, "startup.status");

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/client/commands/client-a")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let empty: ClientCommandList =
        serde_json::from_slice(&body).expect("command list must deserialize");

    assert_eq!(empty.total, 0);
}

#[tokio::test]
async fn unsupported_client_command_is_rejected() {
    let app = build_router_with_web_dir(ServerState::default(), None);
    let body = serde_json::to_vec(&ClientCommandRequest {
        command_type: "shell.exec".to_string(),
        payload: serde_json::json!({}),
    })
    .expect("command request must serialize");

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/client/commands/client-a")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn update_apply_client_command_is_allowed() {
    let app = build_router_with_web_dir(ServerState::default(), None);
    let body = serde_json::to_vec(&ClientCommandRequest {
        command_type: "update.apply".to_string(),
        payload: serde_json::json!({}),
    })
    .expect("command request must serialize");

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/client/commands/client-a")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/client/commands/client-a")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let commands: ClientCommandList =
        serde_json::from_slice(&body).expect("command list must deserialize");

    assert_eq!(commands.total, 1);
    assert_eq!(commands.items[0].command_type, "update.apply");
}

#[tokio::test]
async fn client_command_receipt_can_be_created_and_listed() {
    let app = build_router_with_web_dir(ServerState::default(), None);
    let body = serde_json::to_vec(&ClientCommandReceiptRequest {
        command_id: "cmd-1".to_string(),
        command_type: "startup.status".to_string(),
        success: true,
        summary: "ok".to_string(),
    })
    .expect("receipt request must serialize");

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/client/command-receipts/client-a")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/client/command-receipts/client-a")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let receipts: ClientCommandReceiptList =
        serde_json::from_slice(&body).expect("receipt list must deserialize");

    assert_eq!(receipts.total, 1);
    assert_eq!(receipts.items[0].command_id, "cmd-1");
    assert!(receipts.items[0].success);
}

#[tokio::test]
async fn unsupported_command_receipt_is_rejected() {
    let app = build_router_with_web_dir(ServerState::default(), None);
    let body = serde_json::to_vec(&ClientCommandReceiptRequest {
        command_id: "cmd-1".to_string(),
        command_type: "shell.exec".to_string(),
        success: false,
        summary: "blocked".to_string(),
    })
    .expect("receipt request must serialize");

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/client/command-receipts/client-a")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn script_run_bootstrap_client_command_is_allowed() {
    let app = build_router_with_web_dir(ServerState::default(), None);
    let body = serde_json::to_vec(&ClientCommandRequest {
        command_type: "script.run_bootstrap".to_string(),
        payload: serde_json::json!({}),
    })
    .expect("command request must serialize");

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/client/commands/client-a")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/client/commands/client-a")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let commands: ClientCommandList =
        serde_json::from_slice(&body).expect("command list must deserialize");

    assert_eq!(commands.total, 1);
    assert_eq!(commands.items[0].command_type, "script.run_bootstrap");
}

#[test]
fn status_report_event_marks_first_online() {
    let current = WsEnvelope::status("client-a", ClientStatus::new("client-a"));

    assert_eq!(status_report_event(&current, None), "Client 上线");
}

#[test]
fn status_report_event_marks_refresh() {
    let previous = WsEnvelope::status("client-a", ClientStatus::new("client-a"));
    let current = WsEnvelope::status("client-a", ClientStatus::new("client-a"));

    assert_eq!(
        status_report_event(&current, Some(&previous)),
        "Client 状态刷新"
    );
}

#[test]
fn status_report_event_marks_offline() {
    let mut status = ClientStatus::new("client-a");
    status.online = false;
    let current = WsEnvelope::status("client-a", status);

    assert_eq!(status_report_event(&current, None), "Client 离线");
}

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock must be valid")
        .as_nanos();

    std::env::temp_dir().join(format!("wow-{name}-{}-{nanos}", std::process::id()))
}
