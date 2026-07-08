#![cfg_attr(windows, windows_subsystem = "windows")]

use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use tao::dpi::LogicalSize;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::WindowBuilder;
use wry::WebViewBuilder;

const DEFAULT_BIND: &str = "127.0.0.1:18080";

fn main() {
    if let Err(error) = run() {
        let _ = write_error_log(error.as_ref());
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let console_url = console_url_from_env_and_args(std::env::args().skip(1))?;
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("WoW Control")
        .with_inner_size(LogicalSize::new(1280.0, 860.0))
        .with_min_inner_size(LogicalSize::new(1024.0, 680.0))
        .build(&event_loop)?;

    // 桌面壳只负责加载本机 Management Server 的 Web Admin。
    // 输入：命令行或环境变量给出的 HTTP(S) URL。
    // 输出：独立桌面窗口，避免再调用 Edge 浏览器进程。
    // 边界：Windows WebView2 Runtime 仍是系统渲染内核；如果系统缺失 Runtime，启动错误会写入 logs。
    let _webview = WebViewBuilder::new()
        .with_url(&console_url)
        .with_new_window_req_handler(|_, _| wry::NewWindowResponse::Deny)
        .build(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}

fn console_url_from_env_and_args<I>(args: I) -> io::Result<String>
where
    I: IntoIterator<Item = String>,
{
    let mut explicit_url = None;
    let mut iterator = args.into_iter();
    while let Some(arg) = iterator.next() {
        if arg == "--url" {
            explicit_url = iterator.next();
            break;
        }

        if let Some(value) = arg.strip_prefix("--url=") {
            explicit_url = Some(value.to_string());
            break;
        }
    }

    let raw = explicit_url
        .or_else(|| std::env::var("WOW_DESKTOP_CONSOLE_URL").ok())
        .unwrap_or_else(default_console_url);
    normalize_console_url(&raw)
}

fn default_console_url() -> String {
    std::env::var("MANAGEMENT_SERVER_BIND")
        .ok()
        .and_then(|bind| server_url_from_bind(&bind))
        .unwrap_or_else(|| server_url_from_bind(DEFAULT_BIND).expect("default bind is valid"))
}

fn normalize_console_url(raw: &str) -> io::Result<String> {
    let value = raw.trim().trim_end_matches('/').to_string();
    if value.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "桌面控制台 URL 不能为空",
        ));
    }

    if !(value.starts_with("http://") || value.starts_with("https://")) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "桌面控制台 URL 必须以 http:// 或 https:// 开头",
        ));
    }

    Ok(value)
}

fn server_url_from_bind(bind: &str) -> Option<String> {
    let addr: SocketAddr = bind.parse().ok()?;
    let host = match addr.ip() {
        IpAddr::V4(ip) if ip.is_unspecified() => "127.0.0.1".to_string(),
        IpAddr::V4(ip) => ip.to_string(),
        IpAddr::V6(ip) if ip.is_unspecified() => "127.0.0.1".to_string(),
        IpAddr::V6(ip) => format!("[{ip}]"),
    };
    Some(format!("http://{host}:{}", addr.port()))
}

fn write_error_log(error: &dyn Error) -> io::Result<()> {
    let log_dir = PathBuf::from("logs");
    fs::create_dir_all(&log_dir)?;
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_dir.join("desktop-console-error.log"))?;
    writeln!(file, "{error}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_console_url_trims_trailing_slash() {
        assert_eq!(
            normalize_console_url(" http://127.0.0.1:18080/ ").unwrap(),
            "http://127.0.0.1:18080"
        );
    }

    #[test]
    fn normalize_console_url_rejects_non_http() {
        assert!(normalize_console_url("file:///tmp/index.html").is_err());
    }

    #[test]
    fn server_url_uses_loopback_for_unspecified_bind() {
        assert_eq!(
            server_url_from_bind("0.0.0.0:18080"),
            Some("http://127.0.0.1:18080".to_string())
        );
    }

    #[test]
    fn args_url_wins_over_default() {
        let args = vec!["--url=http://127.0.0.1:18100".to_string()];
        assert_eq!(
            console_url_from_env_and_args(args).unwrap(),
            "http://127.0.0.1:18100"
        );
    }
}
