use std::path::PathBuf;

pub fn default_config_path() -> PathBuf {
    let cwd_config = PathBuf::from("config/client-agent.toml");
    if cwd_config.exists() {
        return cwd_config;
    }

    // 开发期允许从 workspace 根目录运行 `cargo run -p client-agent`。
    // 输入：编译期 crate 目录。
    // 输出：client-agent 模块内的默认配置路径。
    // 边界：后续接入 CLI 参数后，应由用户显式传入配置路径。
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("config")
        .join("client-agent.toml")
}
