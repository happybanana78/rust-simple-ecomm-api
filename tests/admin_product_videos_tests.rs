use bigdecimal::BigDecimal;
use ecomm::admin::products::videos::dto::{CreateProductVideoCommand, UpdateProductVideoSortDTO};
use ecomm::admin::products::videos::service::AdminProductVideoService;
use ecomm::responses::api_responses::LocalApiResponse;
use ecomm::responses::error_responses::ErrorResponse;
use ecomm::storage::LocalStorage;
use tempdir::TempDir;
use uuid::Uuid;

mod utils;

#[actix_rt::test]
async fn test_admin_product_video_upload() {
    let context = utils::TestContextNoServer::new().await;

    let temp_dir = TempDir::new(format!("test_dir_{}", Uuid::new_v4()).as_str()).unwrap();

    let admin_product_video_service = AdminProductVideoService::new(context.database.pool.clone());

    let command = CreateProductVideoCommand {
        product_id: 1,
        alt: "test alt 1".to_string(),
        sort: BigDecimal::from(1000),
        is_main: true,
        url: None,
    };

    let storage = LocalStorage::new(temp_dir.path().to_str().unwrap().to_string());

    let result = admin_product_video_service
        .upload(command, &storage, vec![1, 2, 3, 4, 5, 6], "png")
        .await;

    assert!(result.is_ok(), "{:?}", result);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_video_sort_update() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let auth_token = context.auth_token.clone().unwrap();

    let payload = UpdateProductVideoSortDTO {
        target_index: Some(1),
    };

    let mut res = context
        .srv
        .put(format!("/admin/products/videos/{}/update-sort", 1))
        .insert_header(("Authorization", format!("Bearer {}", auth_token)))
        .send_json(&payload)
        .await
        .unwrap();

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    let body: LocalApiResponse<BigDecimal> = res.json().await.unwrap();

    assert_eq!(body.get_data(), 3000);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_video_delete() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let auth_token = context.auth_token.clone().unwrap();

    let temp_dir = TempDir::new(format!("test_dir_{}", Uuid::new_v4()).as_str()).unwrap();

    let admin_product_video_service = AdminProductVideoService::new(context.database.pool.clone());

    let command = CreateProductVideoCommand {
        product_id: 1,
        alt: "test alt 1".to_string(),
        sort: BigDecimal::from(1000),
        is_main: true,
        url: None,
    };

    let path = temp_dir.path().to_str().unwrap().to_string();

    let storage = LocalStorage::new(path.clone());

    let video_id = admin_product_video_service
        .upload(command, &storage, vec![1, 2, 3, 4, 5, 6], "png")
        .await
        .unwrap();

    let mut res = context
        .srv
        .delete(format!("/admin/products/videos/delete/{}", video_id))
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
async fn test_admin_product_video_stream() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let auth_token = context.auth_token.clone().unwrap();

    let temp_dir = TempDir::new(format!("test_dir_{}", Uuid::new_v4()).as_str()).unwrap();

    let admin_product_video_service = AdminProductVideoService::new(context.database.pool.clone());

    let command = CreateProductVideoCommand {
        product_id: 1,
        alt: "test alt 1".to_string(),
        sort: BigDecimal::from(1000),
        is_main: true,
        url: None,
    };

    let path = temp_dir.path().to_str().unwrap().to_string();

    let storage = LocalStorage::new(path.clone());

    let video_id = admin_product_video_service
        .upload(command, &storage, vec![1, 2, 3, 4, 5, 6], "png")
        .await
        .unwrap();

    let mut res = context
        .srv
        .get(format!("/admin/products/videos/{}/stream", video_id))
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
