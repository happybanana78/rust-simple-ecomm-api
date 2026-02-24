use ecomm::admin::products::images::dto::CreateProductImageCommand;
use ecomm::admin::products::images::service::AdminProductImageService;
use ecomm::responses::error_responses::ErrorResponse;
use ecomm::storage::LocalStorage;
use tempdir::TempDir;
use uuid::Uuid;

mod utils;

#[actix_rt::test]
async fn test_admin_product_image_upload() {
    let context = utils::TestContextNoServer::new().await;

    let temp_dir = TempDir::new(format!("test_dir_{}", Uuid::new_v4()).as_str()).unwrap();

    let admin_product_image_service = AdminProductImageService::new(context.database.pool.clone());

    let command = CreateProductImageCommand {
        product_id: 1,
        alt: "test alt 1".to_string(),
        sort: 0,
        is_main: true,
        url: None,
    };

    let storage = LocalStorage::new(temp_dir.path().to_str().unwrap().to_string());

    let result = admin_product_image_service
        .upload(command, &storage, vec![1, 2, 3, 4, 5, 6], "png")
        .await;

    assert!(result.is_ok(), "{:?}", result);

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_admin_product_image_delete() {
    let context = utils::TestContext::new(Some("admin1@admin.com".to_string())).await;

    let auth_token = context.auth_token.clone().unwrap();

    let temp_dir = TempDir::new(format!("test_dir_{}", Uuid::new_v4()).as_str()).unwrap();

    let admin_product_image_service = AdminProductImageService::new(context.database.pool.clone());

    let command = CreateProductImageCommand {
        product_id: 1,
        alt: "test alt 1".to_string(),
        sort: 0,
        is_main: true,
        url: None,
    };

    let path = temp_dir.path().to_str().unwrap().to_string();

    let storage = LocalStorage::new(path.clone());

    let image_id = admin_product_image_service
        .upload(command, &storage, vec![1, 2, 3, 4, 5, 6], "png")
        .await
        .unwrap();

    let mut res = context
        .srv
        .delete(format!("/admin/products/images/delete/{}", image_id))
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
