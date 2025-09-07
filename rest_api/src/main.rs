mod handlers;
mod models;
mod storage;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use handlers::AppState;
use std::sync::{Arc, Mutex};
use storage::Storage;

#[tokio::main]
async fn main() {
    // Initialize storage
    let storage = Storage::new().expect("Failed to initialize storage");
    let shared_state: AppState = Arc::new(Mutex::new(storage));

    // Define routes
    let app = Router::new()
        .route("/todos", get(handlers::get_todos))
        .route("/todos", post(handlers::create_todo))
        .route("/todos/:id", get(handlers::get_todo))
        .route("/todos/:id", put(handlers::update_todo))
        .route("/todos/:id", delete(handlers::delete_todo))
        .with_state(shared_state);

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("🚀 Server running on http://127.0.0.1:3000");
    println!("📝 Endpoints:");
    println!("  GET    /todos     - Get all todos");
    println!("  POST   /todos     - Create todo");
    println!("  GET    /todos/:id - Get specific todo");
    println!("  PUT    /todos/:id - Update todo");
    println!("  DELETE /todos/:id - Delete todo");

    axum::serve(listener, app).await.unwrap();
}
