use crate::models::{User, UserResponse};
use crate::state::{AppState, UserStore};
use axum::extract::State;
use axum::{
    extract::{Json as ExtractJson, Path},
    http::StatusCode,
    response::Json,
};
use chrono::Utc;
use serde_json::{Value, json};
use uuid::Uuid;

pub async fn hello() -> Json<Value> {
    Json(json!({"message": "Hello Word!"}))
}

pub async fn status() -> Json<Value> {
    let json_resp = json!({
        "status": "OK",
        "service": "Axum API",
        "version": "2.0.0",
        "database": "In-Memory HashMap"
    });

    Json(json_resp)
}
pub async fn create_users(
    State(state): State<AppState>,
    ExtractJson(user): ExtractJson<User>,
) -> (StatusCode, Json<serde_json::Value>) {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    let new_user = UserStore {
        id: id.clone(),
        name: user.name.clone(),
        email: user.email.clone(),
        create_at: now,
        update_at: now,
    };

    let mut users_guard = state.users.lock().await;
    users_guard.insert(id.clone(), new_user);
    drop(users_guard);

    println!("Create user: {} - {}", user.name, user.email);

    let user_resp: UserResponse = UserResponse {
        id,
        name: user.name,
        email: user.email,
        create_at: now,
        update_at: now,
    };

    (
        StatusCode::OK,
        Json(json!({
            "message": "User created",
            "user": user_resp
        })),
    )
}

pub async fn get_all_users(State(state): State<AppState>) -> (StatusCode, Json<Value>) {
    let users_guard = state.users.lock().await;
    let users: Vec<UserResponse> = users_guard
        .values()
        .map(|u| UserResponse {
            id: u.id.clone(),
            name: u.name.clone(),
            email: u.email.clone(),
            create_at: u.create_at,
            update_at: u.update_at,
        })
        .collect();
    (
        StatusCode::OK,
        Json(json!({
            "users": users,
            "count": users.len()
        })),
    )
}
pub async fn get_user_by_id(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> (StatusCode, Json<Value>) {
    let users_guard = state.users.lock().await;

    match users_guard.get(&id) {
        Some(user) => {
            let user_resp = UserResponse {
                id: user.id.clone(),
                name: user.name.clone(),
                email: user.email.clone(),
                create_at: user.create_at,
                update_at: user.update_at,
            };

            (StatusCode::OK, Json(json!({ "user": user_resp })))
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "User not found" })),
        ),
    }
}

pub async fn update_user_by_id(
    Path(id): Path<String>,
    State(state): State<AppState>,
    ExtractJson(user): ExtractJson<User>,
) -> (StatusCode, Json<Value>) {
    let mut users_guard = state.users.lock().await;

    match users_guard.get_mut(&id) {
        Some(existing_user) => {
            existing_user.name = user.name.clone();
            existing_user.email = user.email.clone();
            existing_user.update_at = Utc::now();

            let user_resp = UserResponse {
                id: existing_user.id.clone(),
                name: existing_user.name.clone(),
                email: existing_user.email.clone(),
                create_at: existing_user.create_at,
                update_at: existing_user.update_at,
            };

            println!("Update user {}: {} - {}", id, user.name, user.email);

            (
                StatusCode::OK,
                Json(json!({
                    "message": "User update",
                    "user": user_resp
                })),
            )
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "User not found" })),
        ),
    }
}

pub async fn delete_user_by_id(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> (StatusCode, Json<Value>) {
    let mut users_guard = state.users.lock().await;
    match users_guard.remove(&id) {
        Some(_) => {
            println!("Deleted user {}", id);
            (
                StatusCode::OK,
                Json(json!({
                    "message": format!("User {} deleted", id)
                })),
            )
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "User not found"
            })),
        ),
    }
}
