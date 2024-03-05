use crate::{
    error::CustomError,
    models::{Login, Register, User},
};
use axum::{
    extract::State,
    http::{header::SET_COOKIE, Response, StatusCode},
    response::IntoResponse,
    Form,
};
use cookie::{Cookie, CookieJar};
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

// #[axum_macros::debug_handler]
// pub async fn login_handler(
//     State(pool): State<PgPool>,
//     Form(login): Form<Login>,
// ) -> Result<impl IntoResponse, CustomError> {
//     let result = sqlx::query_as!(
//         User,
//         "SELECT id, username, email, password, CAST(created_at AS timestamptz) AS created_at, CAST(updated_at AS timestamptz) AS updated_at FROM users WHERE username = ($1) AND password = ($2)",
//         login.username,
//         login.password,
//     )
//     .fetch_optional(&pool)
//     .await?;

//     if let Some(user) = result {
//         let user_json = serde_json::to_string(&user)?;
        
//         // let token = key.authenticate(claims)?;

//         // let mut jar: CookieJar = CookieJar::new();
        
//         // println!("123123");
//         // println!("{:?}",jar.add_original(("Token", token.clone())));

//         // let cookie = Cookie::new("Token", token);

//         Ok(Response::builder()
//             .header(SET_COOKIE, cookie.to_string())
//             .header("Content-Type", "application/json")
//             .body(user_json)
//             .unwrap())
//     } else {
//         let error_message = "User not found"; 
//         let response = Response::builder()
//             .status(StatusCode::NOT_FOUND)
//             .body(error_message.to_string())
//             .unwrap();

//         Ok(response)
//     }
// }