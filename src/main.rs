use axum::{
    extract::State,
    http::{header, Method},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::FromRow;
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // 1. Load Environment Variables (Railway automatically sets DATABASE_URL)
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in Railway");

    // 2. Setup Database Connection Pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    // 3. Setup CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::POST, Method::GET, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE]);

    // 4. Build Application with Shared Database State
    let app = Router::new()
        .route("/", get(|| async { "Columbia API Online!" }))
        .route("/api/login", post(login_handler))
        .route("/api/stats", get(get_stats))
        .route("/api/products", get(list_products))
        .route("/api/admin/add-product", post(add_product))
        .with_state(pool) // This injects the database into all handlers
        .layer(cors);

    // 5. Port & Server start
    let port_str = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port: u16 = port_str.parse().expect("PORT must be a number");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("🚀 Columbia Server & DB connected on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// --- UPDATED HANDLERS (Runtime SQL Fix) ---

async fn list_products(State(pool): State<PgPool>) -> Json<Vec<Product>> {
    // Using query_as (Runtime) instead of query_as! (Compile-time) to bypass macro errors
    let products = sqlx::query_as::<_, Product>(
        "SELECT id, name, price, image_url FROM shop_items ORDER BY id DESC",
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    Json(products)
}

async fn add_product(
    State(pool): State<PgPool>,
    Json(payload): Json<NewProduct>,
) -> Json<LoginResponse> {
    // Using query (Runtime) with manual binding to bypass macro errors
    let result = sqlx::query("INSERT INTO shop_items (name, price, image_url) VALUES ($1, $2, $3)")
        .bind(&payload.name)
        .bind(payload.price)
        .bind(&payload.image_url)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Json(LoginResponse {
            success: true,
            message: format!("{} saved to Columbia inventory!", payload.name),
        }),
        Err(e) => Json(LoginResponse {
            success: false,
            message: format!("Database error: {}", e),
        }),
    }
}

// Existing handlers
async fn login_handler(Json(payload): Json<LoginRequest>) -> Json<LoginResponse> {
    if payload.username == "admin" || payload.username == "columbia_user" {
        Json(LoginResponse {
            success: true,
            message: "Welcome!".to_string(),
        })
    } else {
        Json(LoginResponse {
            success: false,
            message: "Invalid credentials.".to_string(),
        })
    }
}

async fn get_stats() -> Json<ProjectStats> {
    Json(ProjectStats {
        days_left: 14,
        completion_percentage: 92,
        pending_approvals: 1,
    })
}

// --- DATA STRUCTURES ---

#[derive(Serialize, FromRow)]
struct Product {
    id: i32,
    name: String,
    price: f64,
    image_url: String,
}

#[derive(Deserialize)]
struct NewProduct {
    name: String,
    price: f64,
    image_url: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
}

#[derive(Serialize)]
struct LoginResponse {
    success: bool,
    message: String,
}

#[derive(Serialize)]
struct ProjectStats {
    days_left: i32,
    completion_percentage: i32,
    pending_approvals: i32,
}
