use sea_orm::DatabaseConnection;
use uuid::Uuid;
use crate::entities::user::ActiveModel;
use crate::errors::{AppError, Result};
use crate::extractors::Pagination;
use crate::models::{UpdateUserRequest, UserResponse};
use crate::repositories::user_repository;

/// 分页结果
#[derive(Debug, serde::Serialize)]
pub struct PagedResult<T> {
    pub list: T,
    pub pagination: PaginationInfo,
}

#[derive(Debug, serde::Serialize)]
pub struct PaginationInfo {
    pub page: u64,
    pub page_size: u64,
    pub total: u64,
    pub total_pages: u64,
}

/// 获取用户列表（带分页）
pub async fn list_users(
    db: &DatabaseConnection,
    pagination: Pagination,
) -> Result<PagedResult<Vec<UserResponse>>> {
    let page = pagination.page.unwrap_or(1);
    let page_size = pagination.limit();
    
    let (users, total) = user_repository::find_all_with_pagination(db, page, page_size).await?;
    
    let users_response: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();
    
    Ok(PagedResult {
        list: users_response,
        pagination: PaginationInfo {
            page: pagination.page.unwrap_or(1),
            page_size: pagination.page_size.unwrap_or(10),
            total,
            total_pages: (total as f64 / page_size as f64).ceil() as u64,
        },
    })
}

/// 根据 ID 获取用户
pub async fn get_user_by_id(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<UserResponse> {
    let user = user_repository::find_by_id(db, user_id).await?
        .ok_or(AppError::NotFound)?;
    
    Ok(UserResponse::from(user))
}

/// 更新用户信息
pub async fn update_user(
    db: &DatabaseConnection,
    user_id: Uuid,
    payload: UpdateUserRequest,
) -> Result<UserResponse> {
    // 检查用户是否存在
    let existing_user = user_repository::find_by_id(db, user_id).await?
        .ok_or(AppError::NotFound)?;
    
    // 构建更新模型
    let mut user: ActiveModel = existing_user.into();
    
    if let Some(username) = payload.username {
        user.username = sea_orm::Set(username);
    }
    if let Some(email) = payload.email {
        user.email = sea_orm::Set(email);
    }
    user.updated_at = sea_orm::Set(chrono::Utc::now());
    
    // 更新用户
    let updated_user = user_repository::update(db, user_id, user).await?;
    
    Ok(UserResponse::from(updated_user))
}

/// 删除用户
pub async fn delete_user(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<()> {
    user_repository::delete(db, user_id).await
}

