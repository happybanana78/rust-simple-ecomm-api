use actix_test::ClientResponse;
use actix_web::http::StatusCode;
use ecomm::admin::categories::dto::{
    AdminPublicCategory, CreateCategoryDTO, IndexCategoryDTO, UpdateCategoryDTO,
};
use ecomm::responses::api_responses::{LocalApiPaginatedResponse, LocalApiResponse};
use ecomm::responses::error_responses::ErrorResponse;

mod utils;

fn get_index_url(payload: IndexCategoryDTO) -> String {
    format!(
        "/admin/categories/list?{}",
        serde_urlencoded::to_string(payload).unwrap()
    )
}

async fn get_category(context: &utils::TestContext, category_id: i64) -> ClientResponse {
    let auth_token = context.auth_token.clone().unwrap();

    context
        .srv
        .get(format!("/admin/categories/get/{}", category_id))
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send()
        .await
        .unwrap()
}

async fn create_category(
    context: &utils::TestContext,
    payload: &CreateCategoryDTO,
) -> ClientResponse {
    let auth_token = context.auth_token.clone().unwrap();

    context
        .srv
        .post("/admin/categories/create")
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send_json(&payload)
        .await
        .unwrap()
}

async fn update_category(
    context: &utils::TestContext,
    payload: &UpdateCategoryDTO,
    category_id: i64,
) -> ClientResponse {
    let auth_token = context.auth_token.clone().unwrap();

    context
        .srv
        .put(format!("/admin/categories/update/{}", category_id))
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send_json(&payload)
        .await
        .unwrap()
}

async fn delete_category(context: &utils::TestContext, category_id: i64) -> ClientResponse {
    let auth_token = context.auth_token.clone().unwrap();

    context
        .srv
        .delete(format!("/admin/categories/delete/{}", category_id))
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send()
        .await
        .unwrap()
}

#[actix_rt::test]
async fn test_admin_category_index() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let query_payload = IndexCategoryDTO {
        page: Some(1),
        limit: Some(10),
        search: None,
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
async fn test_admin_category_index_incomplete_query_params() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let query_payload = IndexCategoryDTO {
        page: None,
        limit: Some(10),
        search: None,
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
async fn test_admin_category_index_search() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let query_payload = IndexCategoryDTO {
        page: Some(1),
        limit: Some(10),
        search: Some("Test Category 1".to_string()),
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

    let body: LocalApiPaginatedResponse<Vec<AdminPublicCategory>> = res.json().await.unwrap();

    assert_eq!(body.get_data().len(), 1);
    assert_eq!(body.get_data()[0].name, "Test Category 1");

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_category_index_simple_filter() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let query_payload = IndexCategoryDTO {
        page: Some(1),
        limit: Some(10),
        search: None,
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

    let body: LocalApiPaginatedResponse<Vec<AdminPublicCategory>> = res.json().await.unwrap();

    assert_eq!(body.get_data().len(), 1);
    assert_eq!(body.get_data()[0].id, 1);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_category_index_wrong_token() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let query_payload = IndexCategoryDTO {
        page: Some(1),
        limit: Some(10),
        search: None,
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
async fn test_admin_category_index_no_token() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let query_payload = IndexCategoryDTO {
        page: Some(1),
        limit: Some(10),
        search: None,
        is_active: None,
    };

    let url = get_index_url(query_payload);

    let res = context.srv.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_category_show() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let mut res = get_category(&context, 1).await;

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_category_show_category_not_found() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let res = get_category(&context, 70).await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_category_show_unauthorized_user() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let res = get_category(&context, 1).await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_category_show_no_token() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let res = context
        .srv
        .get("/admin/categories/get/1")
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_category_create() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let payload = CreateCategoryDTO {
        name: Some("Test Category New 1".to_string()),
        slug: Some("test-category-new-1".to_string()),
        is_active: Some(true),
    };

    let mut res = create_category(&context, &payload).await;

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    let body: LocalApiResponse<AdminPublicCategory> = res.json().await.unwrap();

    assert_eq!(body.get_data().name, "Test Category New 1");

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_category_create_unauthorized_user() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let payload = CreateCategoryDTO {
        name: Some("Test Category New 1".to_string()),
        slug: Some("test-category-new-1".to_string()),
        is_active: Some(true),
    };

    let res = create_category(&context, &payload).await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_category_create_no_token() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let payload = CreateCategoryDTO {
        name: Some("Test Category New 1".to_string()),
        slug: Some("test-category-new-1".to_string()),
        is_active: Some(true),
    };

    let res = context
        .srv
        .post("/admin/categories/create")
        .send_json(&payload)
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_category_create_unprocessable_entity() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let payload = CreateCategoryDTO {
        name: None,
        slug: None,
        is_active: None,
    };

    let res = create_category(&context, &payload).await;

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_category_create_wrong_slug() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let payload = CreateCategoryDTO {
        name: Some("Test Category New 1".to_string()),
        slug: Some("test category 1".to_string()),
        is_active: Some(true),
    };

    let res = create_category(&context, &payload).await;

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_category_create_category_already_exists() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let payload = CreateCategoryDTO {
        name: Some("Test Category New 1".to_string()),
        slug: Some("test-category-new-1".to_string()),
        is_active: Some(true),
    };

    create_category(&context, &payload).await;

    let res = create_category(&context, &payload).await;

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_category_update() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let payload = UpdateCategoryDTO {
        name: Some("Test Category Edited 1".to_string()),
        slug: Some("test-category-edited-1".to_string()),
        is_active: Some(true),
    };

    let mut res = update_category(&context, &payload, 1).await;

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    let mut get_category_res = get_category(&context, 1).await;

    let body: LocalApiResponse<AdminPublicCategory> = get_category_res.json().await.unwrap();

    assert_eq!(body.get_data().name, "Test Category Edited 1");

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_category_update_unauthorized_user() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let payload = UpdateCategoryDTO {
        name: Some("Test Category Edited 1".to_string()),
        slug: Some("test-category-edited-1".to_string()),
        is_active: Some(true),
    };

    let res = update_category(&context, &payload, 1).await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_category_update_unprocessable_entity() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let payload = UpdateCategoryDTO {
        name: None,
        slug: None,
        is_active: None,
    };

    let res = update_category(&context, &payload, 1).await;

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_category_update_wrong_slug() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let payload = UpdateCategoryDTO {
        name: Some("Test Category Edited 1".to_string()),
        slug: Some("test category-1".to_string()),
        is_active: Some(true),
    };

    let res = update_category(&context, &payload, 1).await;

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_category_update_category_already_exists() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let payload = UpdateCategoryDTO {
        name: Some("Test Category Edited 1".to_string()),
        slug: Some("test-category-edited-1".to_string()),
        is_active: Some(true),
    };

    update_category(&context, &payload, 1).await;

    let res = update_category(&context, &payload, 1).await;

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_category_update_category_not_found() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let payload = UpdateCategoryDTO {
        name: Some("Test Category Edited 1".to_string()),
        slug: Some("test-category-edited-1".to_string()),
        is_active: Some(true),
    };

    let res = update_category(&context, &payload, 270).await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_category_delete() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let mut res = delete_category(&context, 1).await;

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_category_delete_unauthorized_user() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let res = delete_category(&context, 1).await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_category_delete_category_not_found() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let res = delete_category(&context, 180).await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    context.database.cleanup().await;
}
