use crate::db::User;
use sqlx::PgPool;
use rust_decimal::Decimal;


impl User {
    // Create 
    pub async fn insert_user(
        pool: &PgPool,
        name: &str,
        age: i32,
        height: Decimal
    ) -> Result<Self, sqlx::Error> {
        let new_user = sqlx::query_as::<_, Self>(
            "INSERT INTO users (name, age, height) VALUES ($1, $2, $3) RETURNING id, name, age, height"
        )
            .bind(name)
            .bind(age)
            .bind(height)
            .fetch_one(pool)
            .await?;

        Ok(new_user)
    }

    // Read 
    pub async fn get_all_users(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let users = sqlx::query_as::<_, Self>("SELECT id, name, age, height FROM users")
            .fetch_all(pool)
            .await?;

        Ok(users)
    }
    pub async fn get_user_by_id(
        pool: &PgPool,
        id: i32
    ) -> Result<Option<Self>, sqlx::Error> {
        let user = sqlx::query_as::<_, Self> (
            "SELECT * FROM users WHERE id = $1"
        )
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }

    // Update 
    pub async fn update_user_age(
        pool: &PgPool,
        new_age: i32,
        id: i32
    ) -> Result<Self, sqlx::Error> {
        let update_user = sqlx::query_as::<_, Self> (
            "UPDATE users
        SET age = $1
        WHERE id = $2 RETURNING *"
        )
            .bind(new_age)
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(update_user)
    }

    // Delete 
    pub async fn delete_user(
        pool: &PgPool,
        id: i32
    ) -> Result<u64, sqlx::Error> {
        let rows_affected = sqlx::query(
            "DELETE FROM users WHERE id = $1"
        )
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected();

        Ok(rows_affected)
    }

    // Display users
    pub async fn show_users(pool: &PgPool) -> Result<(), sqlx::Error> {
        let users = Self::get_all_users(pool).await?;
        for u in &users {
            println!("Database users: {:?}", u);
        }

        Ok(())
    }

}
