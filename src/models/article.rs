use crate::entities::article::Model as ArticleEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 文章响应（前端返回）
#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleResponse {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
}

/// 创建文章请求
#[derive(Debug, Deserialize)]
pub struct CreateArticleRequest {
    pub title: String,
    pub content: String,
    pub is_public: Option<bool>,
}

impl From<ArticleEntity> for ArticleResponse {
    fn from(article: ArticleEntity) -> Self {
        ArticleResponse {
            id: article.id,
            title: article.title,
            content: article.content,
            created_at: article.created_at,
        }
    }
}
