use crate::embedded_web;
use crate::error::ApiError;
use crate::state::{CLIENT_HISTORY_LIMIT, ServerState};
use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use shared_types::{
    ClientCommand, ClientCommandList, ClientCommandRequest, ClientMessage, ClientMessageList,
    ClientMessageRequest, ClientStatus, ClientStatusHistory, HealthResponse, MessageType,
    StatusAck, WsEnvelope,
};
use std::path::PathBuf;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};

pub fn build_router_with_web_dir(state: ServerState, web_dir: Option<PathBuf>) -> Router {
    let router = Router::new()
        .route("/health", get(health))
        .route("/api/client/status", get(list_statuses).post(report_status))
        .route("/api/client/status/{client_id}", get(get_status))
        .route("/api/client/history/{client_id}", get(get_history))
        .route(
            "/api/client/messages/{client_id}",
            get(list_messages).post(push_message),
        )
        .route(
            "/api/client/commands/{client_id}",
            get(list_commands).post(push_command),
        )
        .with_state(state)
        // P4 只用于本机 Web Admin 开发联调。生产部署前必须改为明确来源白名单。
        .layer(CorsLayer::permissive());

    if let Some(web_dir) = web_dir {
        let index_path = web_dir.join("index.html");
        // P10 一键运行模式：Server 可直接托管 Web Admin 静态产物。
        // 输入：MANAGEMENT_SERVER_WEB_DIR 指向的 dist 目录。
        // 输出：未命中 API 的路径返回静态文件，SPA 路径回退到 index.html。
        // 边界：API 路由仍优先匹配；生产部署前仍需补鉴权和 CORS 白名单。
        return router.fallback_service(
            ServeDir::new(web_dir).not_found_service(ServeFile::new(index_path)),
        );
    }

    if embedded_web::has_assets() {
        // P11 单 exe 模式：release 构建时把 Web Admin dist 内嵌进 Server。
        // 输入：build.rs 在编译期生成的静态资源表。
        // 输出：直接运行 management-server.exe 即可访问 Web Admin。
        // 边界：外部 MANAGEMENT_SERVER_WEB_DIR 优先，便于开发期覆盖调试。
        return router.fallback(get(embedded_web::serve_embedded_web));
    }

    router
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse::ok())
}

async fn report_status(
    State(state): State<ServerState>,
    Json(envelope): Json<WsEnvelope<ClientStatus>>,
) -> Result<Json<StatusAck>, ApiError> {
    validate_status_envelope(&envelope)?;

    let previous = state.get_status(&envelope.client_id);
    let ack = StatusAck::accepted(envelope.client_id.clone(), envelope.message_id.clone());
    state
        .save_status(envelope.clone())
        .map_err(|error| ApiError::Internal(format!("failed to save client status: {error}")))?;
    log_status_report(&envelope, previous.as_ref());
    Ok(Json(ack))
}

async fn get_status(
    State(state): State<ServerState>,
    Path(client_id): Path<String>,
) -> Result<Json<WsEnvelope<ClientStatus>>, ApiError> {
    state
        .get_status(&client_id)
        .map(Json)
        .ok_or_else(|| ApiError::NotFound(format!("client status not found: {client_id}")))
}

async fn list_statuses(State(state): State<ServerState>) -> Json<Vec<WsEnvelope<ClientStatus>>> {
    Json(state.list_statuses())
}

async fn get_history(
    State(state): State<ServerState>,
    Path(client_id): Path<String>,
) -> Json<ClientStatusHistory> {
    Json(ClientStatusHistory::new(
        client_id.clone(),
        CLIENT_HISTORY_LIMIT,
        state.get_history(&client_id),
    ))
}

async fn push_message(
    State(state): State<ServerState>,
    Path(client_id): Path<String>,
    Json(request): Json<ClientMessageRequest>,
) -> Result<Json<ClientMessage>, ApiError> {
    validate_message_request(&client_id, &request)?;
    Ok(Json(state.push_message(&client_id, request)))
}

async fn list_messages(
    State(state): State<ServerState>,
    Path(client_id): Path<String>,
) -> Json<ClientMessageList> {
    Json(ClientMessageList::new(
        client_id.clone(),
        state.list_messages(&client_id),
    ))
}

