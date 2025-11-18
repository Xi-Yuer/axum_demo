use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait, QueryOrder, PaginatorTrait};
use uuid::Uuid;
use crate::entities::article::{Entity as Article, Model};
use crate::errors::{AppError, Result};

/// 根据 ID 查找文章
pub async fn find_by_id(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<Option<Model>> {
    Article::find_by_id(id)
        .one(db)
        .await
        .map_err(AppError::Database)
}

/// 创建文章
pub async fn create(
    db: &DatabaseConnection,
    article: crate::entities::article::ActiveModel,
) -> Result<Model> {
    article.insert(db)
        .await
        .map_err(AppError::Database)
}

/// 更新文章
pub async fn update(
    db: &DatabaseConnection,
    id: Uuid,
    mut article: crate::entities::article::ActiveModel,
) -> Result<Model> {
    article.id = Set(id);
    let updated = article.update(db)
        .await
        .map_err(AppError::Database)?;
    Ok(updated)
}

/// 删除文章
pub async fn delete(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<()> {
    let result = Article::delete_by_id(id)
        .exec(db)
        .await
        .map_err(AppError::Database)?;
    
    if result.rows_affected == 0 {
        return Err(AppError::NotFound);
    }
    
    Ok(())
}

/// 分页查询文章列表（根据用户ID和是否公开）
pub async fn find_all_with_pagination(
    db: &DatabaseConnection,
    user_id: Option<Uuid>,
    is_public_only: bool,
    offset: u64,
    limit: u64,
) -> Result<(Vec<Model>, u64)> {
    let mut query = Article::find();
    
    if is_public_only {
        // 只查询公开文章
        query = query.filter(crate::entities::article::Column::IsPublic.eq(true));
    } else if let Some(uid) = user_id {
        // 查询用户的所有文章或公开文章
        query = query.filter(
            sea_orm::Condition::any()
                .add(crate::entities::article::Column::UserId.eq(uid))
                .add(crate::entities::article::Column::IsPublic.eq(true))
        );
    }
    
    let paginator = query
        .order_by_desc(crate::entities::article::Column::CreatedAt)
        .paginate(db, limit);
    
    let total = paginator.num_items().await
        .map_err(AppError::Database)?;
    
    let page = if limit > 0 { offset / limit } else { 0 };
    let articles = paginator.fetch_page(page)
        .await
        .map_err(AppError::Database)?;
    
    Ok((articles, total))
}

