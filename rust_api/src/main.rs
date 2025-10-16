use core::arch;
use std::{
    os::linux::raw::stat,
    sync::{Arc, Mutex},
    vec,
};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, get_service},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

type AppState = Arc<Mutex<Vec<User>>>;

#[tokio::main]
async fn main() {
    let users = vec![
        User {
            id: 1,
            name: "Joe Doe".to_string(),
            email: "joedoe@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Alice Lice".to_string(),
            email: "alli@example.com".to_string(),
        },
    ];

    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(get_users).post(create_user))
        .route("/user/:id", get(get_users).delete(delete_user))
        .with_state(app_state);

    let addr = "127.0.0.1:3000";

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("🚀 Servidor rodando em http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

// Handler para rota raiz
async fn root() -> &'static str {
    "API REST com Rust e Axum 🦀"
}

// GET /users - Lista todos os usuários
async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = state.lock().unwrap();
    Json(users.clone())
}

async fn get_user(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<Json<User>, StatusCode> {
    let users = state.lock().unwrap();

    users
        .iter()
        .find(|u| u.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, Json<User>) {
  
  
    let new_id = User {
        id: new_id,
        name: payload.no,
    };
}
