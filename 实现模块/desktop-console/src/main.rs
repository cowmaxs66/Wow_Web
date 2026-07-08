#![cfg_attr(windows, windows_subsystem = "windows")]

use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use tao::dpi::LogicalSize;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::WindowBuilder;
use wry::WebViewBuilder;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;
const DEFAULT_BIND: &str = "127.0.0.1:18080";
const SERVER_START_TIMEOUT: Duration = Duration::from_secs(8);
const SERVER_PROBE_INTERVAL: Duration = Duration::from_millis(250);

fn main() {
    if let Err(error) = run() {
        let _ = write_error_log(error.as_ref());
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let console_url = console_url_from_env_and_args(std::env::args().skip(1))?;
    let page = prepare_console_page(&console_url);
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("WoW Control")
        .with_inner_size(LogicalSize::new(1280.0, 860.0))
        .with_min_inner_size(LogicalSize::new(1024.0, 680.0))
        .build(&event_loop)?;

    // 桌面壳负责加载本机 Management Server 的 Web Admin。
    // 输入：命令行或环境变量给出的 HTTP(S) URL。
    // 输出：独立桌面窗口，避免再调用 Edge 浏览器进程。
    // 边界：Windows WebView2 Runtime 仍是系统渲染内核；如果 Server 启动失败，会显示本地中文错误页。
    let builder =
        WebViewBuilder::new().with_new_window_req_handler(|_, _| wry::NewWindowResponse::Deny);
    let builder = match page {
        ConsolePage::Url(url) => builder.with_url(url),
        ConsolePage::Html(html) => builder.with_html(html),
    };
    let _webview = builder.build(&window)?;

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

enum ConsolePage {
    Url(String),
    Html(String),
}

fn prepare_console_page(console_url: &str) -> ConsolePage {
    if server_reachable(console_url) {
        return ConsolePage::Url(console_url.to_string());
    }

    if should_autostart_server(console_url) {
        match start_packaged_server(console_url) {
            Ok(()) => {
                if wait_for_server(console_url, SERVER_START_TIMEOUT) {
                    return ConsolePage::Url(console_url.to_string());
                }

                return ConsolePage::Html(local_error_page(
                    console_url,
                    "Server 已尝试启动，但在 8 秒内没有响应。",
                    "请检查 logs/management-server-error.log，或从包根目录双击 management-server.exe。",
                ));
            }
            Err(error) => {
                let _ = write_error_log(&error);
                return ConsolePage::Html(local_error_page(
                    console_url,
                    "无法自动启动 Management Server。",
                    &error.to_string(),
                ));
            }
        }
    }

    ConsolePage::Html(local_error_page(
        console_url,
        "无法连接 Management Server。",
        "当前 URL 不是本机地址，桌面壳不会自动启动远程 Server。",
    ))
}

fn should_autostart_server(console_url: &str) -> bool {
    endpoint_from_url(console_url)
        .map(|endpoint| is_loopback_host(&endpoint.host))
        .unwrap_or(false)
}

fn server_reachable(console_url: &str) -> bool {
    let Some(endpoint) = endpoint_from_url(console_url) else {
        return false;
    };

    let address = format!("{}:{}", endpoint.host, endpoint.port);
    let Ok(mut addrs) = address.to_socket_addrs() else {
        return false;
    };

    addrs.any(|addr| TcpStream::connect_timeout(&addr, Duration::from_millis(300)).is_ok())
}

fn wait_for_server(console_url: &str, timeout: Duration) -> bool {
    let started_at = Instant::now();
    while started_at.elapsed() < timeout {
        if server_reachable(console_url) {
            return true;
        }
        std::thread::sleep(SERVER_PROBE_INTERVAL);
    }
    false
}

fn start_packaged_server(console_url: &str) -> io::Result<()> {
    let exe_dir = executable_dir()?;
    let server_core = packaged_server_core_path(&exe_dir);
    if !server_core.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("未找到 Server 核心程序：{}", server_core.display()),
        ));
    }

    let log_dir = exe_dir.join("logs");
    fs::create_dir_all(&log_dir)?;
    let stdout = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_dir.join("management-server.log"))?;
    let stderr = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_dir.join("management-server-error.log"))?;

    let mut command = Command::new(server_core);
    command
        .arg("--no-open-browser")
        .current_dir(&exe_dir)
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr));

    if let Some(bind) = management_bind_from_url(console_url) {
        command.env("MANAGEMENT_SERVER_BIND", bind);
    }

    spawn_hidden(command)
}

fn executable_dir() -> io::Result<PathBuf> {
    let exe = std::env::current_exe()?;
    exe.parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| io::Error::other("无法取得桌面控制台所在目录"))
}

fn packaged_server_core_path(exe_dir: &Path) -> PathBuf {
    exe_dir.join("bin").join(if cfg!(windows) {
        "management-server-core.exe"
    } else {
        "management-server-core"
    })
}

