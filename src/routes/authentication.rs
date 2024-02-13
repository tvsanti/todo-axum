use axum::{extract::State, http::StatusCode, response::IntoResponse, Form};
use sqlx::PgPool;

use crate::{error::CustomError, models::{Login, Register, User}};

pub async fn register_handler(    
    State(pool): State<PgPool>,
    Form(register): Form<Register>,
) -> Result<impl IntoResponse, CustomError> {
    sqlx::query!(
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3)",
        register.username,
        register.email,
        register.password,
    )
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::OK).into_response())
}


pub async fn login_handler(
    State(pool): State<PgPool>,
    Form(login): Form<Login>,
) -> Result<impl IntoResponse, CustomError> {
    let result = sqlx::query_as!(
        User,
        "SELECT id, username, email, password, CAST(created_at AS timestamptz) AS created_at, CAST(updated_at AS timestamptz) AS updated_at FROM users WHERE username = ($1) AND password = ($2)",
        login.username,
        login.password,
    )
    .fetch_optional(&pool)
    .await?;

    let result = result.expect("HOla").username;
    Ok(result)
}
