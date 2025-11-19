use chrono::{DateTime, FixedOffset, Utc};

/// 自定义时间格式化器（使用中国时区 UTC+8）
struct ChinaTimer;

impl tracing_subscriber::fmt::time::FormatTime for ChinaTimer {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        // 获取当前 UTC 时间
        let utc = Utc::now();
        // 转换为中国时区 (UTC+8)
        let china_offset = FixedOffset::east_opt(8 * 3600).unwrap();
        let china_time: DateTime<FixedOffset> = utc.with_timezone(&china_offset);
        // 格式化为 RFC3339 格式，带时区信息
        write!(w, "{}", china_time.format("%Y-%m-%d %H:%M:%S%.3f %z"))
    }
}

/// 初始化日志系统
/// 
/// 从环境变量 `RUST_LOG` 读取日志级别，如果未设置则默认使用 `info` 级别
/// 
/// 日志级别（从低到高）：
/// - `trace`: 最详细，用于追踪程序执行流程
/// - `debug`: 调试信息，用于开发调试（SQL 查询会在此级别显示）
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
/// // RUST_LOG=debug cargo run  // 显示 debug 及以上级别（包括 SQL 查询）
/// // RUST_LOG=warn cargo run   // 只显示警告和错误
/// ```
pub fn init_logging() {
    // 从环境变量 RUST_LOG 读取日志级别，如果未设置则默认使用 info
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    
    // 配置日志输出格式，使用更美观的格式
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)  // 应用日志级别过滤
        .with_target(true)            // 显示目标模块路径（如 axum_demo::config）
        .with_thread_ids(false)       // 开发环境可以关闭线程 ID，使日志更简洁
        .with_thread_names(false)     // 关闭线程名称
        .with_file(false)             // 关闭文件名（可选，如果需要可以开启）
        .with_line_number(false)      // 关闭行号（可选，如果需要可以开启）
        .with_ansi(true)              // 启用颜色输出（终端支持时）
        .with_timer(ChinaTimer)       // 使用自定义时区（中国时区 UTC+8）
        .pretty()                     // 使用更美观的格式（带颜色和缩进）
        .init();                      // 初始化日志系统（全局只能调用一次）
}

