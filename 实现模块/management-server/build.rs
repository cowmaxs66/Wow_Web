use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-env-changed=MANAGEMENT_SERVER_EMBED_WEB_DIR");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let web_dir = env::var("MANAGEMENT_SERVER_EMBED_WEB_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| manifest_dir.join("../web-admin/dist"));
    println!("cargo:rerun-if-changed={}", web_dir.display());

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("out dir"));
    let output_path = out_dir.join("embedded_web_assets.rs");
    let assets = collect_assets(&web_dir)?;
    let mut output = String::new();

    output.push_str("pub static EMBEDDED_WEB_ASSETS: &[EmbeddedAsset] = &[\n");
    for asset in assets {
        output.push_str("    EmbeddedAsset {\n");
        output.push_str(&format!("        path: {:?},\n", asset.path));
        output.push_str(&format!(
            "        content_type: {:?},\n",
            asset.content_type
        ));
        output.push_str("        bytes: &[\n");
        for chunk in asset.bytes.chunks(24) {
            output.push_str("            ");
            for byte in chunk {
                output.push_str(&format!("{byte},"));
            }
            output.push('\n');
        }
        output.push_str("        ],\n");
        output.push_str("    },\n");
    }
    output.push_str("];\n");

    fs::write(output_path, output)
}

struct Asset {
    path: String,
    content_type: &'static str,
    bytes: Vec<u8>,
}

fn collect_assets(web_dir: &Path) -> io::Result<Vec<Asset>> {
    if !web_dir.join("index.html").exists() {
        return Ok(Vec::new());
    }

    let mut assets = Vec::new();
    collect_assets_from_dir(web_dir, web_dir, &mut assets)?;
    assets.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(assets)
}

fn collect_assets_from_dir(root: &Path, current: &Path, assets: &mut Vec<Asset>) -> io::Result<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_assets_from_dir(root, &path, assets)?;
            continue;
        }

        let relative = path
            .strip_prefix(root)
            .expect("asset path must be under web root")
            .to_string_lossy()
            .replace('\\', "/");
        let bytes = fs::read(&path)?;

        assets.push(Asset {
            content_type: content_type_for_path(&relative),
            path: relative,
            bytes,
        });
    }

    Ok(())
}

fn content_type_for_path(path: &str) -> &'static str {
    match Path::new(path).extension().and_then(|value| value.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("ico") => "image/x-icon",
        Some("wasm") => "application/wasm",
        _ => "application/octet-stream",
    }
}
