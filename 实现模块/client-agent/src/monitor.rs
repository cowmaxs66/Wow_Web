use crate::agent::run_once_local;
use crate::config::{AgentConfig, default_config_path};
use crate::local_log::LocalLog;
use crate::notifier;
use crate::server_reporter::StatusReporter;
use crate::status::AgentStatusSnapshot;
use shared_types::{
    ClientCommand, ClientCommandReceiptRequest, ClientMessage, ClientStatus, WsEnvelope,
};
use std::collections::HashSet;
use std::error::Error;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub(crate) type SharedSeenCommands = Arc<Mutex<HashSet<String>>>;

pub fn run_monitor(config: AgentConfig) -> Result<(), Box<dyn Error>> {
    run_monitor_until_shutdown(config, None)
}

pub fn run_monitor_until_shutdown(
    config: AgentConfig,
    shutdown: Option<Receiver<()>>,
) -> Result<(), Box<dyn Error>> {
    let log = LocalLog::default();
    let interval = monitor_interval();
    let max_jitter_ms = monitor_max_jitter_ms();
    let mut seen_messages = HashSet::new();
    let seen_commands = Arc::new(Mutex::new(HashSet::new()));
    let mut active_config = config;
    log.append_event("Client monitor 已启动")?;
    let _ = notifier::notify("WoW Client", "客户端监控已启动");
    let mut realtime_worker = crate::client_realtime::RealtimeWorker::start(
        active_config.clone(),
        Arc::clone(&seen_commands),
    );

    loop {
        if should_shutdown(shutdown.as_ref()) {
            log.append_event("Client monitor 已收到停止信号")?;
            break;
        }

        active_config = reload_config_or_keep(active_config, &log);

        match run_once_local(&active_config) {
            Ok(result) => {
                log.append_status(&result.envelope)?;
                log.append_event(&format!(
                    "状态已刷新：client_id={} message_id={}",
                    result.envelope.client_id, result.envelope.message_id
                ))?;

                run_server_roundtrip(
                    &active_config,
                    &log,
                    &mut seen_messages,
                    &seen_commands,
                    &result.envelope,
                )?;
            }
            Err(error) => {
                let message = format!("Client monitor 执行失败：{error}");
                log.append_event(&message)?;
                let _ = notifier::notify("WoW Client 错误", &message);

                let status =
                    AgentStatusSnapshot::online_without_script(&active_config).into_client_status();
                let envelope = WsEnvelope::status(active_config.client.id.clone(), status);
                log.append_status(&envelope)?;
                log.append_event(
                    "Lua 本轮执行失败，但仍继续同步 Server 消息和命令，允许远程停止或替换脚本",
                )?;
                run_server_roundtrip(
                    &active_config,
                    &log,
                    &mut seen_messages,
                    &seen_commands,
                    &envelope,
                )?;
            }
        }

        if sleep_with_shutdown(
            jittered_interval(interval, max_jitter_ms),
            shutdown.as_ref(),
        ) {
            log.append_event("Client monitor 已收到停止信号")?;
            break;
        }
    }

    if let Some(worker) = realtime_worker.take() {
        worker.stop();
    }

    if let Err(error) = report_offline(&active_config, &log) {
        log.append_event(&format!("离线状态上报失败：{error}"))?;
    }

    Ok(())
}

fn reload_config_or_keep(current: AgentConfig, log: &LocalLog) -> AgentConfig {
    let config_path = default_config_path();

    match AgentConfig::load_from_path(&config_path) {
        Ok(config) => config,
        Err(error) => {
            // monitor 是正式常驻链路，配置被用户或 Server 写错时不能直接退出。
            // 输入：默认配置文件路径。
            // 输出：新配置或上一轮已验证配置。
            // 边界：错误会写本机日志，用户修正 TOML 后下一轮会自动恢复。
            let _ = log.append_event(&format!("重新读取配置失败，继续使用上一轮配置：{error}"));
            current
        }
    }
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
    handle_message_items(messages.items, log, seen_messages)
}

