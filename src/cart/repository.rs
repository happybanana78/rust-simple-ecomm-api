use sqlx::PgPool;
use crate::cart::model::{HashCartModel, UserCartModel};
use crate::errors::error::AppError;

pub async fn get_cart_by_user_id(pool: &PgPool, user_id: &i64) -> Result<Option<UserCartModel>, AppError> {
    sqlx::query_as!(
        UserCartModel,
        r#"
        SELECT
            id,
            user_id AS "user_id!",
            total,
            created_at
        FROM cart
        WHERE user_id = $1;
        "#,
        user_id
    )
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)
}

pub async fn get_cart_by_user_hash(pool: &PgPool, hash_id: &i64) -> Result<Option<HashCartModel>, AppError> {
    sqlx::query_as!(
        HashCartModel,
        r#"
        SELECT
            id,
            user_hash_id AS "user_hash_id!",
            total,
            created_at
        FROM cart
        WHERE user_hash_id = $1;
        "#,
        hash_id
    )
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)
}

pub async fn create_user_cart(pool: &PgPool, user_id: &i64) -> Result<UserCartModel, AppError> {
    sqlx::query_as!(
        UserCartModel,
        r#"
        INSERT INTO cart (user_id, total, created_at)
        VALUES ($1, 0, NOW())
        RETURNING id, user_id AS "user_id!", total, created_at;
        "#,
        user_id
    )
        .fetch_one(pool)
        .await
        .map_err(AppError::Database)
}

pub async fn create_hash_cart(pool: &PgPool, hash_id: &i64) -> Result<HashCartModel, AppError> {
    sqlx::query_as!(
        HashCartModel,
        r#"
        INSERT INTO cart (user_hash_id, total, created_at)
        VALUES ($1, 0, NOW())
        RETURNING id, user_hash_id AS "user_hash_id!", total, created_at;
        "#,
        hash_id
    )
        .fetch_one(pool)
        .await
        .map_err(AppError::Database)
}

pub async fn delete_by_user_id(pool: &PgPool, user_id: &i64) -> Result<u64, AppError> {
    let result = sqlx::query_as!(
        CartModel,
        "DELETE FROM cart WHERE user_id = $1;",
        user_id
    )
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

    Ok(result.rows_affected())
}

pub async fn delete_by_hash_id(pool: &PgPool, hash_id: &i64) -> Result<u64, AppError> {
    let result = sqlx::query_as!(
        CartModel,
        "DELETE FROM cart WHERE user_hash_id = $1;",
        hash_id
    )
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

    Ok(result.rows_affected())
}
