use crate::errors::Result;
use crate::extractors::{AuthUser, OptionalAuthUser, Pagination};
use crate::models::{ArticleResponse, CreateArticleRequest};
use crate::response::ApiResponse;
use crate::services::{article_service, PagedResult};
use crate::AppState;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;

/// 获取文章列表（示例：使用 From<Result> trait）
pub async fn list_articles_simple(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
    optional_user: OptionalAuthUser,
) -> ApiResponse<Vec<ArticleResponse>> {
    let user_id = optional_user.user_id();
    let result: Result<Vec<ArticleResponse>> = async {
        let articles = article_service::list_articles(&state.db, pagination, user_id).await?;
        Ok(articles.list)
    }
    .await;

    result.into()
}

/// 获取文章列表（推荐方式：使用 ? 运算符）
pub async fn list_articles(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
    optional_user: OptionalAuthUser,
) -> Result<ApiResponse<PagedResult<Vec<ArticleResponse>>>> {
    let result =
        article_service::list_articles(&state.db, pagination, optional_user.user_id()).await?;

    Ok(ApiResponse::success(PagedResult {
        list: result.list,
        pagination: result.pagination,
    }))
}

/// 根据 ID 获取文章（简单方式）
pub async fn get_article_simple(
    State(state): State<AppState>,
    Path(article_id): Path<Uuid>,
    _optional_user: OptionalAuthUser,
) -> ApiResponse<ArticleResponse> {
    let result: Result<ArticleResponse> =
        async { article_service::get_article_by_id(&state.db, article_id).await }.await;

    result.into()
}

/// 根据 ID 获取文章（推荐方式）
pub async fn get_article(
    State(state): State<AppState>,
    Path(article_id): Path<Uuid>,
    _optional_user: OptionalAuthUser,
) -> Result<ApiResponse<ArticleResponse>> {
    let article = article_service::get_article_by_id(&state.db, article_id).await?;

    Ok(ApiResponse::success(article))
}

/// 创建文章（需要认证）
pub async fn create_article(
    State(state): State<AppState>,
    auth_user: AuthUser, // 直接使用 AuthUser，更清晰
    Json(payload): Json<CreateArticleRequest>,
) -> Result<ApiResponse<ArticleResponse>> {
    let article = article_service::create_article(&state.db, auth_user.user_id, payload).await?;

    Ok(ApiResponse::success_with_message(article, "文章创建成功"))
}
