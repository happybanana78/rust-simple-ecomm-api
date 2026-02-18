use crate::traits::HasId;
use chrono::{DateTime, Duration, Utc};
use fake::Dummy;
use fake::faker::chrono::en::DateTimeBetween;
use fake::faker::name::en::Name;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct AdminCategoryModel {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

impl HasId for AdminCategoryModel {
    fn get_id(&self) -> i64 {
        self.id
    }
}

#[derive(Dummy)]
pub struct AdminCategoryDummy {
    #[dummy(faker = "Name()")]
    pub name: String,

    #[dummy(faker = "Name()")]
    pub slug: String,

    pub is_active: bool,

    #[dummy(faker = "DateTimeBetween(Utc::now() - Duration::days(365), Utc::now())")]
    pub created_at: DateTime<Utc>,
}
