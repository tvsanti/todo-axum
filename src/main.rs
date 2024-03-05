mod error;
mod models;
#[path = "./routes/routes.rs"]
mod routes;

#[path = "./routes/authentication.rs"]
mod authentication;

use axum::{
    routing::{delete, get, post, put}, Router
};
use jsonwebtoken::{decode, encode, errors::ErrorKind::InvalidToken, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use models::Claims;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tower_http::cors::CorsLayer;



#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();

    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost".to_string());

        let my_claims =
        Claims { sub: "b@b.com".to_owned(), company: "ACME".to_owned(), exp: 10000000000 };
    let key = b"secret";

    let header =
        Header { kid: Some("signing_key".to_owned()), alg: Algorithm::HS512, ..Default::default() };

    let token = match encode(&header, &my_claims, &EncodingKey::from_secret(key)) {
        Ok(t) => t,
        Err(_) => panic!(), // in practice you would return the error
    };

    println!("{:?}", token);
    

    let token_data = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(key),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            InvalidToken => panic!(), // Example on how to handle a specific error
            _ => panic!(),
        },
    };
    println!("{:?}", token_data.claims);
    println!("{:?}", token_data.header);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&url)
        .await
        .expect("Can not connect to database");
    let app = Router::new()
        .route("/", get(routes::list))
        .route("/create", post(routes::create))
        // .route("/login", post(authentication::login_handler))
        .route("/register", post(authentication::register_handler))
        .route("/delete/:id", delete(routes::delete_crud))
        .route("/update", put(routes::update_crud))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
