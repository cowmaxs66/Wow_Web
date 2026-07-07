mod app;
mod config;
mod embedded_web;
mod error;
mod persistence;
mod ps_script;
mod state;
mod tray;

use config::ServerConfig;
use std::error::Error;
use std::process::Command;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_logging();

    let launch = parse_launch_args(std::env::args())
        .map_err(|message| std::io::Error::new(std::io::ErrorKind::InvalidInput, message))?;
    if launch.help {
        println!("{}", help_text());
        return Ok(());
    }
    if launch.tray {
        tray::run_tray()?;
        return Ok(());
    }

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
        embedded_web_assets = embedded_web::asset_count(),
        "Management Server 已启动"
    );
    if launch.open_browser {
        open_browser_to(&format!("http://{}", config.bind_addr));
    }

    axum::serve(
        listener,
        app::build_router_with_web_dir(state, config.web_dir),
    )
    .await?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LaunchOptions {
    open_browser: bool,
    tray: bool,
    help: bool,
}

fn parse_launch_args(args: impl IntoIterator<Item = String>) -> Result<LaunchOptions, String> {
    let mut options = LaunchOptions {
        open_browser: true,
        tray: false,
        help: false,
    };

    for arg in args.into_iter().skip(1) {
        match arg.as_str() {
            "--open-browser" => options.open_browser = true,
            "--no-open-browser" | "--api-only" => options.open_browser = false,
            "--tray" => {
                options.tray = true;
                options.open_browser = false;
            }
            "--help" | "-h" => options.help = true,
            unknown => return Err(format!("未知参数：{unknown}\n\n{}", help_text())),
        }
    }

    Ok(options)
}

fn help_text() -> &'static str {
    "management-server 用法：\n  management-server.exe                    启动 Server 并打开 Web 管理页\n  management-server.exe --tray             启动 Server 托盘常驻 UI\n  management-server.exe --open-browser     启动 Server 并打开 Web 管理页\n  management-server.exe --no-open-browser  只启动 API / Web 服务，不自动打开浏览器\n  management-server.exe --api-only         同 --no-open-browser"
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

fn open_browser_to(url: &str) {
    #[cfg(windows)]
    {
        let _ = Command::new("cmd").args(["/C", "start", "", url]).spawn();
    }

    #[cfg(not(windows))]
    {
        let _ = url;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_args_opens_browser_by_default() {
        let options =
            parse_launch_args(["management-server".to_string()]).expect("args must parse");

        assert_eq!(
            options,
            LaunchOptions {
                open_browser: true,
                tray: false,
                help: false
            }
        );
    }

    #[test]
    fn api_only_disables_browser_open() {
        let options = parse_launch_args([
            "management-server".to_string(),
            "--no-open-browser".to_string(),
        ])
        .expect("args must parse");

        assert!(!options.open_browser);
        assert!(!options.tray);
    }

    #[test]
    fn tray_disables_direct_browser_open() {
        let options = parse_launch_args(["management-server".to_string(), "--tray".to_string()])
            .expect("args must parse");

        assert_eq!(
            options,
            LaunchOptions {
                open_browser: false,
                tray: true,
                help: false
            }
        );
    }
}
