use sea_orm::DatabaseConnection;
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;
use crate::entities::user::ActiveModel;
use crate::errors::{AppError, Result};
use crate::jwt::generate_token;
use crate::models::{CreateUserRequest, LoginRequest, LoginResponse, UserResponse};
use crate::repositories::user_repository;
use crate::config::JwtConfig;

/// 用户注册
pub async fn register(
    db: &DatabaseConnection,
    payload: CreateUserRequest,
    _jwt_config: &JwtConfig,
) -> Result<UserResponse> {
    // 检查用户名或邮箱是否已存在
    let exists = user_repository::exists_by_username_or_email(
        db,
        &payload.username,
        &payload.email,
    ).await?;
    
    if exists {
        return Err(AppError::Validation("用户名或邮箱已存在".to_string()));
    }
    
    // 加密密码
    let password_hash = hash(&payload.password, DEFAULT_COST)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("密码加密失败: {}", e)))?;
    
    let user_id = Uuid::new_v4();
    let now = chrono::Utc::now();
    
    // 创建用户
    let user = ActiveModel {
        id: sea_orm::Set(user_id),
        username: sea_orm::Set(payload.username),
        email: sea_orm::Set(payload.email),
        password_hash: sea_orm::Set(password_hash),
        created_at: sea_orm::Set(now),
        updated_at: sea_orm::Set(now),
    };
    
    let created_user = user_repository::create(db, user).await?;
    
    Ok(UserResponse::from(created_user))
}

/// 用户登录
pub async fn login(
    db: &DatabaseConnection,
    payload: LoginRequest,
    jwt_config: &JwtConfig,
) -> Result<LoginResponse> {
    // 查找用户
    let user = user_repository::find_by_username(db, &payload.username).await?
        .ok_or(AppError::Unauthorized)?;
    
    // 验证密码
    let valid = verify(&payload.password, &user.password_hash)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("密码验证失败: {}", e)))?;
    
    if !valid {
        return Err(AppError::Unauthorized);
    }
    
    // 生成 JWT token
    let token = generate_token(user.id, user.username.clone(), jwt_config)?;
    
    Ok(LoginResponse {
        token,
        user: UserResponse::from(user),
    })
}

/// 获取当前用户信息
pub async fn get_current_user(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<UserResponse> {
    let user = user_repository::find_by_id(db, user_id).await?
        .ok_or(AppError::NotFound)?;
    
    Ok(UserResponse::from(user))
}

