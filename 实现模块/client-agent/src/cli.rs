#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentCommand {
    Tray,
    RunOnce { notify: bool },
    Monitor,
    Setup,
    OpenLog,
    LogWindow,
    StartupStatus,
    EnableStartup,
    DisableStartup,
    ServiceRun,
    ServiceInstall,
    ServiceUninstall,
    ServiceStart,
    ServiceStop,
    ServiceStatus,
    SettingsWindow,
    UpdateCheck,
    UpdateDownload,
    UpdateApply,
    ReportOffline,
    Help,
}

pub fn parse_args(args: impl IntoIterator<Item = String>) -> Result<AgentCommand, String> {
    let mut command = AgentCommand::Tray;

    for arg in args.into_iter().skip(1) {
        match arg.as_str() {
            "--run-once" => command = AgentCommand::RunOnce { notify: false },
            "--monitor" => command = AgentCommand::Monitor,
            "--setup" => command = AgentCommand::Setup,
            "--open-log" => command = AgentCommand::OpenLog,
            "--log-window" => command = AgentCommand::LogWindow,
            "--startup-status" => command = AgentCommand::StartupStatus,
            "--enable-startup" => command = AgentCommand::EnableStartup,
            "--disable-startup" => command = AgentCommand::DisableStartup,
            "--service-run" => command = AgentCommand::ServiceRun,
            "--service-install" => command = AgentCommand::ServiceInstall,
            "--service-uninstall" => command = AgentCommand::ServiceUninstall,
            "--service-start" => command = AgentCommand::ServiceStart,
            "--service-stop" => command = AgentCommand::ServiceStop,
            "--service-status" => command = AgentCommand::ServiceStatus,
            "--tray" => command = AgentCommand::Tray,
            "--settings-window" => command = AgentCommand::SettingsWindow,
            "--update-check" => command = AgentCommand::UpdateCheck,
            "--update-download" => command = AgentCommand::UpdateDownload,
            "--update-apply" => command = AgentCommand::UpdateApply,
            "--report-offline" => command = AgentCommand::ReportOffline,
            "--notify" => command = AgentCommand::RunOnce { notify: true },
            "--help" | "-h" => command = AgentCommand::Help,
            unknown => return Err(format!("未知参数：{unknown}")),
        }
    }

    Ok(command)
}

pub fn help_text() -> &'static str {
    "client-agent 用法：\n  client-agent.exe                     启动托盘常驻 UI，并拉起 monitor\n  client-agent.exe --tray              启动托盘常驻 UI，并拉起 monitor\n  client-agent.exe --run-once          执行一次并输出状态 JSON\n  client-agent.exe --monitor           常驻监控、上报状态、轮询 Server 消息和命令\n  client-agent.exe --settings-window   打开原生设置窗口\n  client-agent.exe --open-log          打开本机日志文件\n  client-agent.exe --log-window        打开本机日志查看窗口\n  client-agent.exe --startup-status    查看当前用户开机启动状态\n  client-agent.exe --enable-startup    写入当前用户开机启动项\n  client-agent.exe --disable-startup   删除当前用户开机启动项\n  client-agent.exe --service-install   安装 Windows Service\n  client-agent.exe --service-uninstall 卸载 Windows Service\n  client-agent.exe --service-start     启动 Windows Service\n  client-agent.exe --service-stop      停止 Windows Service\n  client-agent.exe --service-status    查看 Windows Service 状态\n  client-agent.exe --update-check      检查 GitHub 最新版本\n  client-agent.exe --update-download   下载 GitHub 最新发布包\n  client-agent.exe --update-apply      下载新版并安排自替换安装"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_monitor_command() {
        let command = parse_args(["client-agent".to_string(), "--monitor".to_string()])
            .expect("monitor command must parse");

        assert_eq!(command, AgentCommand::Monitor);
    }

    #[test]
    fn parse_no_args_starts_tray() {
        let command = parse_args(["client-agent".to_string()]).expect("default command must parse");

        assert_eq!(command, AgentCommand::Tray);
    }

    #[test]
    fn parse_run_once_command() {
        let command = parse_args(["client-agent".to_string(), "--run-once".to_string()])
            .expect("run once command must parse");

        assert_eq!(command, AgentCommand::RunOnce { notify: false });
    }

    #[test]
    fn parse_startup_commands() {
        let status = parse_args(["client-agent".to_string(), "--startup-status".to_string()])
            .expect("startup status command must parse");
        let enable = parse_args(["client-agent".to_string(), "--enable-startup".to_string()])
            .expect("enable startup command must parse");
        let disable = parse_args(["client-agent".to_string(), "--disable-startup".to_string()])
            .expect("disable startup command must parse");

        assert_eq!(status, AgentCommand::StartupStatus);
        assert_eq!(enable, AgentCommand::EnableStartup);
        assert_eq!(disable, AgentCommand::DisableStartup);
    }

    #[test]
    fn parse_formal_runtime_commands() {
        let service = parse_args(["client-agent".to_string(), "--service-status".to_string()])
            .expect("service status command must parse");
        let tray = parse_args(["client-agent".to_string(), "--tray".to_string()])
            .expect("tray command must parse");
        let settings = parse_args(["client-agent".to_string(), "--settings-window".to_string()])
            .expect("settings command must parse");
        let log_window = parse_args(["client-agent".to_string(), "--log-window".to_string()])
            .expect("log window command must parse");
        let update = parse_args(["client-agent".to_string(), "--update-check".to_string()])
            .expect("update command must parse");
        let update_apply = parse_args(["client-agent".to_string(), "--update-apply".to_string()])
            .expect("update apply command must parse");
        let offline = parse_args(["client-agent".to_string(), "--report-offline".to_string()])
            .expect("offline command must parse");

        assert_eq!(service, AgentCommand::ServiceStatus);
        assert_eq!(tray, AgentCommand::Tray);
        assert_eq!(settings, AgentCommand::SettingsWindow);
        assert_eq!(log_window, AgentCommand::LogWindow);
        assert_eq!(update, AgentCommand::UpdateCheck);
        assert_eq!(update_apply, AgentCommand::UpdateApply);
        assert_eq!(offline, AgentCommand::ReportOffline);
    }
}
