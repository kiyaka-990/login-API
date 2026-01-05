// Add these imports at the top if not there
use std::env;

#[tokio::main]
async fn main() {
    // 1. Get the port from Railway's environment, or default to 4000
    let port = env::var("PORT").unwrap_or_else(|_| "4000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    println!("->> LISTENING on http://{}", addr);

    let _listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    // ... rest of your server code (axum::serve, etc.)
}
