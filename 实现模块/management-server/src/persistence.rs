use shared_types::{ClientStatus, WsEnvelope};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct HistoryPersistence {
    path: Arc<PathBuf>,
    write_lock: Arc<Mutex<()>>,
}

impl HistoryPersistence {
    pub fn open(path: PathBuf) -> Result<Self, PersistenceError> {
        ensure_history_file(&path)?;

        Ok(Self {
            path: Arc::new(path),
            write_lock: Arc::new(Mutex::new(())),
        })
    }

    pub fn load(path: &Path) -> Result<Vec<WsEnvelope<ClientStatus>>, PersistenceError> {
        ensure_history_file(path)?;
        let file = OpenOptions::new().read(true).open(path)?;
        let reader = BufReader::new(file);
        let mut envelopes = Vec::new();

        for (index, line_result) in reader.lines().enumerate() {
            let line = line_result?;
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            // JSONL 每行是一条完整状态信封。这里严格解析并带上行号，
            // 这样持久化文件损坏时不会被静默吞掉，便于生产排查。
            let envelope =
                serde_json::from_str(trimmed).map_err(|source| PersistenceError::InvalidLine {
                    line: index + 1,
                    source,
                })?;
            envelopes.push(envelope);
        }

        Ok(envelopes)
    }

    pub fn append(&self, envelope: &WsEnvelope<ClientStatus>) -> Result<(), PersistenceError> {
        let _guard = self
            .write_lock
            .lock()
            .map_err(|_| PersistenceError::LockPoisoned)?;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.path.as_ref())?;

        // 写入格式固定为一行一个 JSON 对象，便于追加、备份和人工审查。
        // 输入：已通过 API 校验的 Client 状态信封。
        // 输出：追加到本地 JSONL 文件的一条不可变历史记录。
        // 边界：本步骤不做压缩和轮转，保留策略在 P9 设计文档中定义。
        serde_json::to_writer(&mut file, envelope)?;
        file.write_all(b"\n")?;
        file.flush()?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum PersistenceError {
    Io(std::io::Error),
    Json(serde_json::Error),
    InvalidLine {
        line: usize,
        source: serde_json::Error,
    },
    LockPoisoned,
}

impl Display for PersistenceError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "history file io error: {error}"),
            Self::Json(error) => write!(formatter, "history json encode error: {error}"),
            Self::InvalidLine { line, source } => {
                write!(formatter, "history json line {line} is invalid: {source}")
            }
            Self::LockPoisoned => write!(formatter, "history write lock poisoned"),
        }
    }
}

impl Error for PersistenceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Json(error) => Some(error),
            Self::InvalidLine { source, .. } => Some(source),
            Self::LockPoisoned => None,
        }
    }
}

impl From<std::io::Error> for PersistenceError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for PersistenceError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

fn ensure_history_file(path: &Path) -> Result<(), PersistenceError> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)?;
    }

    OpenOptions::new().create(true).append(true).open(path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn persistence_appends_and_loads_jsonl_history() {
        let dir = unique_temp_dir("jsonl-history");
        let path = dir.join("history").join("status.jsonl");
        let store = HistoryPersistence::open(path.clone()).expect("history store must open");
        let mut envelope = WsEnvelope::status("client-a", ClientStatus::new("client-a"));
        envelope.message_id = "message-a".to_string();

        store.append(&envelope).expect("history line must append");

        let loaded = HistoryPersistence::load(&path).expect("history must reload");
        assert_eq!(loaded, vec![envelope]);

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn persistence_reports_invalid_json_line() {
        let dir = unique_temp_dir("bad-jsonl");
        let path = dir.join("status.jsonl");
        fs::create_dir_all(&dir).expect("fixture dir must exist");
        fs::write(&path, "{bad-json}\n").expect("bad fixture must write");

        let error = HistoryPersistence::load(&path).expect_err("bad json must fail");

        match error {
            PersistenceError::InvalidLine { line, .. } => assert_eq!(line, 1),
            other => panic!("unexpected error: {other}"),
        }

        let _ = fs::remove_dir_all(dir);
    }

    fn unique_temp_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock must be valid")
            .as_nanos();

        std::env::temp_dir().join(format!("wow-{name}-{}-{nanos}", std::process::id()))
    }
}
