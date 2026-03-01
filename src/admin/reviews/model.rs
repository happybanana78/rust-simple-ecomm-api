use crate::admin::reviews::dto::ReviewApprovalStatus;
use crate::utils::traits::HasId;
use chrono::{DateTime, Utc};
use fake::{Dummy, Fake, Faker};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct AdminReviewModel {
    pub id: i64,
    pub user_id: Option<i64>,
    pub product_id: i64,
    pub title: String,
    pub content: String,
    pub rating: i16,
    pub approval_status: ReviewApprovalStatus,
    pub created_at: DateTime<Utc>,
}

impl HasId for AdminReviewModel {
    fn get_id(&self) -> i64 {
        self.id
    }
}

pub struct AdminReviewDummy {
    pub user_id: Option<i64>,
    pub product_id: i64,
    pub title: String,
    pub content: String,
    pub rating: i16,
    pub approval_status: ReviewApprovalStatus,
    pub created_at: DateTime<Utc>,
}

// impl Dummy<Faker> for AdminReviewDummy {
//     fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
//         let name: String = fake::faker::name::en::Name().fake_with_rng(rng);
//
//         Self {
//             name,
//             slug: format!("slug-{}", Uuid::new_v4()),
//             price: rng.random_range(10.0..200.0),
//             quantity: rng.random_range(12..200),
//             configurable: false,
//             is_active: true,
//             created_at: Utc::now(),
//         }
//     }
// }
