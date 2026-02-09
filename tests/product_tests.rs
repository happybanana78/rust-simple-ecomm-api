mod utils;

#[actix_rt::test]
async fn test_product_index() {
    let test_db = utils::TestDatabase::new().await;
    utils::seed_products(&test_db.pool).await;

    let srv = utils::create_test_server(test_db.pool.clone());

    let res = srv.get("/products/list").send().await.unwrap();
    assert!(res.status().is_success());

    test_db.cleanup().await;
}

#[actix_rt::test]
async fn test_product_show() {
    let test_db = utils::TestDatabase::new().await;
    utils::seed_products(&test_db.pool).await;

    let srv = utils::create_test_server(test_db.pool.clone());

    let res = srv.get("/products/get/1").send().await.unwrap();
    assert!(res.status().is_success());

    test_db.cleanup().await;
}
