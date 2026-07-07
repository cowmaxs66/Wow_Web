use axum::body::Body;
use axum::http::{StatusCode, Uri, header};
use axum::response::{IntoResponse, Response};

pub struct EmbeddedAsset {
    pub path: &'static str,
    pub content_type: &'static str,
    pub bytes: &'static [u8],
}

include!(concat!(env!("OUT_DIR"), "/embedded_web_assets.rs"));

pub fn has_assets() -> bool {
    !EMBEDDED_WEB_ASSETS.is_empty()
}

pub fn asset_count() -> usize {
    EMBEDDED_WEB_ASSETS.len()
}

pub async fn serve_embedded_web(uri: Uri) -> Response {
    let request_path = normalize_request_path(uri.path());
    let asset = find_asset(&request_path).or_else(|| {
        if should_fallback_to_index(&request_path) {
            find_asset("index.html")
        } else {
            None
        }
    });

    match asset {
        Some(asset) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, asset.content_type)],
            Body::from(asset.bytes),
        )
            .into_response(),
        None => (StatusCode::NOT_FOUND, "embedded web asset not found").into_response(),
    }
}

fn find_asset(path: &str) -> Option<&'static EmbeddedAsset> {
    EMBEDDED_WEB_ASSETS.iter().find(|asset| asset.path == path)
}

fn normalize_request_path(path: &str) -> String {
    let trimmed = path.trim_start_matches('/');
    if trimmed.is_empty() {
        "index.html".to_string()
    } else {
        trimmed.to_string()
    }
}

fn should_fallback_to_index(path: &str) -> bool {
    !path.rsplit('/').next().unwrap_or_default().contains('.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_path_maps_to_index() {
        assert_eq!(normalize_request_path("/"), "index.html");
        assert_eq!(normalize_request_path("/assets/app.js"), "assets/app.js");
    }

    #[test]
    fn spa_paths_fallback_but_asset_paths_do_not() {
        assert!(should_fallback_to_index("clients/local-dev-client"));
        assert!(!should_fallback_to_index("assets/app.js"));
    }
}
