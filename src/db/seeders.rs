use sqlx::PgPool;

pub async fn seed_roles(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO roles (name)
        VALUES ($1)
        "#,
        "admin"
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        r#"
        INSERT INTO roles (name)
        VALUES ($1)
        "#,
        "user"
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn seed_all(pool: &PgPool) -> Result<(), sqlx::Error> {
    seed_roles(pool).await?;
    Ok(())
}
