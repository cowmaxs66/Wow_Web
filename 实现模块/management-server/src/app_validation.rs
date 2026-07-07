use crate::error::ApiError;
use shared_types::{
    ClientCommandReceiptRequest, ClientCommandRequest, ClientConfigPatch, ClientMessageRequest,
    ClientStatus, MessageType, REMOTE_COMMAND_CONFIG_APPLY, WsEnvelope,
    is_supported_remote_command,
};

const CONFIG_APPLY_PAYLOAD_LIMIT: usize = 4_000;

pub(super) fn validate_status_envelope(
    envelope: &WsEnvelope<ClientStatus>,
) -> Result<(), ApiError> {
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

pub(super) fn validate_message_request(
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

pub(super) fn validate_command_request(
    client_id: &str,
    request: &ClientCommandRequest,
) -> Result<(), ApiError> {
    if client_id.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "client_id must not be empty".to_string(),
        ));
    }

    if !is_supported_remote_command(&request.command_type) {
        return Err(ApiError::BadRequest(format!(
            "unsupported command_type: {}",
            request.command_type
        )));
    }

    if request.command_type == REMOTE_COMMAND_CONFIG_APPLY {
        validate_config_apply_payload(&request.payload)?;
    }

    Ok(())
}

fn validate_config_apply_payload(payload: &serde_json::Value) -> Result<(), ApiError> {
    let payload_len = payload.to_string().chars().count();
    if payload_len > CONFIG_APPLY_PAYLOAD_LIMIT {
        return Err(ApiError::BadRequest(
            "config.apply payload must be 4000 chars or fewer".to_string(),
        ));
    }

    let patch: ClientConfigPatch = serde_json::from_value(payload.clone()).map_err(|error| {
        ApiError::BadRequest(format!("config.apply payload is invalid: {error}"))
    })?;

    if patch.is_empty() {
        return Err(ApiError::BadRequest(
            "config.apply payload must contain at least one setting".to_string(),
        ));
    }

    if patch
        .client
        .display_name
        .as_deref()
        .is_some_and(|value| value.trim().is_empty())
    {
        return Err(ApiError::BadRequest(
            "client.display_name must not be empty".to_string(),
        ));
    }

    if patch
        .client
        .group
        .as_deref()
        .is_some_and(|value| value.trim().is_empty())
    {
        return Err(ApiError::BadRequest(
            "client.group must not be empty".to_string(),
        ));
    }

    Ok(())
}

pub(super) fn validate_command_receipt_request(
    client_id: &str,
    request: &ClientCommandReceiptRequest,
) -> Result<(), ApiError> {
    if client_id.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "client_id must not be empty".to_string(),
        ));
    }

    if request.command_id.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "command_id must not be empty".to_string(),
        ));
    }

    if !is_supported_remote_command(&request.command_type) {
        return Err(ApiError::BadRequest(format!(
            "unsupported command_type: {}",
            request.command_type
        )));
    }

    if request.summary.chars().count() > 2000 {
        return Err(ApiError::BadRequest(
            "summary must be 2000 chars or fewer".to_string(),
        ));
    }

    Ok(())
}
