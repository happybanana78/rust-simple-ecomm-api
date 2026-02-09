use crate::products::dto::PublicProduct;
use crate::products::model::ProductModel;
use crate::products::traits::IntoPublic;

fn create_test_product(id: i64) -> ProductModel {
    ProductModel {
        id,
        name: format!("Product {}", id),
        price: 99.99,
        quantity: 10,
        configurable: true,
        is_active: true,
    }
}

#[test]
fn test_public_product_from_product_model() {
    let product = create_test_product(1);
    let public_product = PublicProduct::from(product);

    assert_eq!(public_product.name, "Product 1");
    assert_eq!(public_product.price, 99.99);
    assert_eq!(public_product.quantity, 10);
    assert!(public_product.configurable);
    assert!(public_product.is_active);
}

#[test]
fn test_into_public_trait_single_product() {
    let product = create_test_product(1);
    let public_product: PublicProduct = product.into_public();

    assert_eq!(public_product.name, "Product 1");
    assert_eq!(public_product.price, 99.99);
}

#[test]
fn test_into_public_trait_vec_products() {
    let products = vec![
        create_test_product(1),
        create_test_product(2),
        create_test_product(3),
    ];

    let public_products: Vec<PublicProduct> = products.into_public();

    assert_eq!(public_products.len(), 3);
    assert_eq!(public_products[0].name, "Product 1");
    assert_eq!(public_products[1].name, "Product 2");
    assert_eq!(public_products[2].name, "Product 3");
}

#[test]
fn test_into_public_trait_empty_vec() {
    let products: Vec<ProductModel> = vec![];
    let public_products: Vec<PublicProduct> = products.into_public();

    assert_eq!(public_products.len(), 0);
}

#[test]
fn test_public_product_with_decimal_price() {
    let product = PublicProduct {
        id: 1,
        name: "Precise Price".to_string(),
        price: 12.345,
        quantity: 100,
        configurable: false,
        is_active: true,
    };

    assert_eq!(product.price, 12.345);
}

#[test]
fn test_public_product_with_zero_price() {
    let product = PublicProduct {
        id: 1,
        name: "Free Item".to_string(),
        price: 0.0,
        quantity: 1000,
        configurable: false,
        is_active: true,
    };

    assert_eq!(product.price, 0.0);
}

#[test]
fn test_public_product_with_large_quantity() {
    let product = PublicProduct {
        id: 1,
        name: "Bulk Item".to_string(),
        price: 0.99,
        quantity: i32::MAX,
        configurable: false,
        is_active: true,
    };

    assert_eq!(product.quantity, i32::MAX);
}

#[test]
fn test_public_product_with_unicode_name() {
    let product = PublicProduct {
        id: 1,
        name: "商品 🛒 Product".to_string(),
        price: 25.00,
        quantity: 10,
        configurable: false,
        is_active: true,
    };

    assert_eq!(product.name, "商品 🛒 Product");
}

#[test]
fn test_public_product_with_empty_name() {
    let product = PublicProduct {
        id: 1,
        name: String::new(),
        price: 10.00,
        quantity: 1,
        configurable: false,
        is_active: true,
    };

    assert_eq!(product.name, "");
}

#[test]
fn test_public_product_with_long_name() {
    let long_name = "A".repeat(1000);
    let product = PublicProduct {
        id: 1,
        name: long_name.clone(),
        price: 15.99,
        quantity: 5,
        configurable: false,
        is_active: true,
    };

    assert_eq!(product.name.len(), 1000);
    assert_eq!(product.name, long_name);
}
