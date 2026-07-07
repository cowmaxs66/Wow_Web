use shared_types::{ClientStatus, WsEnvelope};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct LocalLog {
    dir: PathBuf,
}

impl LocalLog {
    pub fn default() -> Self {
        let dir = std::env::var("CLIENT_AGENT_LOG_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("logs"));

        Self { dir }
    }

    pub fn event_path(&self) -> PathBuf {
        self.dir.join("client-agent.log")
    }

    pub fn status_path(&self) -> PathBuf {
        self.dir.join("status-history.jsonl")
    }

    pub fn append_event(&self, message: &str) -> io::Result<()> {
        self.ensure_dir()?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.event_path())?;
        writeln!(file, "{} {message}", current_timestamp_ms())
    }

    pub fn append_status(&self, envelope: &WsEnvelope<ClientStatus>) -> io::Result<()> {
        self.ensure_dir()?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.status_path())?;
        let line = serde_json::to_string(envelope).map_err(io::Error::other)?;
        writeln!(file, "{line}")
    }

    pub fn open_event_log(&self) -> io::Result<()> {
        self.ensure_dir()?;
        if !self.event_path().exists() {
            self.append_event("日志文件已创建")?;
        }
        open_path(&self.event_path())
    }

    fn ensure_dir(&self) -> io::Result<()> {
        fs::create_dir_all(&self.dir)
    }
}

pub fn open_path(path: &Path) -> io::Result<()> {
    let absolute = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

    // P11 只提供本机设置/日志入口，不内嵌完整 GUI。
    // 输入：配置文件或日志文件路径。
    // 输出：交给 Windows 默认程序打开。
    // 边界：生产版托盘菜单和设置窗口后续阶段再做。
    Command::new("cmd")
        .args(["/C", "start", "", &absolute.display().to_string()])
        .spawn()
        .map(|_| ())
}

fn current_timestamp_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be later than UNIX_EPOCH")
        .as_millis()
}
