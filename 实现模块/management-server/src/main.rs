mod app;
mod config;
mod error;
mod state;

use config::ServerConfig;
use std::error::Error;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_logging();

    let config = ServerConfig::from_env()?;
    let listener = TcpListener::bind(config.bind_addr).await?;
    tracing::info!(bind = %config.bind_addr, "Management Server 已启动");

    axum::serve(listener, app::build_router(state::ServerState::default())).await?;
    Ok(())
}

fn init_logging() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_target(false)
        .try_init();
}
