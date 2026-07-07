use std::fs;
use std::io;
use std::path::Path;

const UTF8_BOM: &[u8; 3] = b"\xEF\xBB\xBF";

pub fn write_utf8_bom(path: &Path, script: &str) -> io::Result<()> {
    let mut content = Vec::with_capacity(UTF8_BOM.len() + script.len());
    content.extend_from_slice(UTF8_BOM);
    content.extend_from_slice(script.as_bytes());
    fs::write(path, content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_utf8_bom_before_script_content() {
        let path =
            std::env::temp_dir().join(format!("wow-server-ps-script-{}.ps1", std::process::id()));

        write_utf8_bom(&path, "Write-Host 'ok'").expect("script must be written");
        let bytes = fs::read(&path).expect("script must be readable");

        assert!(bytes.starts_with(UTF8_BOM));
        assert_eq!(&bytes[3..], b"Write-Host 'ok'");

        let _ = fs::remove_file(path);
    }
}
