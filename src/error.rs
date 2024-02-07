use axum::{http::StatusCode, response::IntoResponse};


pub struct CustomError(anyhow::Error);

impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        eprintln!("{:#?}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong",
        ).into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E: Into<anyhow::Error>> From<E> for CustomError {
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
