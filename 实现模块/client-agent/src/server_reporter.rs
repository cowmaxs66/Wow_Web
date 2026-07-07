use crate::config::ServerConfig;
mod error;
mod response;

pub use error::ServerReportError;
use response::{parse_json_response, parse_status_ack};
use shared_types::{
    ClientCommandList, ClientCommandReceipt, ClientCommandReceiptRequest, ClientMessageList,
    ClientStatus, ClientSyncRequest, ClientSyncResponse, StatusAck, WsEnvelope,
};
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

pub struct StatusReporter {
    config: ServerConfig,
}

impl StatusReporter {
    pub fn new(config: ServerConfig) -> Self {
        Self { config }
    }

    pub fn report_status(
        &self,
        envelope: &WsEnvelope<ClientStatus>,
    ) -> Result<StatusAck, ServerReportError> {
        let body = serde_json::to_string(envelope)
            .map_err(|error| ServerReportError::SerializeFailed(error.to_string()))?;
        let response = self.send_http("POST", &self.config.status_path, Some(&body))?;
        parse_status_ack(&response)
    }

    pub fn fetch_messages(&self, client_id: &str) -> Result<ClientMessageList, ServerReportError> {
        let path = format!("/api/client/messages/{client_id}");
        let response = self.send_http("GET", &path, None)?;
        parse_json_response(&response)
    }

    pub fn fetch_commands(&self, client_id: &str) -> Result<ClientCommandList, ServerReportError> {
        let path = format!("/api/client/commands/{client_id}");
        let response = self.send_http("GET", &path, None)?;
        parse_json_response(&response)
    }

    pub fn sync_client(
        &self,
        envelope: &WsEnvelope<ClientStatus>,
    ) -> Result<ClientSyncResponse, ServerReportError> {
        let request = ClientSyncRequest {
            status: envelope.clone(),
        };
        let body = serde_json::to_string(&request)
            .map_err(|error| ServerReportError::SerializeFailed(error.to_string()))?;
        let response = self.send_http("POST", "/api/client/sync", Some(&body))?;
        parse_json_response(&response)
    }

    pub fn report_command_receipt(
        &self,
        client_id: &str,
        request: &ClientCommandReceiptRequest,
    ) -> Result<ClientCommandReceipt, ServerReportError> {
        let body = serde_json::to_string(request)
            .map_err(|error| ServerReportError::SerializeFailed(error.to_string()))?;
        let path = format!("/api/client/command-receipts/{client_id}");
        let response = self.send_http("POST", &path, Some(&body))?;
        parse_json_response(&response)
    }

    fn send_http(
        &self,
        method: &str,
        path: &str,
        body: Option<&str>,
    ) -> Result<String, ServerReportError> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        let target = addr
            .to_socket_addrs()
            .map_err(|error| ServerReportError::ResolveFailed(error.to_string()))?
            .next()
            .ok_or_else(|| ServerReportError::ResolveFailed("未解析到可用地址".to_string()))?;
        let timeout = Duration::from_millis(self.config.connect_timeout_ms);
        let mut stream = TcpStream::connect_timeout(&target, timeout)
            .map_err(|error| ServerReportError::ConnectFailed(error.to_string()))?;
        stream
            .set_read_timeout(Some(timeout))
            .map_err(|error| ServerReportError::IoFailed(error.to_string()))?;

