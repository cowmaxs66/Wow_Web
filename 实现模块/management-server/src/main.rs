mod app;
mod config;
mod error;
mod persistence;
mod state;

use config::ServerConfig;
use std::error::Error;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_logging();

    let config = ServerConfig::from_env()?;
    let state = match config.history_path.clone() {
        Some(path) => state::ServerState::with_persistence(path)?,
        None => state::ServerState::default(),
    };

    let listener = TcpListener::bind(config.bind_addr).await?;
    tracing::info!(
        bind = %config.bind_addr,
        history_path = ?config.history_path,
        web_dir = ?config.web_dir,
        "Management Server 已启动"
    );

    axum::serve(
        listener,
        app::build_router_with_web_dir(state, config.web_dir),
    )
    .await?;
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
