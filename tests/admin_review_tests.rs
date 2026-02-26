use actix_test::ClientResponse;
use actix_web::http::StatusCode;
use ecomm::admin::reviews::dto::{
    AdminPublicReview, IndexReviewDTO, ReviewApprovalStatus, UpdateReviewStatusDTO,
};
use ecomm::responses::api_responses::{LocalApiPaginatedResponse, LocalApiResponse};
use ecomm::responses::error_responses::ErrorResponse;

mod utils;

fn get_index_url(payload: IndexReviewDTO) -> String {
    format!(
        "/admin/reviews/list?{}",
        serde_urlencoded::to_string(payload).unwrap()
    )
}

async fn get_review(context: &utils::TestContext, review_id: i64) -> ClientResponse {
    let auth_token = context.auth_token.clone().unwrap();

    context
        .srv
        .get(format!("/admin/reviews/{}/get", review_id))
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send()
        .await
        .unwrap()
}

async fn update_review_status(
    context: &utils::TestContext,
    payload: &UpdateReviewStatusDTO,
    review_id: i64,
) -> ClientResponse {
    let auth_token = context.auth_token.clone().unwrap();

    context
        .srv
        .put(format!("/admin/reviews/{}/update-status", review_id))
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send_json(&payload)
        .await
        .unwrap()
}

async fn delete_review(context: &utils::TestContext, review_id: i64) -> ClientResponse {
    let auth_token = context.auth_token.clone().unwrap();

    context
        .srv
        .delete(format!("/admin/reviews/{}/delete", review_id))
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send()
        .await
        .unwrap()
}

#[actix_rt::test]
async fn test_admin_review_index() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let query_payload = IndexReviewDTO {
        page: Some(1),
        limit: Some(10),
        search: None,
        user_id: None,
        product_id: None,
        rating: None,
        status: None,
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

    let body: LocalApiPaginatedResponse<Vec<AdminPublicReview>> = res.json().await.unwrap();

    assert_eq!(body.get_data().len(), 10);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_review_index_incomplete_query_params() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let query_payload = IndexReviewDTO {
        page: None,
        limit: Some(10),
        search: None,
        user_id: None,
        product_id: None,
        rating: None,
        status: None,
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
async fn test_admin_review_index_search() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let query_payload_title = IndexReviewDTO {
        page: Some(1),
        limit: Some(10),
        search: Some("product 1 review title 1".to_string()),
        user_id: None,
        product_id: None,
        rating: None,
        status: None,
    };

    let query_payload_content = IndexReviewDTO {
        page: Some(1),
        limit: Some(10),
        search: Some("product 1 review content 1".to_string()),
        user_id: None,
        product_id: None,
        rating: None,
        status: None,
    };

    // try search by title
    let url = get_index_url(query_payload_title);

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

    let body: LocalApiPaginatedResponse<Vec<AdminPublicReview>> = res.json().await.unwrap();

    assert_eq!(body.get_data().len(), 1);
    assert_eq!(body.get_data()[0].title, "product 1 review title 1");

    // try search by content
    let url = get_index_url(query_payload_content);

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

    let body: LocalApiPaginatedResponse<Vec<AdminPublicReview>> = res.json().await.unwrap();

    assert_eq!(body.get_data().len(), 1);
    assert_eq!(body.get_data()[0].content, "product 1 review content 1");

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_review_index_simple_filter() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let query_payload = IndexReviewDTO {
        page: Some(1),
        limit: Some(10),
        search: None,
        user_id: Some(2),
        product_id: None,
        rating: None,
        status: None,
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

    let body: LocalApiPaginatedResponse<Vec<AdminPublicReview>> = res.json().await.unwrap();

    assert_eq!(body.get_data().len(), 3);
    assert_eq!(body.get_data()[0].id, 2);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_review_index_wrong_token() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let query_payload = IndexReviewDTO {
        page: Some(1),
        limit: Some(10),
        search: None,
        user_id: None,
        product_id: None,
        rating: None,
        status: None,
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
async fn test_admin_review_index_no_token() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let query_payload = IndexReviewDTO {
        page: Some(1),
        limit: Some(10),
        search: None,
        user_id: None,
        product_id: None,
        rating: None,
        status: None,
    };

    let url = get_index_url(query_payload);

    let res = context.srv.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_review_show() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let mut res = get_review(&context, 1).await;

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_review_show_review_not_found() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let res = get_review(&context, 70).await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_review_show_unauthorized_user() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let res = get_review(&context, 1).await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_review_show_no_token() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let res = context
        .srv
        .get("/admin/reviews/1/get")
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_review_update() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let mut get_review_res = get_review(&context, 1).await;

    let body: LocalApiResponse<AdminPublicReview> = get_review_res.json().await.unwrap();

    assert_eq!(
        body.get_data().approval_status,
        ReviewApprovalStatus::Pending
    );

    let payload = UpdateReviewStatusDTO {
        status: Some("approved".to_string()),
    };

    let mut res = update_review_status(&context, &payload, 1).await;

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    let mut get_review_res = get_review(&context, 1).await;

    let body: LocalApiResponse<AdminPublicReview> = get_review_res.json().await.unwrap();

    assert_eq!(
        body.get_data().approval_status,
        ReviewApprovalStatus::Approved
    );

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_review_update_unauthorized_user() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let payload = UpdateReviewStatusDTO {
        status: Some("approved".to_string()),
    };

    let res = update_review_status(&context, &payload, 1).await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_review_update_unprocessable_entity() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let payload = UpdateReviewStatusDTO { status: None };

    let res = update_review_status(&context, &payload, 1).await;

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_review_update_status_does_not_exist() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let payload = UpdateReviewStatusDTO {
        status: Some("some_status".to_string()),
    };

    let res = update_review_status(&context, &payload, 1).await;

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_review_update_review_not_found() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let payload = UpdateReviewStatusDTO {
        status: Some("approved".to_string()),
    };

    let res = update_review_status(&context, &payload, 270).await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_review_delete() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let mut res = delete_review(&context, 1).await;

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_review_delete_unauthorized_user() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let res = delete_review(&context, 1).await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_review_delete_review_not_found() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let res = delete_review(&context, 180).await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    context.database.cleanup().await;
}
