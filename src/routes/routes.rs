use anyhow::Context;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Form, Json,
};
use axum_extra::{headers, TypedHeader};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::{
    error::CustomError,
    models::{Claims, NewTodo, ServerState, Todo},
};

#[axum_macros::debug_handler]
pub async fn list(
    state: State<ServerState>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> Result<Json<Vec<Todo>>, String> {
    verify_token(cookies, &state.public_key);
    let todos = sqlx::query_as!(Todo, "SELECT * FROM todos")
        .fetch_all(&state.pool)
        .await
        .unwrap();
    Ok(Json(todos))
}

pub async fn create(
    state: State<ServerState>,
    Form(todo): Form<NewTodo>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> Result<impl IntoResponse, CustomError> {
    verify_token(cookies, &state.public_key);
    sqlx::query!(
        "INSERT INTO todos (description, done) VALUES ($1, $2) RETURNING Id",
        todo.description,
        todo.done
    )
    .fetch_one(&state.pool)
    .await?;

    Ok((StatusCode::OK).into_response())
}

pub async fn delete_crud(
    state: State<ServerState>,
    Path(id): Path<i32>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> Result<impl IntoResponse, CustomError> {
    verify_token(cookies, &state.public_key);
    sqlx::query!("DELETE FROM todos WHERE id = ($1)", id,)
        .fetch_one(&state.pool)
        .await?;

    Ok((StatusCode::OK).into_response())
}

pub async fn update_crud(
    state: State<ServerState>,
    Form(todo): Form<Todo>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> Result<impl IntoResponse, CustomError> {
    verify_token(cookies, &state.public_key);
    sqlx::query!(
        "UPDATE todos SET description = ($1) WHERE id = ($2)",
        todo.description,
        todo.id
    )
    .fetch_one(&state.pool)
    .await?;

    Ok((StatusCode::OK).into_response())
}

async fn verify_token(cookies: headers::Cookie, public_key: &[u8]) {
    let token = cookies
        .get("Token")
        .context("unexpected error getting cookie name");

    match decode::<Claims>(
        &token.unwrap(),
        &DecodingKey::from_secret(public_key),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => panic!(),
            _ => panic!(),
        },
    };
}
