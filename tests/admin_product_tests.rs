use actix_test::ClientResponse;
use actix_web::http::StatusCode;
use ecomm::admin::products::dto::{
    AdminPublicProduct, CreateProductDTO, IndexProductDTO, UpdateProductDTO,
};
use ecomm::responses::api_responses::{LocalApiPaginatedResponse, LocalApiResponse};
use ecomm::responses::error_responses::ErrorResponse;

mod utils;

fn get_index_url(payload: IndexProductDTO) -> String {
    format!(
        "/admin/products/list?{}",
        serde_urlencoded::to_string(payload).unwrap()
    )
}

async fn get_product(context: &utils::TestContext, product_id: i64) -> ClientResponse {
    let auth_token = context.auth_token.clone().unwrap();

    context
        .srv
        .get(format!("/admin/products/get/{}", product_id))
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send()
        .await
        .unwrap()
}

async fn create_product(
    context: &utils::TestContext,
    payload: &CreateProductDTO,
) -> ClientResponse {
    let auth_token = context.auth_token.clone().unwrap();

    context
        .srv
        .post("/admin/products/create")
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send_json(&payload)
        .await
        .unwrap()
}

async fn update_product(
    context: &utils::TestContext,
    payload: &UpdateProductDTO,
    product_id: i64,
) -> ClientResponse {
    let auth_token = context.auth_token.clone().unwrap();

    context
        .srv
        .put(format!("/admin/products/update/{}", product_id))
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send_json(&payload)
        .await
        .unwrap()
}

async fn delete_product(context: &utils::TestContext, product_id: i64) -> ClientResponse {
    let auth_token = context.auth_token.clone().unwrap();

    context
        .srv
        .delete(format!("/admin/products/delete/{}", product_id))
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send()
        .await
        .unwrap()
}

