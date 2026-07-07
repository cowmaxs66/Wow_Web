use crate::agent::run_once;
use crate::config::AgentConfig;
use crate::local_log::LocalLog;
use crate::notifier;
use crate::server_reporter::StatusReporter;
use crate::status::AgentStatusSnapshot;
use shared_types::{ClientCommandReceiptRequest, WsEnvelope};
use std::collections::HashSet;
use std::error::Error;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::thread;
use std::time::Duration;

pub fn run_monitor(config: AgentConfig) -> Result<(), Box<dyn Error>> {
    run_monitor_until_shutdown(config, None)
}

pub fn run_monitor_until_shutdown(
    config: AgentConfig,
    shutdown: Option<Receiver<()>>,
) -> Result<(), Box<dyn Error>> {
    let log = LocalLog::default();
    let interval = monitor_interval();
    let mut seen_messages = HashSet::new();
    let mut seen_commands = HashSet::new();
    log.append_event("Client monitor 已启动")?;
    let _ = notifier::notify("WoW Client", "客户端监控已启动");

    loop {
        if should_shutdown(shutdown.as_ref()) {
            log.append_event("Client monitor 已收到停止信号")?;
            break;
        }

        match run_once(&config) {
            Ok(result) => {
                log.append_status(&result.envelope)?;
                log.append_event(&format!(
                    "状态已刷新：client_id={} message_id={}",
                    result.envelope.client_id, result.envelope.message_id
                ))?;

                if config.server.enabled {
                    if let Err(error) = poll_messages(&config, &log, &mut seen_messages) {
                        log.append_event(&format!("轮询 Server 消息失败：{error}"))?;
                    }
                    if let Err(error) = poll_commands(&config, &log, &mut seen_commands) {
                        log.append_event(&format!("轮询 Server 命令失败：{error}"))?;
                    }
                }
            }
            Err(error) => {
                let message = format!("Client monitor 执行失败：{error}");
                log.append_event(&message)?;
                let _ = notifier::notify("WoW Client 错误", &message);
            }
        }

        if sleep_with_shutdown(interval, shutdown.as_ref()) {
            log.append_event("Client monitor 已收到停止信号")?;
            break;
        }
    }

    if let Err(error) = report_offline(&config, &log) {
        log.append_event(&format!("离线状态上报失败：{error}"))?;
    }

    Ok(())
}

pub fn report_offline(config: &AgentConfig, log: &LocalLog) -> Result<(), Box<dyn Error>> {
    if !config.server.enabled {
        log.append_event("Server 上报未启用，跳过离线状态上报")?;
        return Ok(());
    }

    let status = AgentStatusSnapshot::offline(config).into_client_status();
    let envelope = WsEnvelope::status(config.client.id.clone(), status);
    let ack = StatusReporter::new(config.server.clone()).report_status(&envelope)?;
    log.append_status(&envelope)?;
    log.append_event(&format!(
        "离线状态已上报：client_id={} message_id={} accepted={}",
        ack.client_id, ack.message_id, ack.accepted
    ))?;
    Ok(())
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

fn poll_commands(
    config: &AgentConfig,
    log: &LocalLog,
    seen_commands: &mut HashSet<String>,
) -> Result<(), Box<dyn Error>> {
    let reporter = StatusReporter::new(config.server.clone());
    let commands = reporter.fetch_commands(&config.client.id)?;

    for command in commands.items {
        if !seen_commands.insert(command.id.clone()) {
            continue;
        }

        let result = crate::remote_command::execute_remote_command(&command.command_type, config);
        let (success, summary) = match result {
            Ok(summary) => {
                log.append_event(&format!(
                    "执行 Server 命令成功：id={} type={} result={}",
                    command.id, command.command_type, summary
                ))?;
                (true, summary)
            }
            Err(error) => {
                let summary = error.to_string();
                let message = format!(
                    "执行 Server 命令失败：id={} type={} error={}",
                    command.id, command.command_type, summary
                );
                log.append_event(&message)?;
                let _ = notifier::notify("WoW Client 命令失败", &message);
                (false, summary)
            }
        };

        // P24 回执是“执行后上报”的审计补充，不能反过来影响本机命令执行结果。
        // 输入：本轮已执行的命令 ID、命令类型、成功标记和摘要。
        // 输出：Server 内存回执队列，供 Web Admin 展示最近执行结果。
        // 边界：Server 不可达时只写本机日志，下一轮继续正常监控。
        let receipt = ClientCommandReceiptRequest {
            command_id: command.id.clone(),
            command_type: command.command_type.clone(),
            success,
            summary: receipt_summary(&summary),
        };
        match reporter.report_command_receipt(&config.client.id, &receipt) {
            Ok(saved) => log.append_event(&format!(
                "Server 命令回执已上报：id={} command_id={} success={}",
                saved.id, saved.command_id, saved.success
            ))?,
            Err(error) => log.append_event(&format!(
                "上报 Server 命令回执失败：command_id={} error={}",
                command.id, error
            ))?,
        }
    }

    Ok(())
}

fn receipt_summary(summary: &str) -> String {
    summary.chars().take(2000).collect()
}

fn should_shutdown(shutdown: Option<&Receiver<()>>) -> bool {
    matches!(
        shutdown.map(Receiver::try_recv),
        Some(Ok(_)) | Some(Err(TryRecvError::Disconnected))
    )
}

fn sleep_with_shutdown(duration: Duration, shutdown: Option<&Receiver<()>>) -> bool {
    let Some(shutdown) = shutdown else {
        thread::sleep(duration);
        return false;
    };

    matches!(
        shutdown.recv_timeout(duration),
        Ok(_) | Err(std::sync::mpsc::RecvTimeoutError::Disconnected)
    )
}
