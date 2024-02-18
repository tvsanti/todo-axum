use crate::{
    error::CustomError,
    models::{Login, Register, User},
};
use axum::{
    extract::State,
    http::{
        header::SET_COOKIE, Response, StatusCode
    },
    response::{IntoResponse, Response},
    Form,
};
use cookie::Cookie;
use jwt_simple::prelude::*;
use sqlx::PgPool;

#[axum_macros::debug_handler]
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

#[axum_macros::debug_handler]
pub async fn login_handler(
    State(pool): State<PgPool>,
    Form(login): Form<Login>,
) -> Result<impl Response, CustomError> {
    let result = sqlx::query_as!(
        User,
        "SELECT id, username, email, password, CAST(created_at AS timestamptz) AS created_at, CAST(updated_at AS timestamptz) AS updated_at FROM users WHERE username = ($1) AND password = ($2)",
        login.username,
        login.password,
    )
    .fetch_optional(&pool)
    .await?;

        let key = HS256Key::generate();
        let claims = Claims::create(Duration::from_hours(2));
        let token = key.authenticate(claims)?;
        let cookie = Cookie::new("token", token);

        let response = Response::builder()
            .header(SET_COOKIE, cookie.to_string())
            .body(result).unwrap();
        Ok(response)
}
// let claims = key.verify_token::<NoCustomClaims>(&token, None)?;
