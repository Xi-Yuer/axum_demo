use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use crate::config::DatabaseConfig;

/// 创建数据库连接
/// 
/// 配置了连接池、超时时间和 SQL 日志记录
pub async fn create_connection(config: &DatabaseConfig) -> Result<DatabaseConnection, DbErr> {
    // 使用 ConnectOptions 来配置连接参数
    let mut opt = ConnectOptions::new(&config.url);
    
    // 配置连接池
    opt.max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .connect_timeout(config.connection_timeout)
        .idle_timeout(config.idle_timeout)
        .max_lifetime(config.max_lifetime)
        .sqlx_logging(true);  // 启用 SQL 日志记录（SQL 查询会在 debug 级别显示）
    
    // 创建数据库连接
    let db = Database::connect(opt).await?;
    
    tracing::info!(
        "数据库连接创建成功 - 连接池: {}-{} 连接, 超时: {:?}",
        config.min_connections,
        config.max_connections,
        config.connection_timeout
    );
    
    Ok(db)
}

