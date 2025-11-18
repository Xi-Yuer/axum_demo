use sea_orm::{Database, DatabaseConnection, DbErr};
use crate::config::DatabaseConfig;

/// 创建数据库连接
pub async fn create_connection(config: &DatabaseConfig) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(&config.url).await?;
    Ok(db)
}

