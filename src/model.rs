use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
// use std::fmt;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
  pub id: Option<String>,
  pub title: String,
  pub content: String,
  pub completed: Option<bool>,
  pub createdAt: Option<DateTime<Utc>>,
  pub updatedAt: Option<DateTime<Utc>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodoSchema {
  pub title: String,
  pub content: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoSchema {
  pub title: Option<String>,
  pub content: Option<String>,
  pub completed: Option<bool>,
}

impl From<CreateTodoSchema> for Todo {
  fn from(todo: CreateTodoSchema) -> Self {
    let now = Utc::now();
    Todo {
      id: None,
      title: todo.title,
      content: todo.content,
      completed: None,
      createdAt: Some(now),
      updatedAt: Some(now),
    }
  }
}

pub struct AppState {
  pub todo_db: Arc<Mutex<Vec<Todo>>>,
}

impl AppState {
  pub fn init() -> AppState {
    AppState {
      todo_db: Arc::new(Mutex::new(Vec::new())),
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct QueryOptions {
  pub limit: Option<usize>,
  pub page: Option<usize>,
}