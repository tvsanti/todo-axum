use axum::{error_handling::HandleErrorLayer, http::StatusCode, routing::{delete, get, patch, post}, BoxError, Router};
use tokio::{sync::{broadcast::error, RwLock}, time::Timeout};
use tower::ServiceBuilder;


fn main() {
    let db = Db::default();
    let app = Router::new()
    .route("/create", get(createCRUD))
    .route("/read", post(readCRUD))
    .route("/delete/:id", delete(deleteCRUD))
    .route("/update/:id", patch(updateCRUD))
    .layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|error: BoxError| async move {
                if error.is::<tower::timeout::error::Elapsed>() {                    
                    Ok(StatusCode::REQUEST_TIMEOUT)
                } else {
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {error}"),
                    ))
                }
            }))
            .timeout(Duration::from_secs(10))
            .layer(TraceLayer::new_for_http())
            .into_inner(),
        );

    
}



fn createCRUD() {}
fn readCRUD() {}
fn deleteCRUD() {}
fn updateCRUD() {}