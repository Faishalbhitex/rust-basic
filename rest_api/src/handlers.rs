use crate::models::{Todo, CreateTodoRequest, UpdateTodoRequest};
use crate::storage::Storage;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use std::sync::{Arc, Mutex};

// Type alias untuk state yang akan dishare antar handlers
pub type AppState = Arc<Mutex<Storage>>;

// GET /todos - Get all todos
pub async fn get_todos(State(storage): State<AppState>) -> Result<Json<Vec<Todo>>, StatusCode> {
    let storage = storage.lock().unwrap();
    Ok(Json(storage.get_all()))
}

// GET /todos/:id - Get specific todo
pub async fn get_todo(
    Path(id): Path<u32>,
    State(storage): State<AppState>,
) -> Result<Json<Todo>, StatusCode> {
    let storage = storage.lock().unwrap();
    match storage.get_by_id(id) {
        Some(todo) => Ok(Json(todo)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// POST /todos - Create new todo
pub async fn create_todo(
    State(storage): State<AppState>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<Json<Todo>, StatusCode> {
    let mut storage = storage.lock().unwrap();
    let todo = storage.create(payload.title);
    Ok(Json(todo))
}

// PUT /todos/:id - Update existing todo
pub async fn update_todo(
    Path(id): Path<u32>,
    State(storage): State<AppState>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<Json<Todo>, StatusCode> {
    let mut storage = storage.lock().unwrap();
    match storage.update(id, payload.title, payload.completed) {
        Some(todo) => Ok(Json(todo)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// DELETE /todos/:id - Delete todo
pub async fn delete_todo(
    Path(id): Path<u32>,
    State(storage): State<AppState>,
) -> Result<StatusCode, StatusCode> {
    let mut storage = storage.lock().unwrap();
    if storage.delete(id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
