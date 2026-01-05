use axum::{
    http::{header, Method},
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // 1. Setup CORS (Crucial for React -> Railway connection)
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allows your frontend to talk to this API
        .allow_methods([Method::POST, Method::GET, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE]);

    // 2. Build our application with a route and the CORS layer
    let app = Router::new()
        .route("/api/login", post(login_handler))
        .layer(cors);

    // 3. Set the port (Railway provides this via environment variable)
    let port_str = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port: u16 = port_str.parse().expect("PORT must be a number");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("🚀 Server starting on http://{}", addr);

    // 4. Run the server
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Simple handler for the login route
async fn login_handler(Json(payload): Json<LoginRequest>) -> Json<LoginResponse> {
    println!("Login attempt for: {}", payload.username);

    // Replace this logic with your actual database check
    if payload.username == "admin" {
        Json(LoginResponse {
            success: true,
            message: "Welcome back!".to_string(),
        })
    } else {
        Json(LoginResponse {
            success: false,
            message: "Invalid credentials".to_string(),
        })
    }
}

// Data structures for JSON
#[derive(Deserialize)]
struct LoginRequest {
    username: String,
}

#[derive(Serialize)]
struct LoginResponse {
    success: bool,
    message: String,
}
