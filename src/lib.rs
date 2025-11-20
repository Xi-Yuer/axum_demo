// 库模块导出
pub mod config;
pub mod controllers;
pub mod database;
pub mod entities;
pub mod errors;
pub mod extractors;
pub mod jwt;
pub mod logging;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod response;
pub mod routes;
pub mod services;
pub mod utils;

use crate::config::Config;
use axum::extract::FromRef;
use sea_orm::DatabaseConnection;

/// 应用状态（共享状态）
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: Config,
}

impl FromRef<AppState> for DatabaseConnection {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

impl FromRef<AppState> for Config {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}
