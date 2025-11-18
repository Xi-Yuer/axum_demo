use sea_orm::DatabaseConnection;
use uuid::Uuid;
use crate::entities::article::ActiveModel;
use crate::errors::{AppError, Result};
use crate::extractors::Pagination;
use crate::models::{ArticleResponse, CreateArticleRequest};
use crate::repositories::article_repository;
use crate::services::user_service::PagedResult;

/// 获取文章列表（带分页）
pub async fn list_articles(
    db: &DatabaseConnection,
    pagination: Pagination,
    user_id: Option<Uuid>,
) -> Result<PagedResult<Vec<ArticleResponse>>> {
    let offset = pagination.offset();
    let limit = pagination.limit();
    let is_public_only = user_id.is_none();
    
    let (articles, total) = article_repository::find_all_with_pagination(
        db,
        user_id,
        is_public_only,
        offset,
        limit,
    ).await?;
    
    let articles_response: Vec<ArticleResponse> = articles
        .into_iter()
        .map(ArticleResponse::from)
        .collect();
    
    Ok(PagedResult {
        list: articles_response,
        pagination: crate::services::user_service::PaginationInfo {
            page: pagination.page.unwrap_or(1),
            page_size: pagination.page_size.unwrap_or(10),
            total,
            total_pages: (total as f64 / limit as f64).ceil() as u64,
        },
    })
}

/// 根据 ID 获取文章
pub async fn get_article_by_id(
    db: &DatabaseConnection,
    article_id: Uuid,
) -> Result<ArticleResponse> {
    let article = article_repository::find_by_id(db, article_id).await?
        .ok_or(AppError::NotFound)?;
    
    Ok(ArticleResponse::from(article))
}

/// 创建文章
pub async fn create_article(
    db: &DatabaseConnection,
    user_id: Uuid,
    payload: CreateArticleRequest,
) -> Result<ArticleResponse> {
    let article_id = Uuid::new_v4();
    let now = chrono::Utc::now();
    
    let article = ActiveModel {
        id: sea_orm::Set(article_id),
        title: sea_orm::Set(payload.title),
        content: sea_orm::Set(payload.content),
        user_id: sea_orm::Set(Some(user_id)),
        is_public: sea_orm::Set(payload.is_public),
        created_at: sea_orm::Set(Some(now)),
    };
    
    let created_article = article_repository::create(db, article).await?;
    
    Ok(ArticleResponse::from(created_article))
}

/// 更新文章
pub async fn update_article(
    db: &DatabaseConnection,
    article_id: Uuid,
    payload: CreateArticleRequest,
) -> Result<ArticleResponse> {
    // 检查文章是否存在
    let existing_article = article_repository::find_by_id(db, article_id).await?
        .ok_or(AppError::NotFound)?;
    
    // 构建更新模型
    let mut article: ActiveModel = existing_article.into();
    article.title = sea_orm::Set(payload.title);
    article.content = sea_orm::Set(payload.content);
    if let Some(is_public) = payload.is_public {
        article.is_public = sea_orm::Set(Some(is_public));
    }
    
    let updated_article = article_repository::update(db, article_id, article).await?;
    
    Ok(ArticleResponse::from(updated_article))
}

/// 删除文章
pub async fn delete_article(
    db: &DatabaseConnection,
    article_id: Uuid,
) -> Result<()> {
    article_repository::delete(db, article_id).await
}