async fn push_command(
    State(state): State<ServerState>,
    Path(client_id): Path<String>,
    Json(request): Json<ClientCommandRequest>,
) -> Result<Json<ClientCommand>, ApiError> {
    validate_command_request(&client_id, &request)?;
    Ok(Json(state.push_command(&client_id, request)))
}

async fn list_commands(
    State(state): State<ServerState>,
    Path(client_id): Path<String>,
) -> Json<ClientCommandList> {
    Json(ClientCommandList::new(
        client_id.clone(),
        state.take_commands(&client_id),
    ))
}

fn validate_status_envelope(envelope: &WsEnvelope<ClientStatus>) -> Result<(), ApiError> {
    if envelope.schema_version != 1 {
        return Err(ApiError::BadRequest(
            "unsupported schema_version".to_string(),
        ));
    }

    if envelope.message_type != MessageType::Status {
        return Err(ApiError::BadRequest(
            "message_type must be status".to_string(),
        ));
    }

    if envelope.client_id.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "client_id must not be empty".to_string(),
        ));
    }

    if envelope.client_id != envelope.data.client_id {
        return Err(ApiError::BadRequest(
            "envelope client_id must match data.client_id".to_string(),
        ));
    }

    Ok(())
}

fn validate_message_request(
    client_id: &str,
    request: &ClientMessageRequest,
) -> Result<(), ApiError> {
    if client_id.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "client_id must not be empty".to_string(),
        ));
    }

    if request.title.trim().is_empty() {
        return Err(ApiError::BadRequest("title must not be empty".to_string()));
    }

    if request.body.trim().is_empty() {
        return Err(ApiError::BadRequest("body must not be empty".to_string()));
    }

    if request.title.chars().count() > 80 {
        return Err(ApiError::BadRequest(
            "title must be 80 chars or fewer".to_string(),
        ));
    }

    if request.body.chars().count() > 1000 {
        return Err(ApiError::BadRequest(
            "body must be 1000 chars or fewer".to_string(),
        ));
    }

    Ok(())
}

fn validate_command_request(
    client_id: &str,
    request: &ClientCommandRequest,
) -> Result<(), ApiError> {
    if client_id.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "client_id must not be empty".to_string(),
        ));
    }

    if !is_allowed_command(&request.command_type) {
        return Err(ApiError::BadRequest(format!(
            "unsupported command_type: {}",
            request.command_type
        )));
    }

    Ok(())
}

fn is_allowed_command(command_type: &str) -> bool {
    matches!(
        command_type,
        "startup.status"
            | "startup.enable"
            | "startup.disable"
            | "service.status"
            | "service.install"
            | "service.start"
            | "service.stop"
            | "update.check"
            | "update.download"
            | "update.apply"
            | "settings.open"
            | "log.open"
            | "tray.open"
    )
}

fn log_status_report(
    envelope: &WsEnvelope<ClientStatus>,
    previous: Option<&WsEnvelope<ClientStatus>>,
) {
    let event = status_report_event(envelope, previous);
    let script = envelope.data.current_script.as_deref().unwrap_or("无");

    tracing::info!(
        client_id = %envelope.client_id,
        online = envelope.data.online,
        script = %script,
        release_version = %envelope.data.runtime.release_version,
        message_id = %envelope.message_id,
        "{}", event
    );
    // 控制台直接输出一行人能读的上线日志，避免只依赖 tracing 后用户看不到 Client 上线。
    // 输入：Client 最新状态信封和事件分类。
    // 输出：Server core 控制台中的上线/刷新/离线记录。
    // 边界：GUI launcher 没有控制台；需要查看这行日志时运行 bin/management-server-core.exe。
    println!(
        "[server] {event}: client_id={} online={} script={} release_version={} message_id={}",
        envelope.client_id,
        envelope.data.online,
        script,
        envelope.data.runtime.release_version,
        envelope.message_id
    );
}

fn status_report_event(
    envelope: &WsEnvelope<ClientStatus>,
    previous: Option<&WsEnvelope<ClientStatus>>,
) -> &'static str {
    if !envelope.data.online {
        return "Client 离线";
    }

    if previous.is_none_or(|status| !status.data.online) {
        return "Client 上线";
    }

    "Client 状态刷新"
}

#[cfg(test)]
mod tests {
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
        fs::write(web_dir.join("index.html"), "<main>WoW Control</main>")
            .expect("index must write");
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
}
