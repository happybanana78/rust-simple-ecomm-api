use actix_web::http::StatusCode;
use ecomm::app::products::dto::PublicProduct;
use ecomm::app::products::dto::{IndexProductDTO, ShowProductDTO};
use ecomm::responses::api_responses::{LocalApiPaginatedResponse, LocalApiResponse};
use ecomm::responses::error_responses::ErrorResponse;

mod utils;

fn get_index_url(payload: IndexProductDTO) -> String {
    format!(
        "/products/list?{}",
        serde_urlencoded::to_string(payload).unwrap()
    )
}

fn get_show_url(payload: Option<ShowProductDTO>, slug: String) -> String {
    match payload {
        Some(payload) => format!(
            "/products/get/{}?{}",
            slug,
            serde_urlencoded::to_string(payload).unwrap()
        ),
        None => format!("/products/get/{}", slug),
    }
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
        images: None,
        videos: None,
        reviews: None,
    };

    let url = get_index_url(query_payload);

    let mut res = context.srv.get(url).send().await.unwrap();

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    let body: LocalApiPaginatedResponse<Vec<PublicProduct>> = res.json().await.unwrap();

    assert_eq!(body.get_data().len(), 1); // only the active one
    assert!(body.get_data()[0].images.clone().is_none());
    assert!(body.get_data()[0].videos.clone().is_none());
    assert!(body.get_data()[0].reviews.clone().is_none());

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_product_index_with_all_relations() {
    let context = utils::TestContext::new(None).await;

    let query_payload = IndexProductDTO {
        page: Some(1),
        limit: Some(10),
        search: None,
        category: None,
        price_min: None,
        price_max: None,
        images: Some(true),
        videos: Some(true),
        reviews: Some(true),
    };

    let url = get_index_url(query_payload);

    let mut res = context.srv.get(url).send().await.unwrap();

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    let body: LocalApiPaginatedResponse<Vec<PublicProduct>> = res.json().await.unwrap();

    assert_eq!(body.get_data().len(), 1); // only the active one
    assert_eq!(
        body.get_data()[0].images.clone().unwrap()[0].alt,
        "p1 example image 1"
    );
    assert_eq!(
        body.get_data()[0].videos.clone().unwrap()[0].alt,
        "p1 example video 1"
    );
    assert_eq!(
        body.get_data()[0].reviews.clone().unwrap()[0].title,
        "product 1 review title 1"
    );

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_product_index_with_only_images_relation() {
    let context = utils::TestContext::new(None).await;

    let query_payload = IndexProductDTO {
        page: Some(1),
        limit: Some(10),
        search: None,
        category: None,
        price_min: None,
        price_max: None,
        images: Some(true),
        videos: None,
        reviews: None,
    };

    let url = get_index_url(query_payload);

    let mut res = context.srv.get(url).send().await.unwrap();

    assert!(
        res.status().is_success(),
        "detailed error: {:#?}",
        res.json::<ErrorResponse>().await.unwrap()
    );

    let body: LocalApiPaginatedResponse<Vec<PublicProduct>> = res.json().await.unwrap();

    assert_eq!(body.get_data().len(), 1); // only the active one
    assert_eq!(
        body.get_data()[0].images.clone().unwrap()[0].alt,
        "p1 example image 1"
    );
    assert!(body.get_data()[0].videos.clone().is_none());
    assert!(body.get_data()[0].reviews.clone().is_none());

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
        images: None,
        videos: None,
        reviews: None,
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
        images: None,
        videos: None,
        reviews: None,
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
async fn test_product_show_no_query_params() {
    let context = utils::TestContext::new(None).await;

    let url = get_show_url(None, "test-product-1".to_string());

    let mut res = context.srv.get(url).send().await.unwrap();

    assert!(res.status().is_success());

    let body: LocalApiResponse<PublicProduct> = res.json().await.unwrap();

    assert!(body.get_data().images.clone().is_none());
    assert!(body.get_data().videos.clone().is_none());
    assert!(body.get_data().reviews.clone().is_none());

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_product_show_with_all_relations() {
    let context = utils::TestContext::new(None).await;

    let query_payload = ShowProductDTO {
        images: Some(true),
        videos: Some(true),
        reviews: Some(true),
    };

    let url = get_show_url(Some(query_payload), "test-product-1".to_string());

    let mut res = context.srv.get(url).send().await.unwrap();

    assert!(res.status().is_success());

    let body: LocalApiResponse<PublicProduct> = res.json().await.unwrap();

    assert_eq!(
        body.get_data().images.clone().unwrap()[0].alt,
        "p1 example image 1"
    );
    assert_eq!(
        body.get_data().videos.clone().unwrap()[0].alt,
        "p1 example video 1"
    );
    assert_eq!(
        body.get_data().reviews.clone().unwrap()[0].title,
        "product 1 review title 1"
    );

    context.database.cleanup().await;
}

#[actix_rt::test]
async fn test_product_show_with_only_images_relation() {
    let context = utils::TestContext::new(None).await;

    let query_payload = ShowProductDTO {
        images: Some(true),
        videos: None,
        reviews: None,
    };

    let url = get_show_url(Some(query_payload), "test-product-1".to_string());

    let mut res = context.srv.get(url).send().await.unwrap();

    assert!(res.status().is_success());

    let body: LocalApiResponse<PublicProduct> = res.json().await.unwrap();

    assert_eq!(
        body.get_data().images.clone().unwrap()[0].alt,
        "p1 example image 1"
    );
    assert!(body.get_data().videos.clone().is_none());
    assert!(body.get_data().reviews.clone().is_none());

    context.database.cleanup().await;
}
