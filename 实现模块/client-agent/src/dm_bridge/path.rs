use super::error::DmBridgeError;
use std::fs;
use std::path::{Path, PathBuf};

const IMAGE_FILE_MACHINE_I386: u16 = 0x014c;
const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;

pub fn resolve_bridge_path(path: &Path) -> PathBuf {
    if path.is_absolute() {
        return path.to_path_buf();
    }

    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path)
}

pub fn validate_process_architecture(path: &Path) -> Result<(), DmBridgeError> {
    let machine = read_pe_machine(path)?;
    let dll_arch = dll_arch_label(machine).to_string();
    let process_arch = process_arch_label().to_string();

    // Windows 进程不能跨位数加载 DLL。
    // 输入：DmBridge.dll 的 PE Machine 字段和当前 Rust 进程位数。
    // 输出：位数匹配则继续，位数不匹配则提前给出明确错误。
    // 边界：未知 Machine 交给系统加载器处理，避免误杀未来平台。
    if (cfg!(target_pointer_width = "64") && machine == IMAGE_FILE_MACHINE_I386)
        || (cfg!(target_pointer_width = "32") && machine == IMAGE_FILE_MACHINE_AMD64)
    {
        return Err(DmBridgeError::UnsupportedArchitecture {
            path: path.to_path_buf(),
            dll_arch,
            process_arch,
        });
    }

    Ok(())
}

fn read_pe_machine(path: &Path) -> Result<u16, DmBridgeError> {
    let bytes =
        fs::read(path).map_err(|error| DmBridgeError::load_failed(path, error.to_string()))?;
    if bytes.len() < 0x40 || &bytes[..2] != b"MZ" {
        return Err(DmBridgeError::load_failed(path, "不是有效的 PE 文件"));
    }

    let pe_offset = read_u32(path, &bytes, 0x3c)? as usize;
    if bytes.len() < pe_offset + 6 || &bytes[pe_offset..pe_offset + 4] != b"PE\0\0" {
        return Err(DmBridgeError::load_failed(path, "PE 头不完整"));
    }

    read_u16(path, &bytes, pe_offset + 4)
}

fn read_u16(path: &Path, bytes: &[u8], offset: usize) -> Result<u16, DmBridgeError> {
    let value = bytes
        .get(offset..offset + 2)
        .ok_or_else(|| DmBridgeError::load_failed(path, "PE 字段越界"))?;
    Ok(u16::from_le_bytes([value[0], value[1]]))
}

fn read_u32(path: &Path, bytes: &[u8], offset: usize) -> Result<u32, DmBridgeError> {
    let value = bytes
        .get(offset..offset + 4)
        .ok_or_else(|| DmBridgeError::load_failed(path, "PE 字段越界"))?;
    Ok(u32::from_le_bytes([value[0], value[1], value[2], value[3]]))
}

fn dll_arch_label(machine: u16) -> &'static str {
    match machine {
        IMAGE_FILE_MACHINE_I386 => "32-bit",
        IMAGE_FILE_MACHINE_AMD64 => "64-bit",
        _ => "unknown",
    }
}

fn process_arch_label() -> &'static str {
    if cfg!(target_pointer_width = "64") {
        "64-bit"
    } else {
        "32-bit"
    }
}
