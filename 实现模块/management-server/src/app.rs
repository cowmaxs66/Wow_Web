use crate::embedded_web;
use crate::error::ApiError;
use crate::state::{CLIENT_HISTORY_LIMIT, ServerState};
#[path = "app_validation.rs"]
mod app_validation;
use app_validation::{
    validate_command_receipt_request, validate_command_request, validate_message_request,
    validate_status_envelope,
};
use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use shared_types::{
    ClientCommand, ClientCommandList, ClientCommandReceipt, ClientCommandReceiptList,
    ClientCommandReceiptRequest, ClientCommandRequest, ClientMessage, ClientMessageList,
    ClientMessageRequest, ClientStatus, ClientStatusHistory, ClientSyncRequest, ClientSyncResponse,
    HealthResponse, StatusAck, WsEnvelope,
};
use std::path::PathBuf;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};

pub fn build_router_with_web_dir(state: ServerState, web_dir: Option<PathBuf>) -> Router {
    let router = Router::new()
        .route("/health", get(health))
        .route("/api/client/status", get(list_statuses).post(report_status))
        .route("/api/client/sync", axum::routing::post(sync_client))
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
        .route(
            "/api/client/command-receipts/{client_id}",
            get(list_command_receipts).post(push_command_receipt),
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

async fn sync_client(
    State(state): State<ServerState>,
    Json(request): Json<ClientSyncRequest>,
) -> Result<Json<ClientSyncResponse>, ApiError> {
    validate_status_envelope(&request.status)?;

    let envelope = request.status;
    let client_id = envelope.client_id.clone();
    let previous = state.get_status(&client_id);
    let ack = StatusAck::accepted(client_id.clone(), envelope.message_id.clone());
    state
        .save_status(envelope.clone())
        .map_err(|error| ApiError::Internal(format!("failed to save client status: {error}")))?;
    log_status_report(&envelope, previous.as_ref());

    // P30 合并同步把 monitor 的三次 HTTP 往返收敛为一次。
    // 输入：Client 最新状态。
    // 输出：状态 ACK、当前消息列表、取出的命令列表。
    // 边界：消息保持旧接口 list 语义；命令保持 take 语义，避免同一命令重复执行。
    let messages = ClientMessageList::new(client_id.clone(), state.list_messages(&client_id));
    let commands = ClientCommandList::new(client_id.clone(), state.take_commands(&client_id));
    Ok(Json(ClientSyncResponse::new(ack, messages, commands)))
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

async fn push_command_receipt(
    State(state): State<ServerState>,
    Path(client_id): Path<String>,
    Json(request): Json<ClientCommandReceiptRequest>,
) -> Result<Json<ClientCommandReceipt>, ApiError> {
    validate_command_receipt_request(&client_id, &request)?;
    Ok(Json(state.push_command_receipt(&client_id, request)))
}

async fn list_command_receipts(
    State(state): State<ServerState>,
    Path(client_id): Path<String>,
) -> Json<ClientCommandReceiptList> {
    Json(ClientCommandReceiptList::new(
        client_id.clone(),
        state.list_command_receipts(&client_id),
    ))
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
#[path = "app_tests.rs"]
mod tests;
