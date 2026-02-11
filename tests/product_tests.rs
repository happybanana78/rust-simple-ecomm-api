mod utils;

#[actix_rt::test]
async fn test_product_index() {
    let context = utils::TestContext::new(None).await;

    let res = context.srv.get("/products/list").send().await.unwrap();
    assert!(res.status().is_success());

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_product_show() {
    let context = utils::TestContext::new(None).await;

    let res = context.srv.get("/products/get/1").send().await.unwrap();
    assert!(res.status().is_success());

    context.database.cleanup().await;
}
