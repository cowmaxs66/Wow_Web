use crate::config::AgentConfig;
use crate::lua_host::ScriptRunReport;
use shared_types::{
    ClientIdentityInfo, ClientRuntimeInfo, ClientScriptInfo, ClientServerInfo, ClientStatus,
};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentStatusSnapshot {
    client_id: String,
    online: bool,
    current_script: Option<String>,
    identity: ClientIdentityInfo,
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
            online: true,
            current_script: Some(report.script_name.clone()),
            identity: identity_info(config),
            runtime: runtime_info(),
            script: script_info(config, report.instruction_limit),
            server: server_info(config),
        }
    }

    pub fn online_without_script(config: &AgentConfig) -> Self {
        // Lua 被管理端停用时，Client 仍然保持在线和可接收命令。
        // 输入：当前 AgentConfig。
        // 输出：不执行 Lua 的在线状态快照。
        // 边界：只影响脚本执行，不停止 monitor、托盘、消息和远程命令轮询。
        Self {
            client_id: config.client.id.clone(),
            online: true,
            current_script: None,
            identity: identity_info(config),
            runtime: runtime_info(),
            script: script_info(config, config.lua.instruction_limit),
            server: server_info(config),
        }
    }

    pub fn offline(config: &AgentConfig) -> Self {
        // 离线状态用于 monitor 退出时主动通知 Server。
        // 输入：当前已合并环境变量的 AgentConfig。
        // 输出：online=false 的状态快照，保留版本、架构、脚本配置与上报目标摘要。
        // 边界：不重新执行 Lua，避免退出阶段因为脚本异常阻塞离线回写。
        Self {
            client_id: config.client.id.clone(),
            online: false,
            current_script: None,
            identity: identity_info(config),
            runtime: runtime_info(),
            script: script_info(config, config.lua.instruction_limit),
            server: server_info(config),
        }
    }

    pub fn into_client_status(self) -> ClientStatus {
        let mut status = ClientStatus::new(self.client_id);
        status.online = self.online;
        status.current_script = self.current_script;
        status.identity = self.identity;
        status.runtime = self.runtime;
        status.script = self.script;
        status.server = self.server;
        status
    }
}

fn identity_info(config: &AgentConfig) -> ClientIdentityInfo {
    ClientIdentityInfo {
        display_name: config.client.display_name.trim().to_string(),
        group: config.client.group.trim().to_string(),
        tags: normalize_tags(&config.client.tags),
    }
}

fn normalize_tags(tags: &[String]) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut normalized = Vec::new();

    for tag in tags {
        let tag = tag.trim();
        if tag.is_empty() || !seen.insert(tag.to_string()) {
            continue;
        }

        // 标签顺序来自用户配置，去重只防止 Web 过滤和远程配置重复展示。
        // 输入：client.tags 字符串数组。
        // 输出：去空、去重、保持原顺序的标签列表。
        // 边界：这里只清理展示和上报数据，不修改磁盘配置文件。
        normalized.push(tag.to_string());
    }

    normalized
}

fn framework_release_version() -> String {
    include_str!("../../../VERSION").trim().to_string()
}

fn runtime_info() -> ClientRuntimeInfo {
    ClientRuntimeInfo {
        release_version: framework_release_version(),
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        process_id: std::process::id(),
    }
}

fn script_info(config: &AgentConfig, instruction_limit: u32) -> ClientScriptInfo {
    ClientScriptInfo {
        bootstrap_name: config.lua.bootstrap_name.clone(),
        enabled: config.lua.enabled,
        instruction_limit,
        security_enabled: config.script_security.enabled,
        allowed_permissions: config.script_security.allowed_permissions.clone(),
    }
}

fn server_info(config: &AgentConfig) -> ClientServerInfo {
    ClientServerInfo {
        report_enabled: config.server.enabled,
        report_target: report_target(config),
    }
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
                display_name: "主控一号".to_string(),
                group: "raid-a".to_string(),
                tags: vec!["dm".to_string(), "dm".to_string(), "test".to_string()],
            },
            lua: LuaConfig {
                enabled: true,
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
            log_messages: Vec::new(),
            instruction_limit: 10_000,
        };

        let status = AgentStatusSnapshot::from_script_report(&config, &report).into_client_status();

        assert_eq!(status.client_id, "client-a");
        assert_eq!(status.identity.display_name, "主控一号");
        assert_eq!(status.identity.group, "raid-a");
        assert_eq!(
            status.identity.tags,
            vec!["dm".to_string(), "test".to_string()]
        );
        assert_eq!(status.current_script, Some("bootstrap".to_string()));
        assert_eq!(status.runtime.release_version, framework_release_version());
        assert_eq!(status.script.bootstrap_name, "bootstrap");
        assert!(status.script.enabled);
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

    #[test]
    fn offline_snapshot_preserves_safe_summary_without_script_run() {
        let config = AgentConfig {
            client: ClientConfig {
                id: "client-a".to_string(),
                display_name: "离线测试机".to_string(),
                group: "default".to_string(),
                tags: Vec::new(),
            },
            lua: LuaConfig {
                enabled: true,
                bootstrap_name: "bootstrap".to_string(),
                bootstrap_path: PathBuf::from("scripts/bootstrap.lua"),
                instruction_limit: 10_000,
            },
            script_security: ScriptSecurityConfig {
                enabled: true,
                manifest_path: PathBuf::from("scripts/bootstrap.manifest.json"),
                trusted_signer_public_key:
                    "1111111111111111111111111111111111111111111111111111111111111111".to_string(),
                allowed_permissions: vec!["host.log".to_string()],
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

        let status = AgentStatusSnapshot::offline(&config).into_client_status();

        assert_eq!(status.client_id, "client-a");
        assert!(!status.online);
        assert_eq!(status.identity.display_name, "离线测试机");
        assert_eq!(status.current_script, None);
        assert_eq!(status.script.bootstrap_name, "bootstrap");
        assert_eq!(
            status.server.report_target,
            "127.0.0.1:18080/api/client/status"
        );
    }
}
