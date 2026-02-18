use crate::admin::products::dto::AdminPublicProduct;
use crate::admin::products::images::dto::AdminPublicProductImage;
use crate::admin::products::model::AdminProductModel;
use crate::pagination::{DataCollection, PaginatedDataCollection};
use std::collections::HashMap;

pub trait IntoPublic<T> {
    fn into_public(self) -> T;
    fn into_public_with_images(self, images: Vec<AdminPublicProductImage>) -> T;
}

impl IntoPublic<AdminPublicProduct> for AdminProductModel {
    fn into_public(self) -> AdminPublicProduct {
        AdminPublicProduct::from(self)
    }

    fn into_public_with_images(self, images: Vec<AdminPublicProductImage>) -> AdminPublicProduct {
        AdminPublicProduct::from_model_with_images(self, images)
    }
}

impl IntoPublic<DataCollection<AdminPublicProduct>> for DataCollection<AdminProductModel> {
    fn into_public(self) -> DataCollection<AdminPublicProduct> {
        DataCollection::new(
            self.data
                .into_iter()
                .map(AdminPublicProduct::from)
                .collect(),
        )
    }

    fn into_public_with_images(
        self,
        images: Vec<AdminPublicProductImage>,
    ) -> DataCollection<AdminPublicProduct> {
        let mut images_by_product: HashMap<i64, Vec<AdminPublicProductImage>> = HashMap::new();

        for image in images {
            images_by_product
                .entry(image.product_id)
                .or_default()
                .push(image);
        }

        DataCollection::new(
            self.data
                .into_iter()
                .map(|product| {
                    let product_images = images_by_product.remove(&product.id).unwrap_or_default();

                    AdminPublicProduct::from_model_with_images(product, product_images)
                })
                .collect(),
        )
    }
}

impl IntoPublic<PaginatedDataCollection<AdminPublicProduct>>
    for PaginatedDataCollection<AdminProductModel>
{
    fn into_public(self) -> PaginatedDataCollection<AdminPublicProduct> {
        PaginatedDataCollection::new(
            self.data
                .into_iter()
                .map(AdminPublicProduct::from)
                .collect(),
            self.pagination,
        )
    }

    fn into_public_with_images(
        self,
        images: Vec<AdminPublicProductImage>,
    ) -> PaginatedDataCollection<AdminPublicProduct> {
        let mut images_by_product: HashMap<i64, Vec<AdminPublicProductImage>> = HashMap::new();

        for image in images {
            images_by_product
                .entry(image.product_id)
                .or_default()
                .push(image);
        }

        PaginatedDataCollection::new(
            self.data
                .into_iter()
                .map(|product| {
                    let product_images = images_by_product.remove(&product.id).unwrap_or_default();

                    AdminPublicProduct::from_model_with_images(product, product_images)
                })
                .collect(),
            self.pagination,
        )
    }
}
