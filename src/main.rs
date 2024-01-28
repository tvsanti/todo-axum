use axum::{extract::State, http::Result, routing::get, Json, Router};
use sqlx::postgres::{PgPoolOptions, PgPool};
use std::time::Duration;
use tower_http::cors::CorsLayer;
use serde::{Deserialize, Serialize}
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
            get(list)
        )
        .with_state(pool)
        .layer(CorsLayer::very_permissive());
    let address = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(address, app).await.unwrap();

}

#[derive(Deserialize, Serialize)]
struct Todo {
    id: i64,
    description: String,
    isDone: bool,
}

async fn list(State(pool): State<PgPool>) -> Result<Json<Todo>> {
    let todos = sql
    x::query_as!(Todo, "SELECT id, description, isDone FROM todos ORDER BY id")
        .fetch_all(&pool)
        .await?;
    Ok(Json(todos))
}