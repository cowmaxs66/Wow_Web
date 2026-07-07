#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentCommand {
    RunOnce { notify: bool },
    Monitor,
    Setup,
    OpenLog,
    Help,
}

pub fn parse_args(args: impl IntoIterator<Item = String>) -> Result<AgentCommand, String> {
    let mut command = AgentCommand::RunOnce { notify: false };

    for arg in args.into_iter().skip(1) {
        match arg.as_str() {
            "--monitor" => command = AgentCommand::Monitor,
            "--setup" => command = AgentCommand::Setup,
            "--open-log" => command = AgentCommand::OpenLog,
            "--notify" => command = AgentCommand::RunOnce { notify: true },
            "--help" | "-h" => command = AgentCommand::Help,
            unknown => return Err(format!("未知参数：{unknown}")),
        }
    }

    Ok(command)
}

pub fn help_text() -> &'static str {
    "client-agent 用法：\n  client-agent.exe              执行一次并输出状态 JSON\n  client-agent.exe --notify     执行一次并弹出通知气泡\n  client-agent.exe --monitor    常驻监控、上报状态、轮询 Server 消息\n  client-agent.exe --setup      打开本机配置文件\n  client-agent.exe --open-log   打开本机日志文件"
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
}
