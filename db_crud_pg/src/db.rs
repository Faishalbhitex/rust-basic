use sqlx::{PgPool, postgres::PgPoolOptions, FromRow};
use rust_decimal::Decimal;


#[derive(Debug, FromRow, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub height: Decimal,
}

#[allow(dead_code)]
#[derive(Debug, FromRow)]
pub struct UserLog {
    pub id: i32,
    pub user_id: i32,
    pub action: String,
    pub create_at: chrono::DateTime<chrono::Utc>,
}

pub async fn connect_db(db_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
}