#[actix_rt::test]
async fn test_admin_product_index() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let query_payload = IndexProductDTO {
        page: Some(1),
        limit: Some(10),
        search: None,
        price_min: None,
        price_max: None,
        in_stock: None,
        is_active: None,
    };

    let url = get_index_url(query_payload);

    let mut res = context
        .srv
        .get(url)
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send()
        .await
        .unwrap();

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_index_incomplete_query_params() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let query_payload = IndexProductDTO {
        page: None,
        limit: Some(10),
        search: None,
        price_min: None,
        price_max: None,
        in_stock: None,
        is_active: None,
    };

    let url = get_index_url(query_payload);

    let res = context
        .srv
        .get(url)
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_index_search() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let query_payload = IndexProductDTO {
        page: Some(1),
        limit: Some(10),
        search: Some("Test Product 1".to_string()),
        price_min: None,
        price_max: None,
        in_stock: None,
        is_active: None,
    };

    let url = get_index_url(query_payload);

    let mut res = context
        .srv
        .get(url)
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send()
        .await
        .unwrap();

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    let body: LocalApiPaginatedResponse<Vec<AdminPublicProduct>> = res.json().await.unwrap();

    assert_eq!(body.get_data().len(), 1);
    assert_eq!(body.get_data()[0].name, "Test Product 1");

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_index_simple_filter() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let query_payload = IndexProductDTO {
        page: Some(1),
        limit: Some(10),
        search: None,
        price_min: None,
        price_max: None,
        in_stock: None,
        is_active: Some(true),
    };

    let url = get_index_url(query_payload);

    let mut res = context
        .srv
        .get(url)
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send()
        .await
        .unwrap();

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    let body: LocalApiPaginatedResponse<Vec<AdminPublicProduct>> = res.json().await.unwrap();

    assert_eq!(body.get_data().len(), 1);
    assert_eq!(body.get_data()[0].id, 1);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_index_wrong_token() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let query_payload = IndexProductDTO {
        page: Some(1),
        limit: Some(10),
        search: None,
        price_min: None,
        price_max: None,
        in_stock: None,
        is_active: None,
    };

    let url = get_index_url(query_payload);

    let res = context
        .srv
        .get(url)
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_index_no_token() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let query_payload = IndexProductDTO {
        page: Some(1),
        limit: Some(10),
        search: None,
        price_min: None,
        price_max: None,
        in_stock: None,
        is_active: None,
    };

    let url = get_index_url(query_payload);

    let res = context.srv.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_show() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let mut res = get_product(&context, 1).await;

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_show_product_not_found() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let res = get_product(&context, 70).await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_show_unauthorized_user() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let res = get_product(&context, 1).await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_show_no_token() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let res = context
        .srv
        .get("/admin/products/get/1")
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_create() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let payload = CreateProductDTO {
        name: Some("Test Product New 1".to_string()),
        categories: Some(vec![1, 2]),
        price: Some(100.0),
        quantity: Some(10),
        configurable: Some(false),
        is_active: Some(true),
    };

    let mut res = create_product(&context, &payload).await;

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    let body: LocalApiResponse<AdminPublicProduct> = res.json().await.unwrap();

    assert_eq!(body.get_data().name, "Test Product New 1");

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_create_unauthorized_user() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let payload = CreateProductDTO {
        name: Some("Test Product New 1".to_string()),
        categories: Some(vec![1, 2]),
        price: Some(100.0),
        quantity: Some(10),
        configurable: Some(false),
        is_active: Some(true),
    };

    let res = create_product(&context, &payload).await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_create_no_token() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let payload = CreateProductDTO {
        name: Some("Test Product New 1".to_string()),
        categories: Some(vec![1, 2]),
        price: Some(100.0),
        quantity: Some(10),
        configurable: Some(false),
        is_active: Some(true),
    };

    let res = context
        .srv
        .post("/admin/products/create")
        .send_json(&payload)
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_create_unprocessable_entity() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let payload = CreateProductDTO {
        name: None,
        categories: None,
        price: None,
        quantity: None,
        configurable: None,
        is_active: None,
    };

    let res = create_product(&context, &payload).await;

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_create_product_already_exists() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let payload = CreateProductDTO {
        name: Some("Test Product New 1".to_string()),
        categories: Some(vec![1]),
        price: Some(100.0),
        quantity: Some(10),
        configurable: Some(false),
        is_active: Some(true),
    };

    create_product(&context, &payload).await;

    let res = create_product(&context, &payload).await;

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_create_category_not_found() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let payload = CreateProductDTO {
        name: Some("Test Product New 1".to_string()),
        categories: Some(vec![1, 24]),
        price: Some(100.0),
        quantity: Some(10),
        configurable: Some(false),
        is_active: Some(true),
    };

    let res = create_product(&context, &payload).await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_update() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let payload = UpdateProductDTO {
        name: Some("Test Product Edited 1".to_string()),
        categories: Some(vec![1, 2]),
        price: Some(100.0),
        quantity: Some(10),
        configurable: Some(false),
        is_active: Some(true),
    };

    let mut res = update_product(&context, &payload, 1).await;

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    let mut get_product_res = get_product(&context, 1).await;

    let body: LocalApiResponse<AdminPublicProduct> = get_product_res.json().await.unwrap();

    assert_eq!(body.get_data().name, "Test Product Edited 1");

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_update_unauthorized_user() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let payload = UpdateProductDTO {
        name: Some("Test Product Edited 1".to_string()),
        categories: Some(vec![1, 2]),
        price: Some(100.0),
        quantity: Some(10),
        configurable: Some(false),
        is_active: Some(true),
    };

    let res = update_product(&context, &payload, 1).await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_update_unprocessable_entity() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let payload = UpdateProductDTO {
        name: None,
        categories: None,
        price: None,
        quantity: None,
        configurable: None,
        is_active: None,
    };

    let res = update_product(&context, &payload, 1).await;

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_update_product_already_exists() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let payload = UpdateProductDTO {
        name: Some("Test Product Edited 1".to_string()),
        categories: Some(vec![1, 2]),
        price: Some(100.0),
        quantity: Some(10),
        configurable: Some(false),
        is_active: Some(true),
    };

    update_product(&context, &payload, 1).await;

    let res = update_product(&context, &payload, 1).await;

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_update_category_not_found() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let payload = UpdateProductDTO {
        name: Some("Test Product Edited 1".to_string()),
        categories: Some(vec![1, 20]),
        price: Some(100.0),
        quantity: Some(10),
        configurable: Some(false),
        is_active: Some(true),
    };

    let res = update_product(&context, &payload, 1).await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_update_product_not_found() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let payload = UpdateProductDTO {
        name: Some("Test Product Edited 1".to_string()),
        categories: Some(vec![1, 2]),
        price: Some(100.0),
        quantity: Some(10),
        configurable: Some(false),
        is_active: Some(true),
    };

    let res = update_product(&context, &payload, 270).await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_delete() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let mut res = delete_product(&context, 1).await;

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_delete_unauthorized_user() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let res = delete_product(&context, 1).await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_delete_product_not_found() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let res = delete_product(&context, 180).await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    context.database.cleanup().await;
}
