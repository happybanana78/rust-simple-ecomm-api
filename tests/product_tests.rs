use actix_web::http::StatusCode;
use ecomm::products::dto::IndexProductDTO;
use ecomm::products::dto::PublicProduct;
use ecomm::responses::api_responses::LocalApiPaginatedResponse;
use ecomm::responses::error_responses::ErrorResponse;

mod utils;

fn get_index_url(payload: IndexProductDTO) -> String {
    format!(
        "/products/list?{}",
        serde_urlencoded::to_string(payload).unwrap()
    )
}

#[actix_rt::test]
async fn test_product_index() {
    let context = utils::TestContext::new(None).await;

    let query_payload = IndexProductDTO {
        page: Some(1),
        limit: Some(10),
        search: None,
        category: None,
        price_min: None,
        price_max: None,
    };

    let url = get_index_url(query_payload);

    let mut res = context.srv.get(url).send().await.unwrap();

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    let body: LocalApiPaginatedResponse<Vec<PublicProduct>> = res.json().await.unwrap();

    assert_eq!(body.get_data().len(), 2);
    assert_eq!(body.get_data()[0].images[0].alt, "p1 example image 1");

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_product_index_incomplete_query_params() {
    let context = utils::TestContext::new(None).await;

    let query_payload = IndexProductDTO {
        page: None,
        limit: Some(10),
        search: None,
        category: None,
        price_min: None,
        price_max: None,
    };

    let url = get_index_url(query_payload);

    let res = context.srv.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_product_index_search() {
    let context = utils::TestContext::new(None).await;

    let query_payload = IndexProductDTO {
        page: Some(1),
        limit: Some(10),
        search: Some("Test Product 1".to_string()),
        category: None,
        price_min: None,
        price_max: None,
    };

    let url = get_index_url(query_payload);

    let mut res = context.srv.get(url).send().await.unwrap();

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    let body: LocalApiPaginatedResponse<Vec<PublicProduct>> = res.json().await.unwrap();

    assert_eq!(body.get_data().len(), 1);
    assert_eq!(body.get_data()[0].name, "Test Product 1");

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_product_show() {
    let context = utils::TestContext::new(None).await;

    let res = context
        .srv
        .get("/products/get/test-product-1")
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());

    context.database.cleanup().await;
}
