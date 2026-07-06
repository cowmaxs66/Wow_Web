use crate::error::ApiError;
use crate::state::ServerState;
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use shared_types::{ClientStatus, HealthResponse, MessageType, StatusAck, WsEnvelope};

pub fn build_router(state: ServerState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/client/status", post(report_status))
        .route("/api/client/status/{client_id}", get(get_status))
        .with_state(state)
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse::ok())
}

async fn report_status(
    State(state): State<ServerState>,
    Json(envelope): Json<WsEnvelope<ClientStatus>>,
) -> Result<Json<StatusAck>, ApiError> {
    validate_status_envelope(&envelope)?;

    let ack = StatusAck::accepted(envelope.client_id.clone(), envelope.message_id.clone());
    state.save_status(envelope);
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Body, to_bytes};
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn status_report_can_be_queried_back() {
        let app = build_router(ServerState::default());
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
        let app = build_router(ServerState::default());
        let mut envelope = WsEnvelope::status("client-a", ClientStatus::new("client-b"));
        envelope.client_id = "client-a".to_string();
        let body = serde_json::to_vec(&envelope).expect("status must serialize");

        let response = app
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
}
