use crate::admin::products::model::AdminProductDummy;
use fake::{Fake, Faker};
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

pub async fn seed_admin_user(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, username, email, password)
        VALUES ($1, $2, $3, $4)
        "#,
        1,
        "admin",
        "admin@admin.com",
        "$argon2id$v=19$m=19456,t=2,p=1$cvMtKFBV7DMHHq7DLLCDAg$JbZ3kqb47wjU5IxeZmeea/6yYIC8Yz6Xqe1KwgWwroc"
    )
        .execute(pool)
        .await?;

    sqlx::query!(
        "INSERT INTO user_has_roles (user_id, role_id) VALUES (1, 1)
         ON CONFLICT DO NOTHING;"
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn seed_dummy_products(pool: &PgPool) -> Result<(), sqlx::Error> {
    let products: Vec<AdminProductDummy> = (0..100)
        .map(|_| Faker.fake::<AdminProductDummy>())
        .collect();

    for p in products {
        sqlx::query!(
            r#"
            INSERT INTO products (name, slug, price, quantity, configurable, is_active, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            p.name,
            p.slug,
            p.price,
            p.quantity,
            p.configurable,
            p.is_active,
            p.created_at
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn seed_all(pool: &PgPool) -> Result<(), sqlx::Error> {
    seed_roles(pool).await?;
    seed_admin_user(pool).await?;
    Ok(())
}
