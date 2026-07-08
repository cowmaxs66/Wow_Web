use crate::config::{AgentConfig, default_config_path};
use crate::local_log::LocalLog;
use crate::monitor::{SharedSeenCommands, execute_command_to_receipt, mark_command_seen};
use crate::notifier;
use shared_types::{ClientMessage, ClientRealtimeMessage, ServerRealtimeMessage};
use std::collections::HashSet;
use std::error::Error;
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use tungstenite::client::IntoClientRequest;
use tungstenite::{Error as WsError, Message, WebSocket};

const RECONNECT_DELAY: Duration = Duration::from_secs(3);
const READ_TICK: Duration = Duration::from_millis(500);

pub(crate) struct RealtimeWorker {
    running: Arc<AtomicBool>,
    handle: thread::JoinHandle<()>,
}

impl RealtimeWorker {
    pub(crate) fn start(config: AgentConfig, seen_commands: SharedSeenCommands) -> Option<Self> {
        if !config.server.enabled {
            return None;
        }

        let running = Arc::new(AtomicBool::new(true));
        let worker_running = Arc::clone(&running);
        let handle = thread::spawn(move || {
            run_realtime_loop(config, seen_commands, worker_running);
        });

        Some(Self { running, handle })
    }

    pub(crate) fn stop(self) {
        self.running.store(false, Ordering::Relaxed);
        let _ = self.handle.join();
    }
}

fn run_realtime_loop(
    config: AgentConfig,
    seen_commands: SharedSeenCommands,
    running: Arc<AtomicBool>,
) {
    let log = LocalLog::default();
    let mut seen_messages = HashSet::new();
    let _ = log.append_event(&format!(
        "Client WebSocket 实时通道启动：url={}",
        build_client_ws_url(&config)
    ));

    while running.load(Ordering::Relaxed) {
        match connect_once(&config, &seen_commands, &mut seen_messages, &log, &running) {
            Ok(()) => {}
            Err(error) => {
                let _ = log.append_event(&format!("Client WebSocket 实时通道断开：{error}"));
            }
        }

        sleep_reconnect_delay(&running);
    }

    let _ = log.append_event("Client WebSocket 实时通道已停止");
}

fn connect_once(
    config: &AgentConfig,
    seen_commands: &SharedSeenCommands,
    seen_messages: &mut HashSet<String>,
    log: &LocalLog,
    running: &Arc<AtomicBool>,
) -> Result<(), Box<dyn Error>> {
    let url = build_client_ws_url(config);
    let timeout = Duration::from_millis(config.server.connect_timeout_ms.max(500));
    let address = server_socket_address(config);
    let target = address
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| format!("未解析到可用 Server 地址：{address}"))?;
    let stream = TcpStream::connect_timeout(&target, timeout)?;
    stream.set_read_timeout(Some(READ_TICK))?;
    stream.set_write_timeout(Some(timeout))?;

    let request = url.as_str().into_client_request()?;
    let (mut socket, _) = tungstenite::client(request, stream)?;
    let _ = log.append_event(&format!("Client WebSocket 已连接：url={url}"));
    send_client_frame(
        &mut socket,
        &ClientRealtimeMessage::Hello {
            client_id: config.client.id.clone(),
        },
    )?;

    while running.load(Ordering::Relaxed) {
        match socket.read() {
            Ok(Message::Text(text)) => {
                handle_server_text(
                    &mut socket,
                    config,
                    seen_commands,
                    seen_messages,
                    log,
                    text.as_str(),
                )?;
            }
            Ok(Message::Binary(bytes)) => {
                let text = std::str::from_utf8(bytes.as_ref())?;
                handle_server_text(&mut socket, config, seen_commands, seen_messages, log, text)?;
            }
            Ok(Message::Ping(payload)) => socket.send(Message::Pong(payload))?,
            Ok(Message::Pong(_)) => {}
            Ok(Message::Close(_)) => break,
            Ok(Message::Frame(_)) => {}
            Err(WsError::Io(error))
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) => {}
            Err(error) => return Err(Box::new(error)),
        }
    }

    let _ = socket.close(None);
    Ok(())
}

