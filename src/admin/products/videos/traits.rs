use crate::admin::products::videos::dto::AdminPublicProductVideo;
use crate::admin::products::videos::model::AdminProductVideoModel;

pub trait IntoPublic<T> {
    fn into_public(self) -> T;
}

impl IntoPublic<AdminPublicProductVideo> for AdminProductVideoModel {
    fn into_public(self) -> AdminPublicProductVideo {
        AdminPublicProductVideo::from(self)
    }
}

impl IntoPublic<Vec<AdminPublicProductVideo>> for Vec<AdminProductVideoModel> {
    fn into_public(self) -> Vec<AdminPublicProductVideo> {
        self.into_iter()
            .map(AdminPublicProductVideo::from)
            .collect()
    }
}
