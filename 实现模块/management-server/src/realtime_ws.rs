use crate::error::ApiError;
use crate::state::ServerState;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use shared_types::{AdminRealtimeMessage, ClientRealtimeMessage, ServerRealtimeMessage};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

pub(crate) async fn client_ws(
    State(state): State<ServerState>,
    Path(client_id): Path<String>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_client_socket(state, client_id, socket))
}

pub(crate) async fn admin_ws(
    State(state): State<ServerState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_admin_socket(state, socket))
}

async fn handle_client_socket(state: ServerState, client_id: String, socket: WebSocket) {
    let (socket_sender, socket_receiver) = socket.split();
    let (server_sender, server_receiver) = unbounded_channel();
    let connection_id = state.register_realtime_client(client_id.clone(), server_sender.clone());

    let _ = server_sender.send(ServerRealtimeMessage::Ready {
        client_id: client_id.clone(),
    });
    for command in state.pending_commands(&client_id) {
        let _ = server_sender.send(ServerRealtimeMessage::Command { command });
    }

    tracing::info!(
        client_id = %client_id,
        connection_id,
        "Client WebSocket 已连接"
    );

    let write_task = tokio::spawn(write_server_frames(socket_sender, server_receiver));
    let read_task = tokio::spawn(read_client_frames(
        state.clone(),
        client_id.clone(),
        server_sender,
        socket_receiver,
    ));

    tokio::select! {
        _ = write_task => {}
        _ = read_task => {}
    }

    state.unregister_realtime_client(&client_id, connection_id);
    tracing::info!(
        client_id = %client_id,
        connection_id,
        "Client WebSocket 已断开"
    );
}

async fn handle_admin_socket(state: ServerState, socket: WebSocket) {
    let (socket_sender, mut socket_receiver) = socket.split();
    let (admin_sender, admin_receiver) = unbounded_channel();
    let connection_id = state.register_realtime_admin(admin_sender.clone());

    // Admin 连接后先补一份当前快照，再接收后续增量事件。
    // 输入：ServerState 内的最新 Client 状态和当前实时 Client ID。
    // 输出：Web Admin 能在 WS 建立后立即与 HTTP 视图保持一致。
    // 边界：前端仍会走 HTTP 防抖刷新，快照事件只做加速提示。
    for status in state.list_statuses() {
        let _ = admin_sender.send(AdminRealtimeMessage::ClientStatus {
            status: Box::new(status),
        });
    }
    for client_id in state.list_realtime_clients() {
        let _ = admin_sender.send(AdminRealtimeMessage::ClientConnected { client_id });
    }

    tracing::info!(
        connection_id,
        admins = state.realtime_admin_count(),
        "Admin WebSocket 已连接"
    );

    let write_task = tokio::spawn(write_admin_frames(socket_sender, admin_receiver));
    let read_task = tokio::spawn(async move {
        while let Some(result) = socket_receiver.next().await {
            match result {
                Ok(Message::Close(_)) => break,
                Ok(_) => {}
                Err(error) => {
                    tracing::debug!(error = %error, "Admin WebSocket 读取失败");
                    break;
                }
            }
        }
    });

    tokio::select! {
        _ = write_task => {}
        _ = read_task => {}
    }

    state.unregister_realtime_admin(connection_id);
    tracing::info!(connection_id, "Admin WebSocket 已断开");
}

async fn write_server_frames(
    mut socket_sender: SplitSink<WebSocket, Message>,
    mut server_receiver: UnboundedReceiver<ServerRealtimeMessage>,
) {
    while let Some(frame) = server_receiver.recv().await {
        if send_json_message(&mut socket_sender, &frame).await.is_err() {
            break;
        }
    }
}

async fn write_admin_frames(
    mut socket_sender: SplitSink<WebSocket, Message>,
    mut admin_receiver: UnboundedReceiver<AdminRealtimeMessage>,
) {
    while let Some(frame) = admin_receiver.recv().await {
        if send_json_message(&mut socket_sender, &frame).await.is_err() {
            break;
        }
    }
}

async fn read_client_frames(
    state: ServerState,
    route_client_id: String,
    server_sender: UnboundedSender<ServerRealtimeMessage>,
    mut socket_receiver: SplitStream<WebSocket>,
) {
    while let Some(result) = socket_receiver.next().await {
        let message = match result {
            Ok(message) => message,
            Err(error) => {
                tracing::debug!(client_id = %route_client_id, error = %error, "Client WebSocket 读取失败");
                break;
            }
        };

        match message {
            Message::Text(text) => {
                handle_client_text(&state, &route_client_id, &server_sender, text.as_str());
            }
            Message::Binary(bytes) => match std::str::from_utf8(bytes.as_ref()) {
                Ok(text) => handle_client_text(&state, &route_client_id, &server_sender, text),
                Err(error) => {
                    send_client_error(&server_sender, format!("二进制消息不是 UTF-8：{error}"))
                }
            },
            Message::Close(_) => break,
            Message::Ping(_) | Message::Pong(_) => {}
        }
    }
}

fn handle_client_text(
    state: &ServerState,
    route_client_id: &str,
    server_sender: &UnboundedSender<ServerRealtimeMessage>,
    text: &str,
) {
    let frame: ClientRealtimeMessage = match serde_json::from_str(text) {
        Ok(frame) => frame,
        Err(error) => {
            send_client_error(server_sender, format!("实时消息 JSON 无效：{error}"));
            return;
        }
    };

    match frame {
        ClientRealtimeMessage::Hello { client_id } => {
            if client_id != route_client_id {
                send_client_error(
                    server_sender,
                    format!("client_id 不匹配：route={route_client_id} payload={client_id}"),
                );
            }
        }
        ClientRealtimeMessage::CommandReceipt { client_id, receipt } => {
            if client_id != route_client_id {
                send_client_error(
                    server_sender,
                    format!("回执 client_id 不匹配：route={route_client_id} payload={client_id}"),
                );
                return;
            }

            if let Err(error) =
                super::app_validation::validate_command_receipt_request(route_client_id, &receipt)
            {
                send_client_error(server_sender, api_error_text(error));
                return;
            }

            let saved = state.push_command_receipt(route_client_id, receipt);
            tracing::info!(
                client_id = %route_client_id,
                command_id = %saved.command_id,
                success = saved.success,
                "Client WebSocket 命令回执已保存"
            );
        }
    }
}

fn send_client_error(sender: &UnboundedSender<ServerRealtimeMessage>, message: impl Into<String>) {
    let _ = sender.send(ServerRealtimeMessage::Error {
        message: message.into(),
    });
}

fn api_error_text(error: ApiError) -> String {
    match error {
        ApiError::BadRequest(message) => message,
        ApiError::NotFound(message) => message,
        ApiError::Internal(message) => message,
    }
}

async fn send_json_message<T: serde::Serialize>(
    socket_sender: &mut SplitSink<WebSocket, Message>,
    frame: &T,
) -> Result<(), axum::Error> {
    let payload = match serde_json::to_string(frame) {
        Ok(payload) => payload,
        Err(error) => {
            tracing::error!(error = %error, "WebSocket frame 序列化失败");
            return Ok(());
        }
    };
    socket_sender.send(Message::Text(payload.into())).await
}
