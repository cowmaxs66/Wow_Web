use crate::config::AgentConfig;
use crate::local_log::LocalLog;
use std::error::Error;

pub fn execute_remote_command(
    command_type: &str,
    config: &AgentConfig,
) -> Result<String, Box<dyn Error>> {
    match command_type {
        "script.run_bootstrap" => run_bootstrap_command(config),
        "startup.status" => Ok(crate::startup::startup_status()?.summary()),
        "startup.enable" => Ok(crate::startup::enable_startup()?.summary()),
        "startup.disable" => Ok(crate::startup::disable_startup()?.summary()),
        "service.status" => Ok(crate::service_runtime::service_status()?),
        "service.install" => Ok(crate::service_runtime::install_service()?),
        "service.start" => Ok(crate::service_runtime::start_service()?),
        "service.stop" => Ok(crate::service_runtime::stop_service()?),
        "update.check" => Ok(crate::updater::check_update()?),
        "update.download" => Ok(crate::updater::download_update()?),
        // Server 下发安装更新时复用本机自替换更新器。
        // 输入：Management Server 命令队列中的 update.apply。
        // 输出：检查 GitHub Release、下载新版包、启动独立替换脚本后的 JSON 摘要。
        // 边界：替换脚本可能停止当前 monitor，执行过程会写入本机 update-apply.log。
        "update.apply" => Ok(crate::updater::apply_update()?),
        "settings.open" => {
            crate::settings_window::open_settings_window()?;
            Ok("已请求打开设置窗口".to_string())
        }
        "log.open" => {
            LocalLog::default().open_event_log()?;
            Ok("已请求打开日志窗口".to_string())
        }
        "tray.open" => {
            crate::tray::run_tray()?;
            Ok("已请求打开托盘".to_string())
        }
        unknown => Err(format!("不支持的远程命令：{unknown}").into()),
    }
}

fn run_bootstrap_command(config: &AgentConfig) -> Result<String, Box<dyn Error>> {
    let result = crate::agent::run_once(config)?;
    let log = LocalLog::default();
    let _ = log.append_status(&result.envelope);

    // 远程脚本命令只重新执行当前本机已配置并通过安全校验的 bootstrap。
    // 输入：Client 本机 client-agent.toml、manifest 与 Lua 文件。
    // 输出：新的状态上报和本机日志摘要。
    // 边界：Server 不直接传入任意 Lua 文本，避免把远程命令变成任意代码执行入口。
    let script = result
        .envelope
        .data
        .current_script
        .as_deref()
        .unwrap_or("无");
    Ok(format!(
        "Lua bootstrap 已执行：client_id={} script={} message_id={}",
        result.envelope.client_id, script, result.envelope.message_id
    ))
}

#[cfg(test)]
mod tests {
    use crate::config::{
        AgentConfig, ClientConfig, DmConfig, LuaConfig, ScriptSecurityConfig, ServerConfig,
    };
    use std::path::PathBuf;

    #[test]
    fn unsupported_remote_command_is_rejected() {
        let error = super::execute_remote_command("shell.exec", &test_config())
            .expect_err("must reject shell");
        assert!(error.to_string().contains("不支持"));
    }

    fn test_config() -> AgentConfig {
        AgentConfig {
            client: ClientConfig {
                id: "remote-test-client".to_string(),
            },
            lua: LuaConfig {
                bootstrap_name: "bootstrap".to_string(),
                bootstrap_path: PathBuf::from("scripts/bootstrap.lua"),
                instruction_limit: 10_000,
            },
            script_security: ScriptSecurityConfig {
                enabled: false,
                manifest_path: PathBuf::from("scripts/bootstrap.manifest.json"),
                trusted_signer_public_key:
                    "1111111111111111111111111111111111111111111111111111111111111111".to_string(),
                allowed_permissions: vec!["host.log".to_string()],
            },
            dm: DmConfig {
                bridge_path: PathBuf::from("missing/DmBridge.dll"),
            },
            server: ServerConfig {
                enabled: false,
                host: "127.0.0.1".to_string(),
                port: 18080,
                status_path: "/api/client/status".to_string(),
                connect_timeout_ms: 3000,
            },
        }
    }
}
