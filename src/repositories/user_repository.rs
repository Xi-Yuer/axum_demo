use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait, QueryOrder, PaginatorTrait};
use uuid::Uuid;
use crate::entities::user::{Entity as User, Model};
use crate::errors::{AppError, Result};

/// 根据 ID 查找用户
pub async fn find_by_id(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<Option<Model>> {
    User::find_by_id(id)
        .one(db)
        .await
        .map_err(AppError::Database)
}

/// 根据用户名查找用户
pub async fn find_by_username(
    db: &DatabaseConnection,
    username: &str,
) -> Result<Option<Model>> {
    User::find()
        .filter(crate::entities::user::Column::Username.eq(username))
        .one(db)
        .await
        .map_err(AppError::Database)
}

/// 根据邮箱查找用户
pub async fn find_by_email(
    db: &DatabaseConnection,
    email: &str,
) -> Result<Option<Model>> {
    User::find()
        .filter(crate::entities::user::Column::Email.eq(email))
        .one(db)
        .await
        .map_err(AppError::Database)
}

/// 检查用户名或邮箱是否已存在
pub async fn exists_by_username_or_email(
    db: &DatabaseConnection,
    username: &str,
    email: &str,
) -> Result<bool> {
    let count = User::find()
        .filter(
            sea_orm::Condition::any()
                .add(crate::entities::user::Column::Username.eq(username))
                .add(crate::entities::user::Column::Email.eq(email))
        )
        .count(db)
        .await
        .map_err(AppError::Database)?;
    
    Ok(count > 0)
}

/// 创建用户
pub async fn create(
    db: &DatabaseConnection,
    user: crate::entities::user::ActiveModel,
) -> Result<Model> {
    user.insert(db)
        .await
        .map_err(AppError::Database)
}

/// 更新用户
pub async fn update(
    db: &DatabaseConnection,
    id: Uuid,
    mut user: crate::entities::user::ActiveModel,
) -> Result<Model> {
    user.id = Set(id);
    let updated = user.update(db)
        .await
        .map_err(AppError::Database)?;
    Ok(updated)
}

/// 删除用户
pub async fn delete(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<()> {
    let result = User::delete_by_id(id)
        .exec(db)
        .await
        .map_err(AppError::Database)?;
    
    if result.rows_affected == 0 {
        return Err(AppError::NotFound);
    }
    
    Ok(())
}

/// 分页查询用户列表
pub async fn find_all_with_pagination(
    db: &DatabaseConnection,
    offset: u64,
    limit: u64,
) -> Result<(Vec<Model>, u64)> {
    let paginator = User::find()
        .order_by_desc(crate::entities::user::Column::CreatedAt)
        .paginate(db, limit);
    
    let total = paginator.num_items().await
        .map_err(AppError::Database)?;
    
    let page = if limit > 0 { offset / limit } else { 0 };
    let users = paginator.fetch_page(page)
        .await
        .map_err(AppError::Database)?;
    
    Ok((users, total))
}

