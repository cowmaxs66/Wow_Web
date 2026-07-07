use crate::agent::run_once;
use crate::config::AgentConfig;
use crate::local_log::LocalLog;
use crate::notifier;
use crate::server_reporter::StatusReporter;
use std::collections::HashSet;
use std::error::Error;
use std::thread;
use std::time::Duration;

pub fn run_monitor(config: AgentConfig) -> Result<(), Box<dyn Error>> {
    let log = LocalLog::default();
    let interval = monitor_interval();
    let mut seen_messages = HashSet::new();
    log.append_event("Client monitor 已启动")?;
    let _ = notifier::notify("WoW Client", "客户端监控已启动");

    loop {
        match run_once(&config) {
            Ok(result) => {
                log.append_status(&result.envelope)?;
                log.append_event(&format!(
                    "状态已刷新：client_id={} message_id={}",
                    result.envelope.client_id, result.envelope.message_id
                ))?;

                if config.server.enabled {
                    poll_messages(&config, &log, &mut seen_messages)?;
                }
            }
            Err(error) => {
                let message = format!("Client monitor 执行失败：{error}");
                log.append_event(&message)?;
                let _ = notifier::notify("WoW Client 错误", &message);
            }
        }

        thread::sleep(interval);
    }
}

fn poll_messages(
    config: &AgentConfig,
    log: &LocalLog,
    seen_messages: &mut HashSet<String>,
) -> Result<(), Box<dyn Error>> {
    let reporter = StatusReporter::new(config.server.clone());
    let messages = reporter.fetch_messages(&config.client.id)?;

    for message in messages.items {
        if !seen_messages.insert(message.id.clone()) {
            continue;
        }

        // Server 消息进入本地日志，再触发系统托盘气泡。
        // 输入：Management Server 内存消息队列。
        // 输出：logs/client-agent.log 和 Windows 右下角通知。
        // 边界：当前只做轮询，不做 WebSocket 长连接与送达确认。
        log.append_event(&format!(
            "收到 Server 消息：id={} title={} body={}",
            message.id, message.title, message.body
        ))?;
        let _ = notifier::notify(&message.title, &message.body);
    }

    Ok(())
}

fn monitor_interval() -> Duration {
    let seconds = std::env::var("CLIENT_AGENT_MONITOR_INTERVAL_SECONDS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value >= 2)
        .unwrap_or(10);

    Duration::from_secs(seconds)
}
