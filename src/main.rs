mod error;
mod models;
#[path = "./routes/routes.rs"]
mod routes;

#[path = "./routes/authentication.rs"]
mod authentication;
mod config;

use std::sync::Arc;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use config::Config;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;

pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();

    let config = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/", get(routes::list))
        .route("/create", post(routes::create))
        .route("/login", post(authentication::login_handler))
        .route("/register", post(authentication::register_handler))
        .route("/delete/:id", delete(routes::delete_crud))
        .route("/update", put(routes::update_crud))
        .with_state(Arc::new(AppState {
            db: pool.clone(),
            env: config.clone(),
        }))
        .layer(CookieManagerLayer::new())
        .layer(CorsLayer::very_permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
