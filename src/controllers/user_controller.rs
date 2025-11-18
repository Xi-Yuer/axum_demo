use crate::errors::{AppError, Result};
use crate::extractors::{AuthUser, Pagination};
use crate::models::UpdateUserRequest;
use crate::response::ApiResponse;
use crate::services::user_service;
use crate::AppState;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;

/// 获取用户列表（带分页）
pub async fn list_users(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<ApiResponse<serde_json::Value>> {
    let result = user_service::list_users(&state.db, pagination).await?;

    Ok(ApiResponse::success(serde_json::json!({
        "list": result.list,
        "pagination": result.pagination,
    })))
}

/// 根据 ID 获取用户
pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<ApiResponse<crate::models::UserResponse>> {
    let user = user_service::get_user_by_id(&state.db, user_id).await?;

    Ok(ApiResponse::success(user))
}

/// 更新用户信息
pub async fn update_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    auth_user: AuthUser,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<ApiResponse<crate::models::UserResponse>> {
    // 只能更新自己的信息
    if auth_user.user_id != user_id {
        return Err(AppError::Forbidden);
    }

    let user = user_service::update_user(&state.db, user_id, payload).await?;

    Ok(ApiResponse::success_with_message(user, "更新成功"))
}

/// 删除用户
pub async fn delete_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    auth_user: AuthUser,
) -> Result<ApiResponse<()>> {
    // 只能删除自己的账户
    if auth_user.user_id != user_id {
        return Err(AppError::Forbidden);
    }

    user_service::delete_user(&state.db, user_id).await?;

    Ok(ApiResponse::success_with_message((), "用户已删除"))
}
