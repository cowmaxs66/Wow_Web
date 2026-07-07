#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentCommand {
    RunOnce { notify: bool },
    Monitor,
    Setup,
    OpenLog,
    StartupStatus,
    EnableStartup,
    DisableStartup,
    Help,
}

pub fn parse_args(args: impl IntoIterator<Item = String>) -> Result<AgentCommand, String> {
    let mut command = AgentCommand::RunOnce { notify: false };

    for arg in args.into_iter().skip(1) {
        match arg.as_str() {
            "--monitor" => command = AgentCommand::Monitor,
            "--setup" => command = AgentCommand::Setup,
            "--open-log" => command = AgentCommand::OpenLog,
            "--startup-status" => command = AgentCommand::StartupStatus,
            "--enable-startup" => command = AgentCommand::EnableStartup,
            "--disable-startup" => command = AgentCommand::DisableStartup,
            "--notify" => command = AgentCommand::RunOnce { notify: true },
            "--help" | "-h" => command = AgentCommand::Help,
            unknown => return Err(format!("未知参数：{unknown}")),
        }
    }

    Ok(command)
}

pub fn help_text() -> &'static str {
    "client-agent 用法：\n  client-agent.exe                   执行一次并输出状态 JSON\n  client-agent.exe --notify          执行一次并弹出通知气泡\n  client-agent.exe --monitor         常驻监控、上报状态、轮询 Server 消息\n  client-agent.exe --setup           打开本机配置文件\n  client-agent.exe --open-log        打开本机日志文件\n  client-agent.exe --startup-status  查看当前用户开机启动状态\n  client-agent.exe --enable-startup  写入当前用户开机启动项\n  client-agent.exe --disable-startup 删除当前用户开机启动项"
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
}
