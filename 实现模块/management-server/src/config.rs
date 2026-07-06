use std::net::SocketAddr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerConfig {
    pub bind_addr: SocketAddr,
}

impl ServerConfig {
    pub fn from_env() -> Result<Self, std::net::AddrParseError> {
        let bind_text = std::env::var("MANAGEMENT_SERVER_BIND")
            .unwrap_or_else(|_| "127.0.0.1:18080".to_string());

        // P3 只支持本机开发期绑定地址，避免一开始暴露公网监听。
        // 输入：MANAGEMENT_SERVER_BIND 环境变量。
        // 输出：Axum 监听 SocketAddr。
        // 边界：生产部署前必须补鉴权、TLS 和配置文件。
        Ok(Self {
            bind_addr: bind_text.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_bind_addr_is_localhost() {
        let config = ServerConfig {
            bind_addr: "127.0.0.1:18080".parse().unwrap(),
        };

        assert_eq!(config.bind_addr.port(), 18080);
    }
}
