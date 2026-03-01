use crate::utils::traits::HasId;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use fake::{Dummy, Fake, Faker};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct AdminProductVideoModel {
    pub id: i64,
    pub product_id: i64,
    pub url: String,
    pub alt: String,
    pub is_main: bool,
    pub sort: BigDecimal,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl HasId for AdminProductVideoModel {
    fn get_id(&self) -> i64 {
        self.id
    }
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct AdminProductVideoOnlySortModel {
    pub id: i64,
    pub sort: BigDecimal,
}

impl HasId for AdminProductVideoOnlySortModel {
    fn get_id(&self) -> i64 {
        self.id
    }
}

pub struct AdminProductVideoDummy {
    pub product_id: i64,
    pub url: String,
    pub alt: String,
    pub is_main: bool,
    pub sort: BigDecimal,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl Dummy<Faker> for AdminProductVideoDummy {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let url: String = fake::faker::internet::en::MACAddress().fake_with_rng(rng);

        Self {
            product_id: 1,
            url,
            alt: Faker.fake(),
            is_main: Faker.fake(),
            sort: BigDecimal::from_bigint(1.into(), 0),
            deleted_at: None,
            created_at: Faker.fake(),
        }
    }
}
