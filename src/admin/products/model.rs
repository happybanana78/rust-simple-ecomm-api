use crate::traits::HasId;
use chrono::{DateTime, Utc};
use fake::{Dummy, Fake, Faker};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct AdminProductModel {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub price: f64,
    pub quantity: i32,
    pub configurable: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

impl HasId for AdminProductModel {
    fn get_id(&self) -> i64 {
        self.id
    }
}

pub struct AdminProductDummy {
    pub name: String,

    pub slug: String,

    pub price: f64,

    pub quantity: i32,

    pub configurable: bool,

    pub is_active: bool,

    pub created_at: DateTime<Utc>,
}

impl Dummy<Faker> for AdminProductDummy {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let name: String = fake::faker::name::en::Name().fake_with_rng(rng);

        Self {
            name,
            slug: format!("slug-{}", Uuid::new_v4()),
            price: rng.random_range(10.0..200.0),
            quantity: rng.random_range(12..200),
            configurable: false,
            is_active: true,
            created_at: Utc::now(),
        }
    }
}
