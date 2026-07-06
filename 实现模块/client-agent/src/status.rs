use crate::config::AgentConfig;
use crate::lua_host::ScriptRunReport;
use shared_types::{ClientRuntimeInfo, ClientScriptInfo, ClientServerInfo, ClientStatus};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentStatusSnapshot {
    client_id: String,
    current_script: Option<String>,
    runtime: ClientRuntimeInfo,
    script: ClientScriptInfo,
    server: ClientServerInfo,
}

impl AgentStatusSnapshot {
    pub fn from_script_report(config: &AgentConfig, report: &ScriptRunReport) -> Self {
        // 状态映射集中在本模块，避免 main.rs 直接拼协议字段。
        // 输入：已合并环境变量覆盖后的 AgentConfig，以及 Lua 执行报告。
        // 输出：可转换为 shared-types::ClientStatus 的真实状态快照。
        // 边界：只暴露运行与配置摘要，不输出签名私钥、真实账号或本机敏感路径。
        Self {
            client_id: config.client.id.clone(),
            current_script: Some(report.script_name.clone()),
            runtime: ClientRuntimeInfo {
                release_version: framework_release_version(),
                os: std::env::consts::OS.to_string(),
                arch: std::env::consts::ARCH.to_string(),
                process_id: std::process::id(),
            },
            script: ClientScriptInfo {
                bootstrap_name: config.lua.bootstrap_name.clone(),
                instruction_limit: report.instruction_limit,
                security_enabled: config.script_security.enabled,
                allowed_permissions: config.script_security.allowed_permissions.clone(),
            },
            server: ClientServerInfo {
                report_enabled: config.server.enabled,
                report_target: report_target(config),
            },
        }
    }

    pub fn into_client_status(self) -> ClientStatus {
        let mut status = ClientStatus::new(self.client_id);
        status.current_script = self.current_script;
        status.runtime = self.runtime;
        status.script = self.script;
        status.server = self.server;
        status
    }
}

fn framework_release_version() -> String {
    include_str!("../../../VERSION").trim().to_string()
}

fn report_target(config: &AgentConfig) -> String {
    if !config.server.enabled {
        return "disabled".to_string();
    }

    format!(
        "{}:{}{}",
        config.server.host, config.server.port, config.server.status_path
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{
        AgentConfig, ClientConfig, DmConfig, LuaConfig, ScriptSecurityConfig, ServerConfig,
    };
    use std::path::PathBuf;

    #[test]
    fn status_snapshot_includes_runtime_script_and_server_summary() {
        let config = AgentConfig {
            client: ClientConfig {
                id: "client-a".to_string(),
            },
            lua: LuaConfig {
                bootstrap_name: "bootstrap".to_string(),
                bootstrap_path: PathBuf::from("scripts/bootstrap.lua"),
                instruction_limit: 10_000,
            },
            script_security: ScriptSecurityConfig {
                enabled: true,
                manifest_path: PathBuf::from("scripts/bootstrap.manifest.json"),
                trusted_signer_public_key:
                    "1111111111111111111111111111111111111111111111111111111111111111".to_string(),
                allowed_permissions: vec!["host.log".to_string(), "config.read".to_string()],
            },
            dm: DmConfig {
                bridge_path: PathBuf::from("target/dm-bridge/Win32/DmBridge.dll"),
            },
            server: ServerConfig {
                enabled: true,
                host: "127.0.0.1".to_string(),
                port: 18080,
                status_path: "/api/client/status".to_string(),
                connect_timeout_ms: 3000,
            },
        };
        let report = ScriptRunReport {
            script_name: "bootstrap".to_string(),
            script_path: PathBuf::from("scripts/bootstrap.lua"),
            result: "ok".to_string(),
            instruction_limit: 10_000,
        };

        let status = AgentStatusSnapshot::from_script_report(&config, &report).into_client_status();

        assert_eq!(status.client_id, "client-a");
        assert_eq!(status.current_script, Some("bootstrap".to_string()));
        assert_eq!(status.runtime.release_version, framework_release_version());
        assert_eq!(status.script.bootstrap_name, "bootstrap");
        assert!(status.script.security_enabled);
        assert_eq!(
            status.script.allowed_permissions,
            vec!["host.log".to_string(), "config.read".to_string()]
        );
        assert!(status.server.report_enabled);
        assert_eq!(
            status.server.report_target,
            "127.0.0.1:18080/api/client/status"
        );
    }
}
