use ecomm::products::repository::ProductRepository;
use sqlx::PgPool;

#[sqlx::test]
async fn test_get_product(pool: PgPool) {
    let repo = ProductRepository::new(pool);

    let product = repo.show(&1).await.unwrap();

    assert!(product.is_some());
}
