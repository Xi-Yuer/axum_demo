use axum::{extract::State, Json};
use crate::errors::Result;
use crate::extractors::AuthUser;
use crate::models::{CreateUserRequest, LoginRequest, LoginResponse, UserResponse};
use crate::response::ApiResponse;
use crate::services::auth_service;
use crate::AppState;

/// 用户注册
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<ApiResponse<UserResponse>>> {
    let user = auth_service::register(&state.db, payload, &state.config.jwt).await?;
    
    Ok(Json(ApiResponse::success_with_message(
        user,
        "注册成功"
    )))
}

/// 用户登录
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>> {
    let login_response = auth_service::login(&state.db, payload, &state.config.jwt).await?;
    
    Ok(Json(ApiResponse::success_with_message(
        login_response,
        "登录成功"
    )))
}

/// 获取当前用户信息
pub async fn me(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<ApiResponse<UserResponse>>> {
    let user = auth_service::get_current_user(&state.db, auth_user.user_id).await?;
    
    Ok(Json(ApiResponse::success(user)))
}

