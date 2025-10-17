use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Estrutura de dados para um usuário
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// Estado compartilhado da aplicação
type AppState = Arc<Mutex<Vec<User>>>;

#[tokio::main]
async fn main() {
    // Inicializa o estado com alguns usuários
    let users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ];

    let app_state = Arc::new(Mutex::new(users));

    // Define as rotas
    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user).delete(delete_user))
        .with_state(app_state);

    // Inicia o servidor
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

// GET /users/:id - Busca um usuário específico
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

// POST /users - Cria um novo usuário
async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, Json<User>) {
    let mut users = state.lock().unwrap();

    let new_id = users.iter().map(|u| u.id).max().unwrap_or(0) + 1;

    let new_user = User {
        id: new_id,
        name: payload.name,
        email: payload.email,
    };

    users.push(new_user.clone());

    (StatusCode::CREATED, Json(new_user))
}

// DELETE /users/:id - Remove um usuário
async fn delete_user(Path(id): Path<u32>, State(state): State<AppState>) -> StatusCode {
    let mut users: std::sync::MutexGuard<'_, Vec<User>> = state.lock().unwrap();

    if let Some(pos) = users.iter().position(|u| u.id == id) {
        users.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

// Estrutura para criação de usuário
#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}
