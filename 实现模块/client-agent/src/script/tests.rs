use super::*;
use crate::config::{
    AgentConfig, ClientConfig, DmConfig, LuaConfig, ScriptSecurityConfig, ServerConfig,
};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn relative_script_path_is_resolved_from_module_root() {
    let path = resolve_module_path(&PathBuf::from("scripts/bootstrap.lua"));

    assert!(path.ends_with("scripts/bootstrap.lua"));
    assert!(path.is_absolute());
}

#[test]
fn secure_bootstrap_loads_manifest_and_permissions() {
    let workspace = create_test_workspace("secure_load");
    let script_path = workspace.join("bootstrap.lua");
    let script_content = "return 'ok'";
    fs::write(&script_path, script_content).expect("script must write");
    let manifest_path = write_manifest(
        &workspace,
        &sha256_hex(script_content.as_bytes()),
        &["host.log", "config.read"],
    );
    let config = test_config(&script_path, &manifest_path, &["host.log", "config.read"]);

    let script = ScriptSource::load_bootstrap(&config).expect("secure script must load");

    assert_eq!(script.name, "bootstrap");
    assert!(script.permissions.allows(PERMISSION_HOST_LOG));
    assert!(script.permissions.allows(PERMISSION_CONFIG_READ));
}

#[test]
fn secure_bootstrap_rejects_hash_mismatch() {
    let workspace = create_test_workspace("hash_mismatch");
    let script_path = workspace.join("bootstrap.lua");
    fs::write(&script_path, "return 'changed'").expect("script must write");
    let manifest_path = write_manifest(&workspace, &"0".repeat(64), &["host.log"]);
    let config = test_config(&script_path, &manifest_path, &["host.log"]);

    let error = ScriptSource::load_bootstrap(&config).expect_err("hash mismatch must fail");

    assert!(error.to_string().contains("sha256 不匹配"));
}

#[test]
fn secure_bootstrap_rejects_signature_mismatch() {
    let workspace = create_test_workspace("signature_mismatch");
    let script_path = workspace.join("bootstrap.lua");
    let script_content = "return 'ok'";
    fs::write(&script_path, script_content).expect("script must write");
    let manifest_path = write_manifest_with_signature(
        &workspace,
        &sha256_hex(script_content.as_bytes()),
        &["host.log"],
        &"0".repeat(128),
    );
    let config = test_config(&script_path, &manifest_path, &["host.log"]);

    let error = ScriptSource::load_bootstrap(&config).expect_err("signature mismatch must fail");

    assert!(error.to_string().contains("脚本安全校验失败"));
}

#[test]
fn secure_bootstrap_rejects_permission_outside_whitelist() {
    let workspace = create_test_workspace("permission_reject");
    let script_path = workspace.join("bootstrap.lua");
    let script_content = "return 'ok'";
    fs::write(&script_path, script_content).expect("script must write");
    let manifest_path = write_manifest(
        &workspace,
        &sha256_hex(script_content.as_bytes()),
        &["dm.access"],
    );
    let config = test_config(&script_path, &manifest_path, &["host.log"]);

    let error = ScriptSource::load_bootstrap(&config).expect_err("permission must fail");

    assert!(error.to_string().contains("未在配置白名单中"));
}

#[test]
fn shipped_dm_smoke_manifest_matches_script_and_permissions() {
    let module_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let script_path = module_dir.join("scripts/dm_smoke.lua");
    let manifest_path = module_dir.join("scripts/dm_smoke.manifest.json");
    let config = test_config_with_name(
        "dm-smoke",
        &script_path,
        &manifest_path,
        &["host.log", "dm.access"],
    );

    let script = ScriptSource::load_bootstrap(&config).expect("dm smoke manifest must load");

    assert_eq!(script.name, "dm-smoke");
    assert!(script.permissions.allows(PERMISSION_HOST_LOG));
    assert!(script.permissions.allows(PERMISSION_DM_ACCESS));
    assert!(!script.permissions.allows(PERMISSION_CONFIG_READ));
}

fn create_test_workspace(name: &str) -> PathBuf {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock must be valid")
        .as_nanos();
    let workspace = std::env::temp_dir().join(format!("wow_script_{name}_{now}"));
    fs::create_dir_all(&workspace).expect("test workspace must exist");
    workspace
}

fn write_manifest(workspace: &Path, hash: &str, permissions: &[&str]) -> PathBuf {
    let signature = sign_manifest_payload(hash, permissions);
    write_manifest_with_signature(workspace, hash, permissions, &signature)
}

fn write_manifest_with_signature(
    workspace: &Path,
    hash: &str,
    permissions: &[&str],
    signature: &str,
) -> PathBuf {
    let permission_json = permissions
        .iter()
        .map(|permission| format!("\"{permission}\""))
        .collect::<Vec<_>>()
        .join(", ");
    let manifest = format!(
        r#"{{
  "schema_version": 1,
  "script_id": "bootstrap",
  "version": "0.1.0",
  "entry": "bootstrap.lua",
  "sha256": "{hash}",
  "permissions": [{permission_json}],
  "signature": "{signature}"
}}"#
    );
    let manifest_path = workspace.join("bootstrap.manifest.json");
    fs::write(&manifest_path, manifest).expect("manifest must write");
    manifest_path
}

fn test_config(
    script_path: &Path,
    manifest_path: &Path,
    allowed_permissions: &[&str],
) -> AgentConfig {
    test_config_with_name("bootstrap", script_path, manifest_path, allowed_permissions)
}

fn test_config_with_name(
    bootstrap_name: &str,
    script_path: &Path,
    manifest_path: &Path,
    allowed_permissions: &[&str],
) -> AgentConfig {
    AgentConfig {
        client: ClientConfig {
            id: "script-test-client".to_string(),
            display_name: "Script Test Client".to_string(),
            group: "test".to_string(),
            tags: Vec::new(),
        },
        lua: LuaConfig {
            enabled: true,
            bootstrap_name: bootstrap_name.to_string(),
            bootstrap_path: script_path.to_path_buf(),
            instruction_limit: 1000,
        },
        script_security: ScriptSecurityConfig {
            enabled: true,
            manifest_path: manifest_path.to_path_buf(),
            trusted_signer_public_key: test_public_key_hex(),
            allowed_permissions: allowed_permissions
                .iter()
                .map(|permission| permission.to_string())
                .collect(),
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

fn sign_manifest_payload(hash: &str, permissions: &[&str]) -> String {
    use ed25519_dalek::{Signer, SigningKey};

    let mut permissions = permissions
        .iter()
        .map(|permission| permission.to_string())
        .collect::<Vec<_>>();
    permissions.sort();
    let payload = format!(
        "wow-framework-script-manifest-v1\nschema_version=1\nscript_id=bootstrap\nversion=0.1.0\nentry=bootstrap.lua\nsha256={}\npermissions={}\n",
        hash.to_ascii_lowercase(),
        permissions.join(",")
    );
    let signing_key = SigningKey::from_bytes(&[7u8; 32]);
    signing_key
        .sign(payload.as_bytes())
        .to_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn test_public_key_hex() -> String {
    use ed25519_dalek::SigningKey;

    let signing_key = SigningKey::from_bytes(&[7u8; 32]);
    signing_key
        .verifying_key()
        .to_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