fn spawn_hidden(mut command: Command) -> io::Result<()> {
    #[cfg(windows)]
    {
        command.creation_flags(CREATE_NO_WINDOW);
    }

    command.spawn().map(|_| ())
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

struct Endpoint {
    scheme: String,
    host: String,
    port: u16,
}

fn endpoint_from_url(url: &str) -> Option<Endpoint> {
    let value = url.trim();
    let (scheme, rest) = value.split_once("://")?;
    let scheme = scheme.to_ascii_lowercase();
    if scheme != "http" && scheme != "https" {
        return None;
    }

    let authority = rest.split('/').next()?.trim();
    if authority.is_empty() {
        return None;
    }

    let (host, port) = parse_authority(authority, &scheme)?;
    Some(Endpoint { scheme, host, port })
}

fn parse_authority(authority: &str, scheme: &str) -> Option<(String, u16)> {
    if let Some(rest) = authority.strip_prefix('[') {
        let (host, after_host) = rest.split_once(']')?;
        let port = if let Some(port_text) = after_host.strip_prefix(':') {
            port_text.parse().ok()?
        } else {
            default_port_for_scheme(scheme)?
        };
        return Some((host.to_string(), port));
    }

    let (host, port) = match authority.rsplit_once(':') {
        Some((host, port_text)) if !host.contains(':') => (host, port_text.parse().ok()?),
        Some(_) => (authority, default_port_for_scheme(scheme)?),
        None => (authority, default_port_for_scheme(scheme)?),
    };
    Some((host.to_string(), port))
}

fn default_port_for_scheme(scheme: &str) -> Option<u16> {
    match scheme {
        "http" => Some(80),
        "https" => Some(443),
        _ => None,
    }
}

fn management_bind_from_url(url: &str) -> Option<String> {
    let endpoint = endpoint_from_url(url)?;
    if endpoint.scheme != "http" || !is_loopback_host(&endpoint.host) {
        return None;
    }

    Some(format!("{}:{}", bind_host(&endpoint.host), endpoint.port))
}

fn bind_host(host: &str) -> &str {
    if host.eq_ignore_ascii_case("localhost") {
        "127.0.0.1"
    } else {
        host
    }
}

fn is_loopback_host(host: &str) -> bool {
    host.eq_ignore_ascii_case("localhost")
        || host == "::1"
        || host.parse::<IpAddr>().is_ok_and(|ip| ip.is_loopback())
}

fn local_error_page(console_url: &str, title: &str, detail: &str) -> String {
    format!(
        r#"<!doctype html>
<html lang="zh-Hant">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>WoW Control</title>
  <style>
    body {{ margin: 0; font-family: "Microsoft JhengHei", "Microsoft YaHei", Segoe UI, sans-serif; background: #f6f7fb; color: #172033; }}
    main {{ max-width: 760px; margin: 14vh auto 0; padding: 0 32px; }}
    .panel {{ background: #fff; border: 1px solid #d9e0ec; border-radius: 8px; padding: 28px; box-shadow: 0 18px 50px rgba(28, 42, 72, .08); }}
    h1 {{ margin: 0 0 12px; font-size: 28px; }}
    p {{ margin: 10px 0; line-height: 1.7; color: #4a5872; }}
    code {{ background: #eef2f8; padding: 2px 6px; border-radius: 4px; color: #23314d; }}
    button {{ margin-top: 18px; padding: 10px 18px; border: 0; border-radius: 6px; background: #2563eb; color: white; font-size: 15px; cursor: pointer; }}
  </style>
</head>
<body>
  <main>
    <section class="panel">
      <h1>{}</h1>
      <p>{}</p>
      <p>目标地址：<code>{}</code></p>
      <p>建议：确认包根目录存在 <code>bin/management-server-core.exe</code>，或直接双击 <code>management-server.exe</code> 启动托盘。</p>
      <button onclick="location.reload()">重新连接</button>
    </section>
  </main>
</body>
</html>"#,
        escape_html(title),
        escape_html(detail),
        escape_html(console_url)
    )
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
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

    #[test]
    fn endpoint_from_url_reads_loopback_host_and_port() {
        let endpoint = endpoint_from_url("http://127.0.0.1:18080/dashboard").unwrap();

        assert_eq!(endpoint.scheme, "http");
        assert_eq!(endpoint.host, "127.0.0.1");
        assert_eq!(endpoint.port, 18080);
    }

    #[test]
    fn endpoint_from_url_reads_ipv6_host() {
        let endpoint = endpoint_from_url("http://[::1]:18080").unwrap();

        assert_eq!(endpoint.host, "::1");
        assert_eq!(endpoint.port, 18080);
    }

    #[test]
    fn management_bind_from_url_normalizes_localhost() {
        assert_eq!(
            management_bind_from_url("http://localhost:18080"),
            Some("127.0.0.1:18080".to_string())
        );
    }

    #[test]
    fn management_bind_from_url_rejects_remote_url() {
        assert_eq!(management_bind_from_url("http://192.168.1.20:18080"), None);
    }

    #[test]
    fn local_error_page_escapes_dynamic_text() {
        let html = local_error_page("http://127.0.0.1:18080", "<bad>", "\"quoted\"");

        assert!(html.contains("&lt;bad&gt;"));
        assert!(html.contains("&quot;quoted&quot;"));
    }
}
