use super::ScriptError;
use super::hash::is_sha256_hex;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ScriptManifest {
    pub schema_version: u16,
    pub script_id: String,
    pub version: String,
    pub entry: PathBuf,
    pub sha256: String,
    pub permissions: Vec<String>,
    pub signature: String,
}

impl ScriptManifest {
    pub fn load(path: &Path) -> Result<Self, ScriptError> {
        let content = fs::read_to_string(path).map_err(|error| ScriptError::read(path, error))?;
        let manifest: Self =
            serde_json::from_str(&content).map_err(|error| ScriptError::parse(path, error))?;
        manifest.validate(path)?;
        Ok(manifest)
    }

    pub fn entry_path(&self, manifest_path: &Path) -> Result<PathBuf, ScriptError> {
        if self.entry.is_absolute() {
            return Err(ScriptError::validate(
                manifest_path,
                "entry 必须是相对路径，禁止使用绝对路径",
            ));
        }

        let Some(manifest_dir) = manifest_path.parent() else {
            return Err(ScriptError::validate(
                manifest_path,
                "manifest 路径缺少父目录",
            ));
        };

        Ok(manifest_dir.join(&self.entry))
    }

    pub fn validate_script_id(
        &self,
        manifest_path: &Path,
        expected_script_id: &str,
    ) -> Result<(), ScriptError> {
        if self.script_id != expected_script_id {
            return Err(ScriptError::validate(
                manifest_path,
                format!(
                    "script_id 必须等于配置中的 bootstrap_name：{}",
                    expected_script_id
                ),
            ));
        }

        Ok(())
    }

    fn validate(&self, path: &Path) -> Result<(), ScriptError> {
        if self.schema_version != 1 {
            return Err(ScriptError::validate(path, "schema_version 当前必须是 1"));
        }

        if self.script_id.trim().is_empty() {
            return Err(ScriptError::validate(path, "script_id 不能为空"));
        }

        if self.version.trim().is_empty() {
            return Err(ScriptError::validate(path, "version 不能为空"));
        }

        if self.entry.as_os_str().is_empty() {
            return Err(ScriptError::validate(path, "entry 不能为空"));
        }

        if !is_sha256_hex(&self.sha256) {
            return Err(ScriptError::validate(path, "sha256 必须是 64 位十六进制"));
        }

        if !is_hex_with_len(&self.signature, 128) {
            return Err(ScriptError::validate(
                path,
                "signature 必须是 128 位十六进制 Ed25519 签名",
            ));
        }

        if self
            .permissions
            .iter()
            .any(|permission| permission.trim().is_empty())
        {
            return Err(ScriptError::validate(path, "permissions 不能包含空权限"));
        }

        Ok(())
    }

    pub fn signing_payload(&self) -> String {
        let mut permissions = self.permissions.clone();
        permissions.sort();
        format!(
            "wow-framework-script-manifest-v1\nschema_version={}\nscript_id={}\nversion={}\nentry={}\nsha256={}\npermissions={}\n",
            self.schema_version,
            self.script_id,
            self.version,
            self.entry.to_string_lossy().replace('\\', "/"),
            self.sha256.to_ascii_lowercase(),
            permissions.join(",")
        )
    }
}

fn is_hex_with_len(value: &str, expected_len: usize) -> bool {
    value.len() == expected_len && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}
