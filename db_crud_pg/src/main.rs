use db_crud_pg::{connect_db, User, cli_crud_pg};
use sqlx::{Postgres, migrate::MigrateDatabase};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let separator = "=".repeat(20);

    let database_url = std::env::var("DATABASE_URL")?;
    
    // Extract database name from URL for creation
    let db_name = database_url.split('/').last().unwrap_or("crud_db");
    let server_url = database_url.rsplit_once('/').map(|(server, _)| server).unwrap_or(&database_url);
    
    // Check if database exists, create if not
    if !Postgres::database_exists(&database_url).await.unwrap_or(false) {
        println!("Database '{}' doesn't exist. Creating...", db_name);
        Postgres::create_database(&database_url).await?;
        println!("Database '{}' created successfully!", db_name);
    }

    // Connect to database
    let pool = connect_db(&database_url).await?;

    // Run migrations
    println!("Running database migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("Migrations completed successfully!");

    println!("\n#{separator} DB Start {separator}#\n");
    User::show_users(&pool).await?;
    cli_crud_pg(&pool).await?;
    println!("\n#{separator} DB End {separator}#\n");
    User::show_users(&pool).await?;

    pool.close().await;
    println!("\nProgram postgresql finished successfully!");
    Ok(())
}
