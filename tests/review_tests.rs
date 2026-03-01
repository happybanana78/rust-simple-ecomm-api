use actix_test::ClientResponse;
use actix_web::http::StatusCode;
use ecomm::app::products::reviews::dto::{CreateProductReviewDto, PublicProductReview};
use ecomm::responses::api_responses::LocalApiResponse;
use ecomm::responses::error_responses::ErrorResponse;

mod utils;

async fn create_review_user(
    context: &utils::TestContext,
    payload: &CreateProductReviewDto,
    product_id: i64,
) -> ClientResponse {
    let auth_token = context.auth_token.clone().unwrap();

    context
        .srv
        .post(format!("/products/{}/reviews/create-user", product_id))
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send_json(&payload)
        .await
        .unwrap()
}

async fn create_review_guest(
    context: &utils::TestContext,
    payload: &CreateProductReviewDto,
    product_id: i64,
) -> ClientResponse {
    context
        .srv
        .post(format!("/products/{}/reviews/create-guest", product_id))
        .send_json(&payload)
        .await
        .unwrap()
}

#[actix_rt::test]
async fn test_review_create_with_user() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let payload = CreateProductReviewDto {
        product_id: Some(1),
        title: Some("new review title 1".to_string()),
        content: Some("new review content 1".to_string()),
        rating: Some(4),
    };

    let mut res = create_review_user(&context, &payload, 1).await;

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    let body: LocalApiResponse<PublicProductReview> = res.json().await.unwrap();

    assert_eq!(body.get_data().title, "new review title 1");

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_review_create_with_user_unexisting_user() {
    let context = utils::TestContext::new(Some("test23@test.com".to_string())).await;

    let payload = CreateProductReviewDto {
        product_id: Some(1),
        title: Some("new review title 1".to_string()),
        content: Some("new review content 1".to_string()),
        rating: Some(4),
    };

    let res = create_review_user(&context, &payload, 1).await;

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_review_create_with_user_no_token() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let payload = CreateProductReviewDto {
        product_id: Some(1),
        title: Some("new review title 1".to_string()),
        content: Some("new review content 1".to_string()),
        rating: Some(4),
    };

    let res = context
        .srv
        .post("/products/1/reviews/create-user")
        .send_json(&payload)
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_review_create_with_user_product_not_found() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let payload = CreateProductReviewDto {
        product_id: Some(100),
        title: Some("new review title 1".to_string()),
        content: Some("new review content 1".to_string()),
        rating: Some(4),
    };

    let res = create_review_user(&context, &payload, 1).await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_review_create_with_user_unprocessable_entity() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let payload = CreateProductReviewDto {
        product_id: Some(1),
        title: Some("new review title 1".to_string()),
        content: None,
        rating: None,
    };

    let res = create_review_user(&context, &payload, 1).await;

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_review_create_guest() {
    let context = utils::TestContext::new(None).await;

    let payload = CreateProductReviewDto {
        product_id: Some(1),
        title: Some("new review title 1".to_string()),
        content: Some("new review content 1".to_string()),
        rating: Some(4),
    };

    let mut res = create_review_guest(&context, &payload, 1).await;

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    let body: LocalApiResponse<PublicProductReview> = res.json().await.unwrap();

    assert_eq!(body.get_data().title, "new review title 1");

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_review_create_guest_product_not_found() {
    let context = utils::TestContext::new(None).await;

    let payload = CreateProductReviewDto {
        product_id: Some(100),
        title: Some("new review title 1".to_string()),
        content: Some("new review content 1".to_string()),
        rating: Some(4),
    };

    let res = create_review_guest(&context, &payload, 1).await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_review_create_guest_unprocessable_entity() {
    let context = utils::TestContext::new(None).await;

    let payload = CreateProductReviewDto {
        product_id: Some(1),
        title: Some("new review title 1".to_string()),
        content: None,
        rating: None,
    };

    let res = create_review_guest(&context, &payload, 1).await;

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);

    context.database.cleanup().await;
}
