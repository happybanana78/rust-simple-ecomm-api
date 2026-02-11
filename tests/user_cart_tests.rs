mod utils;

use actix_test::{ClientResponse, TestServer};
use actix_web::http::StatusCode;
use ecomm::cart::cart_items::dto::{AddItemDto, RemoveItemDto, UpdateItemDto};
use ecomm::cart::user_cart::dto::PublicUserCart;
use ecomm::responses::api_responses::LocalApiResponse;
use uuid::Uuid;

#[actix_rt::test]
async fn test_get_empty_user_cart() {
    let test_db = utils::TestDatabase::new().await;

    utils::seed_roles(&test_db.pool).await;
    utils::seed_users(&test_db.pool).await;
    utils::seed_products(&test_db.pool).await;

    let srv = utils::create_test_server(test_db.pool.clone());

    let auth_token = utils::auto_login(&srv, "test1@test.com".to_string()).await;

    let cart = get_user_cart(&srv, &auth_token).await;

    assert!(cart.get_data().items.is_empty());

    test_db.cleanup().await;
}

#[actix_rt::test]
async fn test_get_user_cart_with_wrong_auth_token() {
    let test_db = utils::TestDatabase::new().await;

    utils::seed_roles(&test_db.pool).await;
    utils::seed_users(&test_db.pool).await;
    utils::seed_products(&test_db.pool).await;

    let srv = utils::create_test_server(test_db.pool.clone());

    let auth_token = Uuid::new_v4().to_string();

    let res = srv
        .get("/cart/user/get")
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED, "{:#?}", res);

    test_db.cleanup().await;
}

#[actix_rt::test]
async fn test_get_user_cart_after_adding_an_item() {
    let test_db = utils::TestDatabase::new().await;

    utils::seed_roles(&test_db.pool).await;
    utils::seed_users(&test_db.pool).await;
    utils::seed_products(&test_db.pool).await;

    let srv = utils::create_test_server(test_db.pool.clone());

    let auth_token = utils::auto_login(&srv, "test1@test.com".to_string()).await;

    let payload = AddItemDto {
        product_id: Some(1),
        price: Some(10.99),
        quantity: Some(1),
    };

    let res = add_item_to_user_cart(&srv, &auth_token, payload).await;
    assert!(res.status().is_success(), "{:#?}", res);

    let cart = get_user_cart(&srv, &auth_token).await;

    assert_eq!(cart.get_data().items.len(), 1);

    test_db.cleanup().await;
}

#[actix_rt::test]
async fn test_add_to_cart_non_existing_product() {
    let test_db = utils::TestDatabase::new().await;

    utils::seed_roles(&test_db.pool).await;
    utils::seed_users(&test_db.pool).await;

    let srv = utils::create_test_server(test_db.pool.clone());

    let auth_token = utils::auto_login(&srv, "test1@test.com".to_string()).await;

    let payload = AddItemDto {
        product_id: Some(1),
        price: Some(10.99),
        quantity: Some(1),
    };

    let res = add_item_to_user_cart(&srv, &auth_token, payload).await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND, "{:#?}", res);

    test_db.cleanup().await;
}

#[actix_rt::test]
async fn test_add_to_cart_with_invalid_quantity() {
    let test_db = utils::TestDatabase::new().await;

    utils::seed_roles(&test_db.pool).await;
    utils::seed_users(&test_db.pool).await;
    utils::seed_products(&test_db.pool).await;

    let srv = utils::create_test_server(test_db.pool.clone());

    let auth_token = utils::auto_login(&srv, "test1@test.com".to_string()).await;

    let payload = AddItemDto {
        product_id: Some(1),
        price: Some(10.99),
        quantity: Some(-1),
    };

    let res = add_item_to_user_cart(&srv, &auth_token, payload).await;

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY, "{:#?}", res);

    test_db.cleanup().await;
}

