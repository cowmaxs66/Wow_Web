use super::ScriptError;
use super::manifest::ScriptManifest;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use std::path::Path;

pub fn verify_manifest_signature(
    manifest_path: &Path,
    manifest: &ScriptManifest,
    public_key_hex: &str,
) -> Result<(), ScriptError> {
    let public_key = decode_hex_array::<32>(manifest_path, public_key_hex, "公钥")?;
    let signature = decode_hex_array::<64>(manifest_path, &manifest.signature, "签名")?;
    let verifying_key = VerifyingKey::from_bytes(&public_key)
        .map_err(|error| ScriptError::security(manifest_path, error.to_string()))?;
    let signature = Signature::from_bytes(&signature);

    // 签名只覆盖 manifest 的安全关键字段，不覆盖 JSON 空白和字段顺序。
    // 输入：固定格式 signing_payload 与 Ed25519 公钥/签名。
    // 输出：验证通过或拒绝加载脚本。
    // 边界：签名不覆盖 Lua 文件正文；Lua 正文由 sha256 字段绑定。
    verifying_key
        .verify(manifest.signing_payload().as_bytes(), &signature)
        .map_err(|error| ScriptError::security(manifest_path, error.to_string()))
}

fn decode_hex_array<const N: usize>(
    path: &Path,
    value: &str,
    label: &str,
) -> Result<[u8; N], ScriptError> {
    if value.len() != N * 2 {
        return Err(ScriptError::security(
            path,
            format!("{label} 长度必须是 {} 位十六进制", N * 2),
        ));
    }

    let mut bytes = [0u8; N];
    for index in 0..N {
        let part = &value[index * 2..index * 2 + 2];
        bytes[index] = u8::from_str_radix(part, 16)
            .map_err(|_| ScriptError::security(path, format!("{label} 包含非十六进制字符")))?;
    }

    Ok(bytes)
}
