use actix_web::http::StatusCode;
use ecomm::app::categories::dto::{IndexCategoryDTO, PublicCategory};
use ecomm::responses::api_responses::LocalApiPaginatedResponse;
use ecomm::responses::error_responses::ErrorResponse;

mod utils;

fn get_index_url(payload: IndexCategoryDTO) -> String {
    format!(
        "/categories/list?{}",
        serde_urlencoded::to_string(payload).unwrap()
    )
}

#[actix_rt::test]
async fn test_category_index() {
    let context = utils::TestContext::new(None).await;

    let query_payload = IndexCategoryDTO {
        page: Some(1),
        limit: Some(10),
        search: None,
    };

    let url = get_index_url(query_payload);

    let mut res = context.srv.get(url).send().await.unwrap();

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    let body: LocalApiPaginatedResponse<Vec<PublicCategory>> = res.json().await.unwrap();

    assert_eq!(body.get_data().len(), 1); // only the active one

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_category_index_incomplete_query_params() {
    let context = utils::TestContext::new(None).await;

    let query_payload = IndexCategoryDTO {
        page: None,
        limit: Some(10),
        search: None,
    };

    let url = get_index_url(query_payload);

    let res = context.srv.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_category_index_search() {
    let context = utils::TestContext::new(None).await;

    let query_payload = IndexCategoryDTO {
        page: Some(1),
        limit: Some(10),
        search: Some("Test Category 1".to_string()),
    };

    let url = get_index_url(query_payload);

    let mut res = context.srv.get(url).send().await.unwrap();

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    let body: LocalApiPaginatedResponse<Vec<PublicCategory>> = res.json().await.unwrap();

    assert_eq!(body.get_data().len(), 1);
    assert_eq!(body.get_data()[0].name, "Test Category 1");

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_category_show() {
    let context = utils::TestContext::new(None).await;

    let res = context
        .srv
        .get("/categories/get/test-category-1")
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());

    context.database.cleanup().await;
}
