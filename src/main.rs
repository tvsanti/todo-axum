mod error;
mod models;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Form, Json, Router,
};
use error::CustomError;
use models::{NewTodo, Todo};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use tower_http::cors::CorsLayer;
#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&url)
        .await
        .expect("Can not connect to database");
    let app = Router::new()
        .route("/", get(list))
        .route("/create", post(create))
        .route("/delete/:id", delete(delete_crud))
        .route("/update", put(update_crud))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn list(State(pool): State<PgPool>) -> Json<Vec<Todo>> {
    let todos = sqlx::query_as!(Todo, "SELECT * FROM todos")
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(todos)
}

async fn create(
    State(pool): State<PgPool>,
    Form(todo): Form<NewTodo>,
) -> Result<impl IntoResponse, CustomError> {
    let result = sqlx::query!(
        "INSERT INTO todos (description, done) VALUES ($1, $2) RETURNING Id",
        todo.description,
        todo.done
    )
    .fetch_one(&pool)
    .await?;
    dbg!("result:", result);

    Ok((StatusCode::OK).into_response())
}

async fn delete_crud(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, CustomError> {
    sqlx::query!("DELETE FROM todos WHERE id = ($1)", id,)
        .fetch_one(&pool)
        .await?;

    Ok((StatusCode::OK).into_response())
}

async fn update_crud(
    State(pool): State<PgPool>,
    Form(todo): Form<Todo>,
) -> Result<impl IntoResponse, CustomError> {
    sqlx::query!(
        "UPDATE todos SET description = ($1) WHERE id = ($2)",
        todo.description,
        todo.id
    )
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::OK).into_response())
}
