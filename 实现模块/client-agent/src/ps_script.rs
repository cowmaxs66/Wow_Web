use std::fs;
use std::io;
use std::path::Path;

pub fn write_utf8_bom(path: &Path, script: &str) -> io::Result<()> {
    let mut bytes = Vec::with_capacity(3 + script.len());
    bytes.extend_from_slice(&[0xEF, 0xBB, 0xBF]);
    bytes.extend_from_slice(script.as_bytes());

    // Windows PowerShell 5.1 会把无 BOM UTF-8 脚本当作系统 ANSI 读取。
    // 输入：运行时生成的 PowerShell 脚本文本。
    // 输出：带 UTF-8 BOM 的 .ps1 文件。
    // 边界：PowerShell 7 也能正常读取 UTF-8 BOM，因此这里统一写 BOM。
    fs::write(path, bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn writes_utf8_bom_before_script_content() {
        let path = std::env::temp_dir().join(format!(
            "wow-ps-script-{}-{}.ps1",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("clock must be valid")
                .as_nanos()
        ));

        write_utf8_bom(&path, "Write-Host '测试'").expect("script must write");
        let bytes = fs::read(&path).expect("script must read");

        assert_eq!(&bytes[..3], &[0xEF, 0xBB, 0xBF]);
        assert!(String::from_utf8_lossy(&bytes).contains("测试"));

        let _ = fs::remove_file(path);
    }
}
