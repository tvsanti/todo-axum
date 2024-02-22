use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Form, Json,
};
use cookie::CookieJar;
use sqlx::PgPool;

use crate::{
    error::CustomError,
    models::{NewTodo, Todo},
};

#[axum_macros::debug_handler]
pub async fn list(State(pool): State<PgPool>) -> Json<Vec<Todo>> {
    let jar = CookieJar::new();
    println!("Estamos dentro");
    println!("{:?}",jar);
    if let Some(token) = jar.get("Token") {

        println!("Token: {}", token.value());
    }
    let todos = sqlx::query_as!(Todo, "SELECT * FROM todos")
        .fetch_all(&pool)
        .await
        .unwrap();
    
    Json(todos)
}

pub async fn create(
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

pub async fn delete_crud(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, CustomError> {
    sqlx::query!("DELETE FROM todos WHERE id = ($1)", id,)
        .fetch_one(&pool)
        .await?;

    Ok((StatusCode::OK).into_response())
}

pub async fn update_crud(
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
