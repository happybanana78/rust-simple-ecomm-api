use crate::app::products::videos::dto::PublicProductVideo;
use crate::app::products::videos::model::ProductVideoModel;

pub trait IntoPublic<T> {
    fn into_public(self) -> T;
}

impl IntoPublic<PublicProductVideo> for ProductVideoModel {
    fn into_public(self) -> PublicProductVideo {
        PublicProductVideo::from(self)
    }
}

impl IntoPublic<Vec<PublicProductVideo>> for Vec<ProductVideoModel> {
    fn into_public(self) -> Vec<PublicProductVideo> {
        self.into_iter().map(PublicProductVideo::from).collect()
    }
}
