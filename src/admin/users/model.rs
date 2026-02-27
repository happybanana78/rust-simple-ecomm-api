#[derive(sqlx::FromRow)]
pub struct AdminSafeUserModel {
    pub id: i64,
    pub username: String,
    pub email: String,
}
