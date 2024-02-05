use axum::{
    extract::State,
    routing::{get, post},
    Form, Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use tower_http::cors::CorsLayer;
#[tokio::main]
async fn main() {
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
        .route("/", get(list))
        .route("/create", post(create))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize,Deserialize, Debug)]
pub struct NewTodo {
    pub id: i32,
    pub description: String,
    pub done: bool,
}
async fn list(State(pool): State<PgPool>) -> Json<Vec<NewTodo>>{
    let todos = sqlx::query_as!(NewTodo, "SELECT * FROM todos").fetch_all(&pool).await.unwrap();
    Json(todos)
}


async fn create(State(pool): State<PgPool>, Form(todo): Form<NewTodo>) {
    let result = sqlx::query!(
        "INSERT INTO todos (id, description, done) VALUES ($1, $2, $3) RETURNING Id",
        todo.id,
        todo.description,
        todo.done
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    dbg!("result:", result);
}
