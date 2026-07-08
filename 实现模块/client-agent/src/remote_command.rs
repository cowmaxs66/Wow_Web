use crate::config::AgentConfig;
use crate::local_log::LocalLog;
use shared_types::{
    REMOTE_COMMAND_CONFIG_APPLY, REMOTE_COMMAND_LOG_OPEN, REMOTE_COMMAND_SCRIPT_DEPLOY_BUNDLE,
    REMOTE_COMMAND_SCRIPT_RUN_BOOTSTRAP, REMOTE_COMMAND_SCRIPT_START, REMOTE_COMMAND_SCRIPT_STATUS,
    REMOTE_COMMAND_SCRIPT_STOP, REMOTE_COMMAND_SERVICE_INSTALL, REMOTE_COMMAND_SERVICE_START,
    REMOTE_COMMAND_SERVICE_STATUS, REMOTE_COMMAND_SERVICE_STOP, REMOTE_COMMAND_SETTINGS_OPEN,
    REMOTE_COMMAND_STARTUP_DISABLE, REMOTE_COMMAND_STARTUP_ENABLE, REMOTE_COMMAND_STARTUP_STATUS,
    REMOTE_COMMAND_TRAY_OPEN, REMOTE_COMMAND_UPDATE_APPLY, REMOTE_COMMAND_UPDATE_CHECK,
    REMOTE_COMMAND_UPDATE_DOWNLOAD, is_supported_remote_command,
};
use thiserror::Error;

