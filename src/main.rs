use axum_demo::{
    config::Config, database::create_connection, logging, routes::create_router, AppState,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志系统
    logging::init_logging();

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
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.server.host, config.server.port))
            .await?;

    tracing::info!(
        "服务器启动在 http://{}:{}",
        config.server.host,
        config.server.port
    );

    axum::serve(listener, app).await?;

    Ok(())
}