fn handle_server_text(
    socket: &mut WebSocket<TcpStream>,
    config: &AgentConfig,
    seen_commands: &SharedSeenCommands,
    seen_messages: &mut HashSet<String>,
    log: &LocalLog,
    text: &str,
) -> Result<(), Box<dyn Error>> {
    let frame: ServerRealtimeMessage = serde_json::from_str(text)?;
    match frame {
        ServerRealtimeMessage::Ready { client_id } => {
            let _ = log.append_event(&format!("Client WebSocket 已就绪：client_id={client_id}"));
        }
        ServerRealtimeMessage::Command { command } => {
            if command.client_id != config.client.id {
                let _ = log.append_event(&format!(
                    "忽略非本机 WebSocket 命令：expected={} actual={} command_id={}",
                    config.client.id, command.client_id, command.id
                ));
                return Ok(());
            }

            if !mark_command_seen(seen_commands, &command.id) {
                let _ = log.append_event(&format!(
                    "忽略重复 WebSocket 命令：id={} type={}",
                    command.id, command.command_type
                ));
                return Ok(());
            }

            // 命令执行前重读配置，保证 Server 刚下发的 config.apply 或本机 UI 修改能尽快生效。
            // 输入：默认 client-agent.toml 路径。
            // 输出：最新可用配置；读取失败则使用建立 WS 时的稳定配置。
            // 边界：回执仍使用当前连接的 client_id，避免执行中途改 ID 导致 Server 拒收。
            let command_config = AgentConfig::load_from_path(default_config_path())
                .unwrap_or_else(|_| config.clone());
            let receipt = execute_command_to_receipt(&command, &command_config, log)?;
            let success = receipt.success;
            send_client_frame(
                socket,
                &ClientRealtimeMessage::CommandReceipt {
                    client_id: config.client.id.clone(),
                    receipt,
                },
            )?;
            let _ = log.append_event(&format!(
                "WebSocket 命令回执已发送：command_id={} success={}",
                command.id, success
            ));
        }
        ServerRealtimeMessage::Message { message } => {
            handle_realtime_message(message, seen_messages, log)?;
        }
        ServerRealtimeMessage::Error { message } => {
            let _ = log.append_event(&format!("Server WebSocket 错误：{message}"));
        }
    }

    Ok(())
}

fn handle_realtime_message(
    message: ClientMessage,
    seen_messages: &mut HashSet<String>,
    log: &LocalLog,
) -> Result<(), Box<dyn Error>> {
    if !seen_messages.insert(message.id.clone()) {
        return Ok(());
    }

    log.append_event(&format!(
        "收到 Server WebSocket 消息：id={} title={} body={}",
        message.id, message.title, message.body
    ))?;
    let _ = notifier::notify(&message.title, &message.body);
    Ok(())
}

fn send_client_frame(
    socket: &mut WebSocket<TcpStream>,
    frame: &ClientRealtimeMessage,
) -> Result<(), Box<dyn Error>> {
    let payload = serde_json::to_string(frame)?;
    socket.send(Message::Text(payload.into()))?;
    Ok(())
}

fn sleep_reconnect_delay(running: &Arc<AtomicBool>) {
    let mut slept = Duration::ZERO;
    while running.load(Ordering::Relaxed) && slept < RECONNECT_DELAY {
        thread::sleep(READ_TICK);
        slept += READ_TICK;
    }
}

fn server_socket_address(config: &AgentConfig) -> String {
    format!(
        "{}:{}",
        normalized_server_host(&config.server.host),
        config.server.port
    )
}

pub(crate) fn build_client_ws_url(config: &AgentConfig) -> String {
    let host = normalized_server_host(&config.server.host);
    let display_host = if host.contains(':') && !host.starts_with('[') {
        format!("[{host}]")
    } else {
        host
    };
    format!(
        "ws://{}:{}/ws/client/{}",
        display_host,
        config.server.port,
        encode_path_segment(&config.client.id)
    )
}

fn normalized_server_host(host: &str) -> String {
    let trimmed = host.trim().trim_end_matches('/');
    let without_scheme = trimmed
        .strip_prefix("http://")
        .or_else(|| trimmed.strip_prefix("https://"))
        .unwrap_or(trimmed);
    without_scheme
        .split('/')
        .next()
        .unwrap_or(without_scheme)
        .to_string()
}

fn encode_path_segment(value: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut encoded = String::new();
    for byte in value.as_bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            encoded.push(char::from(*byte));
        } else {
            encoded.push('%');
            encoded.push(char::from(HEX[(byte >> 4) as usize]));
            encoded.push(char::from(HEX[(byte & 0x0F) as usize]));
        }
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ClientConfig, DmConfig, LuaConfig, ScriptSecurityConfig, ServerConfig};
    use std::path::PathBuf;

    #[test]
    fn client_ws_url_uses_configured_server_and_encoded_client_id() {
        let mut config = test_config();
        config.server.host = "http://127.0.0.1".to_string();
        config.server.port = 18080;
        config.client.id = "client a/1".to_string();

        assert_eq!(
            build_client_ws_url(&config),
            "ws://127.0.0.1:18080/ws/client/client%20a%2F1"
        );
    }

    fn test_config() -> AgentConfig {
        AgentConfig {
            client: ClientConfig {
                id: "local-dev-client".to_string(),
                display_name: "Local Dev Client".to_string(),
                group: "default".to_string(),
                tags: Vec::new(),
            },
            lua: LuaConfig {
                enabled: true,
                bootstrap_name: "bootstrap".to_string(),
                bootstrap_path: PathBuf::from("scripts/bootstrap.lua"),
                instruction_limit: 10_000,
            },
            script_security: ScriptSecurityConfig {
                enabled: false,
                manifest_path: PathBuf::from("scripts/bootstrap.manifest.json"),
                trusted_signer_public_key:
                    "1111111111111111111111111111111111111111111111111111111111111111".to_string(),
                allowed_permissions: vec!["host.log".to_string(), "dm.access".to_string()],
            },
            dm: DmConfig {
                bridge_path: PathBuf::from("dm-bridge/DmBridge.dll"),
            },
            server: ServerConfig {
                enabled: true,
                host: "127.0.0.1".to_string(),
                port: 18080,
                status_path: "/api/client/status".to_string(),
                connect_timeout_ms: 3000,
            },
        }
    }
}
