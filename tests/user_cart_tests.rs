mod utils;

use actix_test::{ClientResponse, TestServer};
use actix_web::http::StatusCode;
use ecomm::app::cart::cart_items::dto::{AddItemDto, RemoveItemDto, UpdateItemDto};
use ecomm::app::cart::user_cart::dto::PublicUserCart;
use ecomm::responses::api_responses::LocalApiResponse;
use uuid::Uuid;

#[actix_rt::test]
async fn test_get_empty_user_cart() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let cart = get_user_cart(&context.srv, &auth_token).await;

    assert!(cart.get_data().items.is_empty());

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_get_user_cart_with_wrong_auth_token() {
    let context = utils::TestContext::new(None).await;

    let auth_token = Uuid::new_v4().to_string();

    let res = context
        .srv
        .get("/cart/user/get")
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED, "{:#?}", res);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_get_user_cart_after_adding_an_item() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let payload = AddItemDto {
        product_id: Some(1),
        price: Some(10.99),
        quantity: Some(1),
    };

    let res = add_item_to_user_cart(&context.srv, &auth_token, payload).await;
    assert!(res.status().is_success(), "{:#?}", res);

    let cart = get_user_cart(&context.srv, &auth_token).await;

    assert_eq!(cart.get_data().items.len(), 1);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_add_to_cart_non_existing_product() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let payload = AddItemDto {
        product_id: Some(100),
        price: Some(10.99),
        quantity: Some(1),
    };

    let res = add_item_to_user_cart(&context.srv, &auth_token, payload).await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND, "{:#?}", res);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_add_to_cart_with_invalid_quantity() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let payload = AddItemDto {
        product_id: Some(1),
        price: Some(10.99),
        quantity: Some(-1),
    };

    let res = add_item_to_user_cart(&context.srv, &auth_token, payload).await;

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY, "{:#?}", res);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_update_item_on_user_cart() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let payload = AddItemDto {
        product_id: Some(1),
        price: Some(10.99),
        quantity: Some(1),
    };

    let res = add_item_to_user_cart(&context.srv, &auth_token, payload).await;
    assert!(res.status().is_success(), "{:#?}", res);

    let payload = UpdateItemDto {
        product_id: Some(1),
        quantity: Some(2),
    };

    let res = update_item_on_user_cart(&context.srv, &auth_token, payload).await;
    assert!(res.status().is_success(), "{:#?}", res);

    let cart = get_user_cart(&context.srv, &auth_token).await;

    assert_eq!(cart.get_data().items.len(), 1);
    assert_eq!(cart.get_data().items[0].quantity, 2);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_update_item_on_user_cart_with_product_not_existing_in_cart() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let payload = UpdateItemDto {
        product_id: Some(1),
        quantity: Some(2),
    };

    let res = update_item_on_user_cart(&context.srv, &auth_token, payload).await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND, "{:#?}", res);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_update_item_on_user_cart_with_invalid_quantity() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let payload = AddItemDto {
        product_id: Some(1),
        price: Some(10.99),
        quantity: Some(1),
    };

    let res = add_item_to_user_cart(&context.srv, &auth_token, payload).await;
    assert!(res.status().is_success(), "{:#?}", res);

    let payload = UpdateItemDto {
        product_id: Some(1),
        quantity: Some(-2),
    };

    let res = update_item_on_user_cart(&context.srv, &auth_token, payload).await;
    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY, "{:#?}", res);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_remove_item_from_user_cart() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let payload = AddItemDto {
        product_id: Some(1),
        price: Some(10.99),
        quantity: Some(1),
    };

    let res = add_item_to_user_cart(&context.srv, &auth_token, payload).await;
    assert!(res.status().is_success(), "{:#?}", res);

    let payload = RemoveItemDto {
        product_id: Some(1),
    };

    let res = remove_item_from_user_cart(&context.srv, &auth_token, payload).await;
    assert!(res.status().is_success(), "{:#?}", res);

    let cart = get_user_cart(&context.srv, &auth_token).await;

    assert!(cart.get_data().items.is_empty());

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_remove_non_existing_item_from_user_cart() {
    let context = utils::TestContext::new(Some("test1@test.com".to_string())).await;

    let auth_token = context.auth_token.unwrap();

    let payload = RemoveItemDto {
        product_id: Some(1),
    };

    let res = remove_item_from_user_cart(&context.srv, &auth_token, payload).await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND, "{:#?}", res);

    context.database.cleanup().await;
}

async fn get_user_cart(srv: &TestServer, auth_token: &str) -> LocalApiResponse<PublicUserCart> {
    let mut res = srv
        .get("/cart/user/get")
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send()
        .await
        .unwrap();

    assert!(res.status().is_success(), "{:#?}", res);

    let body: LocalApiResponse<PublicUserCart> = res.json().await.unwrap();

    body
}

async fn add_item_to_user_cart(
    srv: &TestServer,
    auth_token: &str,
    payload: AddItemDto,
) -> ClientResponse {
    srv.post("/cart/user/add")
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send_json(&payload)
        .await
        .unwrap()
}

async fn update_item_on_user_cart(
    srv: &TestServer,
    auth_token: &str,
    payload: UpdateItemDto,
) -> ClientResponse {
    srv.put("/cart/user/update")
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send_json(&payload)
        .await
        .unwrap()
}

async fn remove_item_from_user_cart(
    srv: &TestServer,
    auth_token: &str,
    payload: RemoveItemDto,
) -> ClientResponse {
    srv.delete("/cart/user/remove")
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send_json(&payload)
        .await
        .unwrap()
}
