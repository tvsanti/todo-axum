use crate::{
    error::CustomError,
    models::{Claims, Login, Register, ServerState, User},
};
use axum::{
    extract::State,
    http::{header::SET_COOKIE, Response, StatusCode},
    response::IntoResponse,
    Form,
};
use cookie::Cookie;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

#[axum_macros::debug_handler]
pub async fn register_handler(
    state: State<ServerState>,
    Form(register): Form<Register>,
) -> Result<impl IntoResponse, CustomError> {
    sqlx::query!(
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3)",
        register.username,
        register.email,
        register.password,
    )
    .fetch_one(&state.pool)
    .await?;

    Ok((StatusCode::OK).into_response())
}

#[axum_macros::debug_handler]
pub async fn login_handler(
    state: State<ServerState>,
    Form(login): Form<Login>,
) -> Result<impl IntoResponse, CustomError> {
    let result = sqlx::query_as!(
        User,
        "SELECT id, username, email, password, CAST(created_at AS timestamptz) AS created_at, CAST(updated_at AS timestamptz) AS updated_at FROM users WHERE username = ($1) AND password = ($2)",
        login.username,
        login.password,
    )
    .fetch_optional(&state.pool)
    .await?;

    if let Some(user) = result {
        let user_json = serde_json::to_string(&user)?;
        
        let my_claims: Claims = Claims {
            sub: "b@b.com".to_owned(),
            company: "ACME".to_owned(),
            exp: 10000000000,
        };
    
        let header = Header {
            kid: Some("signing_key".to_owned()),
            alg: Algorithm::HS512,
            ..Default::default()
        };
    
        let token = match encode(&header, &my_claims, &EncodingKey::from_secret(&state.public_key)) {
            Ok(t) => t,
            Err(_) => panic!(), // in practice you would return the error
        };


        let cookie = Cookie::new("Token", token);
        
        Ok(Response::builder()
            .header(SET_COOKIE, cookie.to_string())
            .header("Content-Type", "application/json")
            .body(user_json)
            .unwrap())
    } else {
        let error_message = "User not found"; 
        let response = Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(error_message.to_string())
            .unwrap();

        Ok(response)
    }
}