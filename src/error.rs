use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum TodoRepoError {
    NotFound,
    InvalidTitle,
}

pub enum AppError {
    TodoRepo(TodoRepoError),
}

impl From<TodoRepoError> for AppError {
    fn from(inner: TodoRepoError) -> Self {
        AppError::TodoRepo(inner)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::TodoRepo(TodoRepoError::NotFound) => {
                (StatusCode::NOT_FOUND, "Todo not found")
            }
            AppError::TodoRepo(TodoRepoError::InvalidTitle) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Invalid title")
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
