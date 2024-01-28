use axum::{routing::get, Router};


#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));
    let address = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(address, app).await.unwrap();

}

async fn index() -> String {
    format!("Hello, world")
}