#[actix_rt::test]
async fn test_update_item_on_user_cart() {
    let test_db = utils::TestDatabase::new().await;

    utils::seed_roles(&test_db.pool).await;
    utils::seed_users(&test_db.pool).await;
    utils::seed_products(&test_db.pool).await;

    let srv = utils::create_test_server(test_db.pool.clone());

    let auth_token = utils::auto_login(&srv, "test1@test.com".to_string()).await;

    let payload = AddItemDto {
        product_id: Some(1),
        price: Some(10.99),
        quantity: Some(1),
    };

    let res = add_item_to_user_cart(&srv, &auth_token, payload).await;
    assert!(res.status().is_success(), "{:#?}", res);

    let payload = UpdateItemDto {
        product_id: Some(1),
        quantity: Some(2),
    };

    let res = update_item_on_user_cart(&srv, &auth_token, payload).await;
    assert!(res.status().is_success(), "{:#?}", res);

    let cart = get_user_cart(&srv, &auth_token).await;

    assert_eq!(cart.get_data().items.len(), 1);
    assert_eq!(cart.get_data().items[0].quantity, 2);

    test_db.cleanup().await;
}

#[actix_rt::test]
async fn test_update_item_on_user_cart_with_product_not_existing_in_cart() {
    let test_db = utils::TestDatabase::new().await;

    utils::seed_roles(&test_db.pool).await;
    utils::seed_users(&test_db.pool).await;
    utils::seed_products(&test_db.pool).await;

    let srv = utils::create_test_server(test_db.pool.clone());

    let auth_token = utils::auto_login(&srv, "test1@test.com".to_string()).await;

    let payload = UpdateItemDto {
        product_id: Some(1),
        quantity: Some(2),
    };

    let res = update_item_on_user_cart(&srv, &auth_token, payload).await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND, "{:#?}", res);

    test_db.cleanup().await;
}

#[actix_rt::test]
async fn test_update_item_on_user_cart_with_invalid_quantity() {
    let test_db = utils::TestDatabase::new().await;

    utils::seed_roles(&test_db.pool).await;
    utils::seed_users(&test_db.pool).await;
    utils::seed_products(&test_db.pool).await;

    let srv = utils::create_test_server(test_db.pool.clone());

    let auth_token = utils::auto_login(&srv, "test1@test.com".to_string()).await;

    let payload = AddItemDto {
        product_id: Some(1),
        price: Some(10.99),
        quantity: Some(1),
    };

    let res = add_item_to_user_cart(&srv, &auth_token, payload).await;
    assert!(res.status().is_success(), "{:#?}", res);

    let payload = UpdateItemDto {
        product_id: Some(1),
        quantity: Some(-2),
    };

    let res = update_item_on_user_cart(&srv, &auth_token, payload).await;
    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY, "{:#?}", res);

    test_db.cleanup().await;
}

#[actix_rt::test]
async fn test_remove_item_from_user_cart() {
    let test_db = utils::TestDatabase::new().await;

    utils::seed_roles(&test_db.pool).await;
    utils::seed_users(&test_db.pool).await;
    utils::seed_products(&test_db.pool).await;

    let srv = utils::create_test_server(test_db.pool.clone());

    let auth_token = utils::auto_login(&srv, "test1@test.com".to_string()).await;

    let payload = AddItemDto {
        product_id: Some(1),
        price: Some(10.99),
        quantity: Some(1),
    };

    let res = add_item_to_user_cart(&srv, &auth_token, payload).await;
    assert!(res.status().is_success(), "{:#?}", res);

    let payload = RemoveItemDto {
        product_id: Some(1),
    };

    let res = remove_item_from_user_cart(&srv, &auth_token, payload).await;
    assert!(res.status().is_success(), "{:#?}", res);

    let cart = get_user_cart(&srv, &auth_token).await;

    assert!(cart.get_data().items.is_empty());

    test_db.cleanup().await;
}

#[actix_rt::test]
async fn test_remove_non_existing_item_from_user_cart() {
    let test_db = utils::TestDatabase::new().await;

    utils::seed_roles(&test_db.pool).await;
    utils::seed_users(&test_db.pool).await;
    utils::seed_products(&test_db.pool).await;

    let srv = utils::create_test_server(test_db.pool.clone());

    let auth_token = utils::auto_login(&srv, "test1@test.com".to_string()).await;

    let payload = RemoveItemDto {
        product_id: Some(1),
    };

    let res = remove_item_from_user_cart(&srv, &auth_token, payload).await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND, "{:#?}", res);

    test_db.cleanup().await;
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