pub fn execute_remote_command(
    command_type: &str,
    payload: &serde_json::Value,
    config: &AgentConfig,
) -> Result<String, RemoteCommandError> {
    if !is_supported_remote_command(command_type) {
        return Err(RemoteCommandError::Unsupported(command_type.to_string()));
    }

    match command_type {
        REMOTE_COMMAND_SCRIPT_RUN_BOOTSTRAP => run_bootstrap_command(config),
        REMOTE_COMMAND_SCRIPT_DEPLOY_BUNDLE => crate::script_deploy::deploy_script_bundle(payload)
            .map_err(|error| RemoteCommandError::execute(command_type, error)),
        REMOTE_COMMAND_SCRIPT_START => start_lua_command(),
        REMOTE_COMMAND_SCRIPT_STOP => stop_lua_command(),
        REMOTE_COMMAND_SCRIPT_STATUS => Ok(lua_status_summary(config)),
        REMOTE_COMMAND_STARTUP_STATUS => crate::startup::startup_status()
            .map(|status| status.summary())
            .map_err(|error| RemoteCommandError::execute(command_type, error)),
        REMOTE_COMMAND_STARTUP_ENABLE => crate::startup::enable_startup()
            .map(|status| status.summary())
            .map_err(|error| RemoteCommandError::execute(command_type, error)),
        REMOTE_COMMAND_STARTUP_DISABLE => crate::startup::disable_startup()
            .map(|status| status.summary())
            .map_err(|error| RemoteCommandError::execute(command_type, error)),
        REMOTE_COMMAND_SERVICE_STATUS => crate::service_runtime::service_status()
            .map_err(|error| RemoteCommandError::execute(command_type, error)),
        REMOTE_COMMAND_SERVICE_INSTALL => crate::service_runtime::install_service()
            .map_err(|error| RemoteCommandError::execute(command_type, error)),
        REMOTE_COMMAND_SERVICE_START => crate::service_runtime::start_service()
            .map_err(|error| RemoteCommandError::execute(command_type, error)),
        REMOTE_COMMAND_SERVICE_STOP => crate::service_runtime::stop_service()
            .map_err(|error| RemoteCommandError::execute(command_type, error)),
        REMOTE_COMMAND_UPDATE_CHECK => crate::updater::check_update()
            .map_err(|error| RemoteCommandError::execute(command_type, error)),
        REMOTE_COMMAND_UPDATE_DOWNLOAD => crate::updater::download_update()
            .map_err(|error| RemoteCommandError::execute(command_type, error)),
        // Server 下发安装更新时复用本机自替换更新器。
        // 输入：Management Server 命令队列中的 update.apply。
        // 输出：检查 GitHub Release、下载新版包、启动独立替换脚本后的 JSON 摘要。
        // 边界：替换脚本可能停止当前 monitor，执行过程会写入本机 update-apply.log。
        REMOTE_COMMAND_UPDATE_APPLY => crate::updater::apply_update()
            .map_err(|error| RemoteCommandError::execute(command_type, error)),
        REMOTE_COMMAND_CONFIG_APPLY => crate::config::apply_remote_patch(payload)
            .map_err(|error| RemoteCommandError::execute(command_type, error)),
        REMOTE_COMMAND_SETTINGS_OPEN => {
            crate::settings_window::open_settings_window()
                .map_err(|error| RemoteCommandError::execute(command_type, error))?;
            Ok("已请求打开设置窗口".to_string())
        }
        REMOTE_COMMAND_LOG_OPEN => {
            LocalLog::default()
                .open_event_log()
                .map_err(|error| RemoteCommandError::execute(command_type, error))?;
            Ok("已请求打开日志窗口".to_string())
        }
        REMOTE_COMMAND_TRAY_OPEN => {
            crate::tray::run_tray()
                .map_err(|error| RemoteCommandError::execute(command_type, error))?;
            Ok("已请求打开托盘".to_string())
        }
        _ => Err(RemoteCommandError::Unsupported(command_type.to_string())),
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum RemoteCommandError {
    #[error("不支持的远程命令：{0}")]
    Unsupported(String),
    #[error("执行远程命令失败：{command_type}: {message}")]
    Execute {
        command_type: String,
        message: String,
    },
}

impl RemoteCommandError {
    fn execute(command_type: &str, error: impl ToString) -> Self {
        Self::Execute {
            command_type: command_type.to_string(),
            message: error.to_string(),
        }
    }
}

fn run_bootstrap_command(config: &AgentConfig) -> Result<String, RemoteCommandError> {
    let result = crate::agent::run_once(config)
        .map_err(|error| RemoteCommandError::execute(REMOTE_COMMAND_SCRIPT_RUN_BOOTSTRAP, error))?;
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

fn start_lua_command() -> Result<String, RemoteCommandError> {
    let config = set_lua_enabled(true)
        .map_err(|error| RemoteCommandError::execute(REMOTE_COMMAND_SCRIPT_START, error))?;
    let result = crate::agent::run_once(&config)
        .map_err(|error| RemoteCommandError::execute(REMOTE_COMMAND_SCRIPT_START, error))?;
    let script = result
        .envelope
        .data
        .current_script
        .as_deref()
        .unwrap_or("无");
    Ok(format!(
        "Lua 已启动并执行一次：script={} message_id={}",
        script, result.envelope.message_id
    ))
}

fn stop_lua_command() -> Result<String, RemoteCommandError> {
    let config = set_lua_enabled(false)
        .map_err(|error| RemoteCommandError::execute(REMOTE_COMMAND_SCRIPT_STOP, error))?;
    Ok(format!(
        "Lua 已停止：script={}；Client monitor 仍保持在线和接收命令",
        config.lua.bootstrap_name
    ))
}

fn set_lua_enabled(enabled: bool) -> Result<AgentConfig, Box<dyn std::error::Error>> {
    let config_path = crate::config::default_config_path();
    crate::config::ensure_config_exists(&config_path)?;
    let mut config = AgentConfig::load_file_from_path(&config_path)?;
    config.lua.enabled = enabled;
    config.save_to_path(&config_path)?;
    Ok(config)
}

fn lua_status_summary(config: &AgentConfig) -> String {
    format!(
        "Lua 状态：enabled={} script={} path={} security_enabled={} permissions={}",
        config.lua.enabled,
        config.lua.bootstrap_name,
        config.lua.bootstrap_path.display(),
        config.script_security.enabled,
        config.script_security.allowed_permissions.join(",")
    )
}

#[cfg(test)]
mod tests {
    use crate::config::{
        AgentConfig, ClientConfig, DmConfig, LuaConfig, ScriptSecurityConfig, ServerConfig,
    };
    use std::path::PathBuf;

    #[test]
    fn unsupported_remote_command_is_rejected() {
        let error =
            super::execute_remote_command("shell.exec", &serde_json::json!({}), &test_config())
                .expect_err("must reject shell");
        assert_eq!(
            error,
            super::RemoteCommandError::Unsupported("shell.exec".to_string())
        );
    }

    fn test_config() -> AgentConfig {
        AgentConfig {
            client: ClientConfig {
                id: "remote-test-client".to_string(),
                display_name: "Remote Test Client".to_string(),
                group: "test".to_string(),
                tags: Vec::new(),
            },
            lua: LuaConfig {
                enabled: true,
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
