mod agent;
mod cli;
mod config;
mod dm_bridge;
mod local_log;
mod logging;
mod lua_dm;
mod lua_host;
mod monitor;
mod notifier;
mod remote_command;
mod script;
mod server_reporter;
mod service_runtime;
mod settings_window;
mod startup;
mod status;
mod tray;
mod updater;

use agent::run_once;
use cli::{AgentCommand, help_text, parse_args};
use config::{AgentConfig, default_config_path};
use local_log::{LocalLog, open_path};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    logging::init();

    let command = parse_args(std::env::args()).map_err(|message| {
        eprintln!("{message}\n\n{}", help_text());
        message
    })?;
    if command == AgentCommand::Help {
        println!("{}", help_text());
        return Ok(());
    }

    let config_path = default_config_path();
    if command == AgentCommand::Setup {
        open_path(&config_path)?;
        return Ok(());
    }

    if command == AgentCommand::OpenLog {
        LocalLog::default().open_event_log()?;
        return Ok(());
    }

    if command == AgentCommand::SettingsWindow {
        settings_window::open_settings_window()?;
        return Ok(());
    }

    if command == AgentCommand::Tray {
        tray::run_tray()?;
        return Ok(());
    }

    if command == AgentCommand::StartupStatus {
        println!("{}", startup::startup_status()?.summary());
        return Ok(());
    }

    if command == AgentCommand::EnableStartup {
        let status = startup::enable_startup()?;
        let _ = LocalLog::default().append_event("已启用当前用户开机启动");
        println!("{}", status.summary());
        return Ok(());
    }

    if command == AgentCommand::DisableStartup {
        let status = startup::disable_startup()?;
        let _ = LocalLog::default().append_event("已禁用当前用户开机启动");
        println!("{}", status.summary());
        return Ok(());
    }

    if command == AgentCommand::ServiceRun {
        return service_runtime::run_service();
    }

    if command == AgentCommand::ServiceInstall {
        println!("{}", service_runtime::install_service()?);
        return Ok(());
    }

    if command == AgentCommand::ServiceUninstall {
        println!("{}", service_runtime::uninstall_service()?);
        return Ok(());
    }

    if command == AgentCommand::ServiceStart {
        println!("{}", service_runtime::start_service()?);
        return Ok(());
    }

    if command == AgentCommand::ServiceStop {
        println!("{}", service_runtime::stop_service()?);
        return Ok(());
    }

    if command == AgentCommand::ServiceStatus {
        println!("{}", service_runtime::service_status()?);
        return Ok(());
    }

    if command == AgentCommand::UpdateCheck {
        println!("{}", updater::check_update()?);
        return Ok(());
    }

    if command == AgentCommand::UpdateDownload {
        println!("{}", updater::download_update()?);
        return Ok(());
    }

    let config = AgentConfig::load_from_path(default_config_path())?;

    if command == AgentCommand::Monitor {
        return monitor::run_monitor(config);
    }

    let result = run_once(&config)?;
    let log = LocalLog::default();
    let _ = log.append_status(&result.envelope);
    if let Some(ack) = &result.ack {
        let _ = log.append_event(&format!(
            "状态上报成功：client_id={} message_id={}",
            ack.client_id, ack.message_id
        ));
    }

    if command == (AgentCommand::RunOnce { notify: true }) {
        let _ = notifier::notify(
            "WoW Client 状态",
            &format!("{} 已完成一次状态刷新", result.envelope.client_id),
        );
    }

    // 当前阶段输出标准 JSON，验证配置读取、Lua 宿主和协议消息可以串成闭环。
    // 输入：本地 TOML 配置和 bootstrap Lua 文件。
    // 输出：包含 client_id 与当前脚本名的状态消息。
    // 边界：真实 WebSocket 上报在 P2/P3 接入，这里只输出可验证 JSON。
    let json =
        serde_json::to_string_pretty(&result.envelope).expect("status envelope must serialize");
    println!("{json}");

    Ok(())
}
