use tracing_subscriber::EnvFilter;

pub fn init() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // 日志初始化允许测试或外层宿主提前注册 subscriber。
    // 输入：RUST_LOG 环境变量，未设置时默认 info。
    // 输出：结构化 stderr 日志。
    // 边界：重复初始化不应导致 Agent 启动失败。
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .try_init();
}
