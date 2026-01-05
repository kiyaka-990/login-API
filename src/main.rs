use axum::{
    Json, Router,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

#[derive(Serialize)]
struct Status {
    active: bool,
    message: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
}

#[tokio::main]
async fn main() {
    // 1. Build our application with routes
    let app = Router::new()
        .route("/", get(|| async { "Columbia API is Online" }))
        .route("/api/status", get(get_status))
        .route("/api/login", post(handle_login))
        .layer(CorsLayer::permissive());

    // 2. Define the address and create a TCP listener
    let addr = "127.0.0.1:4000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("->> LISTENING on http://{}", addr);

    // 3. Run the server
    axum::serve(listener, app).await.unwrap();
}

async fn get_status() -> Json<Status> {
    Json(Status {
        active: true,
        message: String::from("System operational"),
    })
}

async fn handle_login(Json(payload): Json<LoginRequest>) -> Json<Status> {
    println!("Login attempt from: {}", payload.username);
    Json(Status {
        active: true,
        message: format!("Welcome, {}!", payload.username),
    })
}
