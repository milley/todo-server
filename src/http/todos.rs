use axum::{extract::{Query, State, Path}, Json, Router, routing::get};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{AppError, TodoRepoError};

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
) -> Result<Json<Vec<Todo>>, AppError> {
    let todos = db.read().unwrap();

    let Query(pagination) = pagination.unwrap_or_default();

    let todos = todos
        .values()
        .skip(pagination.offset.unwrap_or(0))
        .take(pagination.limit.unwrap_or(usize::MAX))
        .cloned()
        .collect::<Vec<_>>();

    Ok(Json(todos))
}

async fn todo_show(
    Path(todo_id): Path<Uuid>,
    State(db): State<Db>
) -> Result<Json<Todo>, AppError> {
    let todos = db.read().unwrap();
    match todos.get(&todo_id) {
        Some(todo) => Ok(Json(todo.to_owned())),
        None => Err(AppError::TodoRepo(TodoRepoError::NotFound)),
    }
}

async fn todos_create(State(db): State<Db>, Json(input): Json<CreateTodo>) -> Result<Json<Todo>, AppError> {
    let todo = Todo {
        id: Uuid::new_v4(),
        text: input.text,
        completed: false,
    };

    db.write().unwrap().insert(todo.id, todo.clone());

    Ok(todo.into())
}

pub fn router(db: Db) -> Router {
    Router::new()
        .route("/todos", get(todos_index).post(todos_create))
        .route("/todo/:id", get(todo_show)).with_state(db)
}