        // P3 使用最小 HTTP/1.1 上报，避免为了单次本机状态上报引入复杂客户端层。
        // 输入：WsEnvelope<ClientStatus> JSON。
        // 输出：Server 返回的 HTTP 响应文本。
        // 边界：仅支持本机明文 HTTP，HTTPS/代理/鉴权后续阶段再做。
        let body = body.unwrap_or("");
        let request = if body.is_empty() {
            format!(
                "GET {path} HTTP/1.1\r\nHost: {addr}\r\nAccept: application/json\r\nConnection: close\r\n\r\n"
            )
        } else {
            format!(
                "{method} {path} HTTP/1.1\r\nHost: {addr}\r\nContent-Type: application/json\r\nAccept: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            )
        };
        stream
            .write_all(request.as_bytes())
            .map_err(|error| ServerReportError::IoFailed(error.to_string()))?;

        let mut response = String::new();
        stream
            .read_to_string(&mut response)
            .map_err(|error| ServerReportError::IoFailed(error.to_string()))?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::thread;

    #[test]
    fn status_reporter_posts_status_and_reads_ack() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("listener must bind");
        let port = listener.local_addr().unwrap().port();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("request must arrive");
            let mut buffer = [0u8; 2048];
            let read_len = stream.read(&mut buffer).expect("request must read");
            let request = String::from_utf8_lossy(&buffer[..read_len]);
            assert!(request.contains("POST /api/client/status HTTP/1.1"));

            let body = r#"{"accepted":true,"client_id":"client-a","message_id":"msg-1"}"#;
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream
                .write_all(response.as_bytes())
                .expect("response must write");
        });
        let config = ServerConfig {
            enabled: true,
            host: "127.0.0.1".to_string(),
            port,
            status_path: "/api/client/status".to_string(),
            connect_timeout_ms: 1000,
        };
        let mut envelope = WsEnvelope::status("client-a", ClientStatus::new("client-a"));
        envelope.message_id = "msg-1".to_string();

        let ack = StatusReporter::new(config)
            .report_status(&envelope)
            .expect("status report must succeed");

        assert!(ack.accepted);
        assert_eq!(ack.client_id, "client-a");
        assert_eq!(ack.message_id, "msg-1");
        server.join().expect("server thread must finish");
    }

    #[test]
    fn status_reporter_posts_command_receipt() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("listener must bind");
        let port = listener.local_addr().unwrap().port();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("request must arrive");
            let mut buffer = [0u8; 4096];
            let read_len = stream.read(&mut buffer).expect("request must read");
            let request = String::from_utf8_lossy(&buffer[..read_len]);
            assert!(request.contains("POST /api/client/command-receipts/client-a HTTP/1.1"));
            assert!(request.contains(r#""command_id":"cmd-1""#));

            let body = r#"{"id":"receipt-1","client_id":"client-a","timestamp_ms":1,"command_id":"cmd-1","command_type":"startup.status","success":true,"summary":"ok"}"#;
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream
                .write_all(response.as_bytes())
                .expect("response must write");
        });
        let config = ServerConfig {
            enabled: true,
            host: "127.0.0.1".to_string(),
            port,
            status_path: "/api/client/status".to_string(),
            connect_timeout_ms: 1000,
        };

        let receipt = StatusReporter::new(config)
            .report_command_receipt(
                "client-a",
                &ClientCommandReceiptRequest {
                    command_id: "cmd-1".to_string(),
                    command_type: "startup.status".to_string(),
                    success: true,
                    summary: "ok".to_string(),
                },
            )
            .expect("receipt report must succeed");

        assert_eq!(receipt.id, "receipt-1");
        assert_eq!(receipt.command_id, "cmd-1");
        assert!(receipt.success);
        server.join().expect("server thread must finish");
    }

    #[test]
    fn status_reporter_posts_sync_request() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("listener must bind");
        let port = listener.local_addr().unwrap().port();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("request must arrive");
            let mut buffer = [0u8; 4096];
            let read_len = stream.read(&mut buffer).expect("request must read");
            let request = String::from_utf8_lossy(&buffer[..read_len]);
            assert!(request.contains("POST /api/client/sync HTTP/1.1"));
            assert!(request.contains(r#""status":"#));

            let body = r#"{"ack":{"accepted":true,"client_id":"client-a","message_id":"msg-1"},"messages":{"client_id":"client-a","total":0,"items":[]},"commands":{"client_id":"client-a","total":0,"items":[]}}"#;
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream
                .write_all(response.as_bytes())
                .expect("response must write");
        });
        let config = ServerConfig {
            enabled: true,
            host: "127.0.0.1".to_string(),
            port,
            status_path: "/api/client/status".to_string(),
            connect_timeout_ms: 1000,
        };
        let mut envelope = WsEnvelope::status("client-a", ClientStatus::new("client-a"));
        envelope.message_id = "msg-1".to_string();

        let response = StatusReporter::new(config)
            .sync_client(&envelope)
            .expect("sync must succeed");

        assert!(response.ack.accepted);
        assert_eq!(response.messages.total, 0);
        assert_eq!(response.commands.total, 0);
        server.join().expect("server thread must finish");
    }
}
