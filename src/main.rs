use tracing_subscriber;
use axum_demo::{config::Config, database::create_connection, routes::create_router, AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 如果没有设置 RUST_LOG 环境变量，默认使用 info 级别
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(true) // 显示目标模块
        .with_thread_ids(true) // 显示线程 ID
        .init();

    // 加载配置
    let config = Config::from_env()?;
    tracing::info!("配置加载成功: {:?}", config);

    // 创建数据库连接
    let db = create_connection(&config.database).await?;
    tracing::info!("数据库连接创建成功");

    // 创建应用状态
    let state = AppState {
        db,
        config: config.clone(),
    };

    // 创建路由
    let app = create_router(state);

    // 启动服务器
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.server.host, config.server.port))
        .await?;

    tracing::info!("服务器启动在 http://{}:{}", config.server.host, config.server.port);

    axum::serve(listener, app).await?;

    Ok(())
}
