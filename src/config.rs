use serde::Deserialize;
use std::env;
use std::time::Duration;

/// 应用配置
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
}

/// 服务器配置
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

/// 数据库配置
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub connection_timeout: Duration,
    pub query_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
    pub min_connections: u32,
}

/// JWT 配置
#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_days: i64,
}

impl Config {
    /// 从环境变量加载配置
    pub fn from_env() -> Result<Self, anyhow::Error> {
        dotenv::dotenv().ok();

        Ok(Config {
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "3000".to_string())
                    .parse()
                    .unwrap_or(3000),
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "mysql://root:password@localhost:3306/axum_db".to_string()),
                max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
                connection_timeout: Duration::from_secs(10),
                query_timeout: Duration::from_secs(10),
                idle_timeout: Duration::from_secs(10),
                max_lifetime: Duration::from_secs(10),
                min_connections: 10,
            },
            jwt: JwtConfig {
                secret: env::var("JWT_SECRET")
                    .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string()),
                expiration_days: env::var("JWT_EXPIRATION_DAYS")
                    .unwrap_or_else(|_| "7".to_string())
                    .parse()
                    .unwrap_or(7),
            },
        })
    }
}
