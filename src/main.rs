use axum::{routing::get, Router};
use sqlx::postgres::{PgPoolOptions, PgPool};
use std::time::Duration;
use tower_http::cors::CorsLayer;
#[tokio::main]
async fn main(){
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&url)
        .await
        .expect("Can not connect to database");


    let app = Router::new()
        .route(
            "/",
            get(index)
        )
        .with_state(pool)
        .layer(CorsLayer::very_permissive());
    let address = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(address, app).await.unwrap();


}

async fn index() -> String {
    format!("Hello, world")
}