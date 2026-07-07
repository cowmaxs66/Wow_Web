use crate::error::ApiError;
use shared_types::{
    ClientCommandReceiptRequest, ClientCommandRequest, ClientMessageRequest, ClientStatus,
    MessageType, WsEnvelope, is_supported_remote_command,
};

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
