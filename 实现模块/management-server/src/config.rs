use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerConfig {
    pub bind_addr: SocketAddr,
    pub history_path: Option<PathBuf>,
}

impl ServerConfig {
    pub fn from_env() -> Result<Self, std::net::AddrParseError> {
        let bind_text = std::env::var("MANAGEMENT_SERVER_BIND")
            .unwrap_or_else(|_| "127.0.0.1:18080".to_string());
        let history_path =
            history_path_from_value(std::env::var("MANAGEMENT_SERVER_HISTORY_PATH").ok());

        // P3 只支持本机开发期绑定地址，避免一开始暴露公网监听。
        // 输入：MANAGEMENT_SERVER_BIND 环境变量。
        // 输出：Axum 监听 SocketAddr。
        // 边界：生产部署前必须补鉴权、TLS 和完整配置文件。
        Ok(Self {
            bind_addr: bind_text.parse()?,
            history_path,
        })
    }
}

fn history_path_from_value(value: Option<String>) -> Option<PathBuf> {
    value.and_then(|text| {
        let trimmed = text.trim();

        if trimmed.is_empty() {
            None
        } else {
            Some(PathBuf::from(trimmed))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_bind_addr_is_localhost() {
        let config = ServerConfig {
            bind_addr: "127.0.0.1:18080".parse().unwrap(),
            history_path: None,
        };

        assert_eq!(config.bind_addr.port(), 18080);
        assert_eq!(config.history_path, None);
    }

    #[test]
    fn empty_history_path_is_disabled() {
        assert_eq!(history_path_from_value(None), None);
        assert_eq!(history_path_from_value(Some("   ".to_string())), None);
    }

    #[test]
    fn non_empty_history_path_is_enabled() {
        assert_eq!(
            history_path_from_value(Some("data/status-history.jsonl".to_string())),
            Some(PathBuf::from("data/status-history.jsonl"))
        );
    }
}
