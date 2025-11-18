/// 初始化日志系统
/// 
/// 从环境变量 `RUST_LOG` 读取日志级别，如果未设置则默认使用 `info` 级别
/// 
/// 日志级别（从低到高）：
/// - `trace`: 最详细，用于追踪程序执行流程
/// - `debug`: 调试信息，用于开发调试
/// - `info`: 一般信息（默认），用于记录重要事件
/// - `warn`: 警告信息，用于记录潜在问题
/// - `error`: 错误信息，用于记录错误
/// 
/// # 使用示例
/// 
/// ```rust
/// // 在 main 函数开始处调用
/// init_logging();
/// 
/// // 通过环境变量控制日志级别
/// // RUST_LOG=debug cargo run  // 显示 debug 及以上级别
/// // RUST_LOG=warn cargo run   // 只显示警告和错误
/// ```
pub fn init_logging() {
    // 从环境变量 RUST_LOG 读取日志级别，如果未设置则默认使用 info
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    
    // 配置日志输出格式
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)  // 应用日志级别过滤
        .with_target(true)            // 显示目标模块路径（如 axum_demo::config）
        .with_thread_ids(true)        // 显示线程 ID（用于调试并发问题）
        .init();                      // 初始化日志系统（全局只能调用一次）
}

