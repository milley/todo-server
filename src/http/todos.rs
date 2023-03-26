use axum::{extract::{Query, State}, response::IntoResponse, Json, http::StatusCode, Router, routing::get};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Db;

#[derive(Debug, Deserialize)]
struct CreateTodo {
    text: String,
}



#[derive(Debug, Serialize, Clone)]
pub struct Todo {
    id: Uuid,
    text: String,
    completed: bool,
}

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

async fn todos_index(
    pagination: Option<Query<Pagination>>,
    State(db): State<Db>
) -> impl IntoResponse {
    let todos = db.read().unwrap();

    let Query(pagination) = pagination.unwrap_or_default();

    let todos = todos
        .values()
        .skip(pagination.offset.unwrap_or(0))
        .take(pagination.limit.unwrap_or(usize::MAX))
        .cloned()
        .collect::<Vec<_>>();

    Json(todos)
}

async fn todos_create(State(db): State<Db>, Json(input): Json<CreateTodo>) -> impl IntoResponse {
    let todo = Todo {
        id: Uuid::new_v4(),
        text: input.text,
        completed: false,
    };

    db.write().unwrap().insert(todo.id, todo.clone());

    (StatusCode::CREATED, Json(todo))
}

pub fn router(db: Db) -> Router {
    Router::new()
        .route("/todos", get(todos_index).post(todos_create)).with_state(db)
}