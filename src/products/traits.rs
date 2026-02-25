use crate::pagination::PaginatedDataCollection;
use crate::products::dto::PublicProduct;
use crate::products::images::dto::PublicProductImage;
use crate::products::model::ProductModel;
use crate::products::videos::dto::PublicProductVideo;
use std::collections::HashMap;

pub trait IntoPublic<T> {
    fn into_public(self) -> T;
    fn into_public_with_media(
        self,
        images: Vec<PublicProductImage>,
        videos: Vec<PublicProductVideo>,
    ) -> T;
}

impl IntoPublic<PublicProduct> for ProductModel {
    fn into_public(self) -> PublicProduct {
        PublicProduct::from(self)
    }

    fn into_public_with_media(
        self,
        images: Vec<PublicProductImage>,
        videos: Vec<PublicProductVideo>,
    ) -> PublicProduct {
        PublicProduct::from_model_with_media(self, images, videos)
    }
}

impl IntoPublic<PaginatedDataCollection<PublicProduct>> for PaginatedDataCollection<ProductModel> {
    fn into_public(self) -> PaginatedDataCollection<PublicProduct> {
        PaginatedDataCollection::new(
            self.data.into_iter().map(PublicProduct::from).collect(),
            self.pagination,
        )
    }

    fn into_public_with_media(
        self,
        images: Vec<PublicProductImage>,
        videos: Vec<PublicProductVideo>,
    ) -> PaginatedDataCollection<PublicProduct> {
        let mut images_by_product: HashMap<i64, Vec<PublicProductImage>> = HashMap::new();
        let mut videos_by_product: HashMap<i64, Vec<PublicProductVideo>> = HashMap::new();

        for image in images {
            images_by_product
                .entry(image.product_id)
                .or_default()
                .push(image);
        }

        for video in videos {
            videos_by_product
                .entry(video.product_id)
                .or_default()
                .push(video);
        }

        PaginatedDataCollection::new(
            self.data
                .into_iter()
                .map(|product| {
                    let product_images = images_by_product.remove(&product.id).unwrap_or_default();
                    let product_videos = videos_by_product.remove(&product.id).unwrap_or_default();

                    PublicProduct::from_model_with_media(product, product_images, product_videos)
                })
                .collect(),
            self.pagination,
        )
    }
}
