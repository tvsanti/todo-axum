mod error;
mod models;
#[path = "./routes/routes.rs"]
mod routes;

#[path = "./routes/authentication.rs"]
mod authentication;

use axum::{
    routing::{delete, get, post, put},
    Router,
};

use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();

    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost".to_string());



    let public_key = b"clave_publica";

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&url)
        .await
        .expect("Can not connect to database");
    let app = Router::new()
        .route("/", get(routes::list))
        .route("/create", post(routes::create))
        .route("/login", post(authentication::login_handler))
        .route("/register", post(authentication::register_handler))
        .route("/delete/:id", delete(routes::delete_crud))
        .route("/update", put(routes::update_crud))
        .with_state(models::ServerState { pool, public_key: *public_key })
        .layer(CorsLayer::very_permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