fn handle_message_items(
    messages: Vec<ClientMessage>,
    log: &LocalLog,
    seen_messages: &mut HashSet<String>,
) -> Result<(), Box<dyn Error>> {
    for message in messages {
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

fn monitor_max_jitter_ms() -> u64 {
    std::env::var("CLIENT_AGENT_MONITOR_JITTER_MS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value <= 10_000)
        .unwrap_or(1500)
}

fn jittered_interval(base: Duration, max_jitter_ms: u64) -> Duration {
    if max_jitter_ms == 0 {
        return base;
    }

    let seed = current_jitter_seed();
    base + Duration::from_millis(seed % (max_jitter_ms + 1))
}

fn current_jitter_seed() -> u64 {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.subsec_nanos() as u64)
        .unwrap_or_default();

    // jitter 用于多机器同时启动时错开上报峰值，不需要密码学随机数。
    // 输入：当前纳秒和进程 ID。
    // 输出：一个轻量扰动种子。
    // 边界：同一机器极端情况下仍可能碰撞，但足够降低批量启动时的瞬时并发。
    nanos ^ u64::from(std::process::id())
}

fn sync_server_roundtrip(
    config: &AgentConfig,
    log: &LocalLog,
    seen_messages: &mut HashSet<String>,
    seen_commands: &SharedSeenCommands,
    envelope: &WsEnvelope<ClientStatus>,
) -> Result<(), Box<dyn Error>> {
    let reporter = StatusReporter::new(config.server.clone());
    let response = reporter.sync_client(envelope)?;
    log.append_event(&format!(
        "合并同步完成：client_id={} message_id={} messages={} commands={}",
        response.ack.client_id,
        response.ack.message_id,
        response.messages.total,
        response.commands.total
    ))?;
    handle_message_items(response.messages.items, log, seen_messages)?;
    handle_command_items(
        response.commands.items,
        config,
        log,
        seen_commands,
        &reporter,
    )?;
    Ok(())
}

fn legacy_server_roundtrip(
    config: &AgentConfig,
    log: &LocalLog,
    seen_messages: &mut HashSet<String>,
    seen_commands: &SharedSeenCommands,
    envelope: &WsEnvelope<ClientStatus>,
) -> Result<(), Box<dyn Error>> {
    let reporter = StatusReporter::new(config.server.clone());
    let ack = reporter.report_status(envelope)?;
    log.append_event(&format!(
        "旧轮询状态上报成功：client_id={} message_id={}",
        ack.client_id, ack.message_id
    ))?;
    poll_messages(config, log, seen_messages)?;
    poll_commands(config, log, seen_commands)?;
    Ok(())
}

fn poll_commands(
    config: &AgentConfig,
    log: &LocalLog,
    seen_commands: &SharedSeenCommands,
) -> Result<(), Box<dyn Error>> {
    let reporter = StatusReporter::new(config.server.clone());
    let commands = reporter.fetch_commands(&config.client.id)?;
    handle_command_items(commands.items, config, log, seen_commands, &reporter)
}

fn run_server_roundtrip(
    config: &AgentConfig,
    log: &LocalLog,
    seen_messages: &mut HashSet<String>,
    seen_commands: &SharedSeenCommands,
    envelope: &WsEnvelope<ClientStatus>,
) -> Result<(), Box<dyn Error>> {
    if !config.server.enabled {
        return Ok(());
    }

    let sync_result = sync_server_roundtrip(config, log, seen_messages, seen_commands, envelope);
    if let Err(error) = sync_result {
        log.append_event(&format!("合并同步失败，尝试旧轮询链路：{error}"))?;
        if let Err(error) =
            legacy_server_roundtrip(config, log, seen_messages, seen_commands, envelope)
        {
            log.append_event(&format!("旧轮询链路也失败：{error}"))?;
        }
    }

    Ok(())
}

pub(crate) fn handle_command_items(
    commands: Vec<ClientCommand>,
    config: &AgentConfig,
    log: &LocalLog,
    seen_commands: &SharedSeenCommands,
    reporter: &StatusReporter,
) -> Result<(), Box<dyn Error>> {
    for command in commands {
        if !mark_command_seen(seen_commands, &command.id) {
            continue;
        }

        // P24 回执是“执行后上报”的审计补充，不能反过来影响本机命令执行结果。
        // 输入：本轮已执行的命令 ID、命令类型、成功标记和摘要。
        // 输出：Server 内存回执队列，供 Web Admin 展示最近执行结果。
        // 边界：Server 不可达时只写本机日志，下一轮继续正常监控。
        let receipt = execute_command_to_receipt(&command, config, log)?;
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

pub(crate) fn execute_command_to_receipt(
    command: &ClientCommand,
    config: &AgentConfig,
    log: &LocalLog,
) -> Result<ClientCommandReceiptRequest, Box<dyn Error>> {
    let result = crate::remote_command::execute_remote_command(
        &command.command_type,
        &command.payload,
        config,
    );
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

    Ok(ClientCommandReceiptRequest {
        command_id: command.id.clone(),
        command_type: command.command_type.clone(),
        success,
        summary: receipt_summary(&summary),
    })
}

pub(crate) fn mark_command_seen(seen_commands: &SharedSeenCommands, command_id: &str) -> bool {
    match seen_commands.lock() {
        Ok(mut seen) => seen.insert(command_id.to_string()),
        Err(poisoned) => poisoned.into_inner().insert(command_id.to_string()),
    }
}

pub(crate) fn receipt_summary(summary: &str) -> String {
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
