use sqlx::FromRow;

#[derive(FromRow)]
pub struct RoleModel {
    pub id: i64,
    pub name: String,
}
