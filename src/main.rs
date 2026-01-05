use std::env;

#[tokio::main]
async fn main() {
    // Railway gives you a port automatically. This line reads it.
    let port = env::var("PORT").unwrap_or_else(|_| "4000".to_string());
    // Use 0.0.0.0 so the internet can reach the container
    let addr = format!("0.0.0.0:{}", port);

    println!("->> LISTENING on http://{}", addr);

    let _listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    // ... rest of your Axum code ...
}
