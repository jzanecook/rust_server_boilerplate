use crate::{
    model::{AppState, CreateTodoSchema, QueryOptions, Todo, UpdateTodoSchema},
    response::{GenericResponse, SingleTodoResponse, TodoData, TodoListResponse},
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use uuid::Uuid;

// Health checker
#[get("/health-check")]
pub async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Actix Server Boilerplate is running.";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

// Get all todos
#[get("/todos")]
async fn get_todos(
    app_state: web::Data<AppState>,
    query: web::Query<QueryOptions>,
) -> impl Responder {
    let todo_db = app_state.todo_db.lock().unwrap();
    let limit = query.limit.unwrap_or(10);
    let page = query.page.unwrap_or(1);
    let offset = (page - 1) * limit;
    let results = todo_db.len();
    let todos = todo_db
        .iter()
        .skip(offset)
        .take(limit)
        .cloned()
        .collect::<Vec<Todo>>();

    let response_json = &TodoListResponse {
        status: "success".to_string(),
        results,
        todos,
    };

    HttpResponse::Ok().json(response_json)
}

// Create new todo
#[post("/todos")]
async fn create_todo(
    app_state: web::Data<AppState>,
    payload: web::Json<CreateTodoSchema>,
) -> impl Responder {
    let mut todo_db = app_state.todo_db.lock().unwrap();
    let mut todo = Todo::from(payload.into_inner());
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    todo.id = Some(id);
    todo.createdAt = Some(now);
    todo.updatedAt = Some(now);
    todo_db.push(todo.clone());

    let response_json = &SingleTodoResponse {
        status: "success".to_string(),
        data: TodoData { todo },
    };

    HttpResponse::Ok().json(response_json)
}

// Get single todo by id
#[get("/todos/{id}")]
async fn get_todo_by_id(app_state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let todo_db = app_state.todo_db.lock().unwrap();
    let id = path.into_inner();
    let todo = todo_db.iter().find(|todo| todo.id == Some(id.clone()));

    if let Some(todo) = todo {
        let response_json = &SingleTodoResponse {
            status: "success".to_string(),
            data: TodoData { todo: todo.clone() },
        };
        HttpResponse::Ok().json(response_json)
    } else {
        let response_json = &GenericResponse {
            status: "error".to_string(),
            message: "Todo not found.".to_string(),
        };
        HttpResponse::NotFound().json(response_json)
    }
}

// Patch route for todos
#[patch("/todos/{id}")]
async fn update_todo_by_id(
    app_state: web::Data<AppState>,
    path: web::Path<String>,
    payload: web::Json<UpdateTodoSchema>,
) -> impl Responder {
    let mut todo_db = app_state.todo_db.lock().unwrap();
    let id = path.into_inner();
    let payload = payload.into_inner();
    let todo = todo_db.iter_mut().find(|todo| todo.id == Some(id.clone()));

    if let Some(todo) = todo {
        let now = Utc::now();
        todo.title = payload.title.unwrap_or(todo.title.clone());
        todo.content = payload.content.unwrap_or(todo.content.clone());
        todo.completed = payload.completed;
        todo.updatedAt = Some(now);

        let response_json = &SingleTodoResponse {
            status: "success".to_string(),
            data: TodoData { todo: todo.clone() },
        };
        HttpResponse::Ok().json(response_json)
    } else {
        let response_json = &GenericResponse {
            status: "error".to_string(),
            message: "Todo not found.".to_string(),
        };
        HttpResponse::NotFound().json(response_json)
    }
}

// Delete route for todos
#[delete("/todos/{id}")]
async fn delete_todo_by_id(
    app_state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let mut todo_db = app_state.todo_db.lock().unwrap();
    let id = path.into_inner();
    let todo = todo_db.iter().position(|todo| todo.id == Some(id.clone()));

    if let Some(todo) = todo {
        todo_db.remove(todo);
        let response_json = &GenericResponse {
            status: "success".to_string(),
            message: "Todo deleted successfully.".to_string(),
        };
        HttpResponse::Ok().json(response_json)
    } else {
        let response_json = &GenericResponse {
            status: "error".to_string(),
            message: "Todo not found.".to_string(),
        };
        HttpResponse::NotFound().json(response_json)
    }
}

// Merge the Routes
pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/v1")
        .service(health_checker_handler)
        .service(get_todos)
        .service(create_todo)
        .service(get_todo_by_id)
        .service(update_todo_by_id)
        .service(delete_todo_by_id);

    conf.service(scope);
}

#[cfg(test)]
mod v1test {
    use super::*;
    use crate::model::CreateTodoSchema;
    use actix_web::{http, test, App};

    #[actix_web::test]
    async fn health_checker_test() {
        let mut app = test::init_service(App::new().service(health_checker_handler)).await;
        let req = test::TestRequest::get().uri("/health-check").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn get_todos_test() {
        let todo_db = AppState::init();
        let app_data = web::Data::new(todo_db);
        let mut app =
            test::init_service(App::new().app_data(app_data.clone()).service(get_todos)).await;
        let req = test::TestRequest::get().uri("/todos").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn create_todo_test() {
        let todo_db = AppState::init();
        let app_data = web::Data::new(todo_db);
        let mut app =
            test::init_service(App::new().app_data(app_data.clone()).service(create_todo)).await;
        let body = CreateTodoSchema {
            title: "Test Todo".to_string(),
            content: "Test Todo Content".to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/todos")
            .set_json(&body)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    // Test creating and getting a todo by id
    #[actix_web::test]
    async fn get_todo_by_id_test() {
        let todo_db = AppState::init();
        let app_data = web::Data::new(todo_db);
        let mut app = test::init_service(
            App::new()
                .app_data(app_data.clone())
                .service(create_todo)
                .service(get_todo_by_id),
        )
        .await;
        let body = CreateTodoSchema {
            title: "Test Todo".to_string(),
            content: "Test Todo Content".to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/todos")
            .set_json(&body)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
        let todo: SingleTodoResponse = test::read_body_json(resp).await;
        let id = todo.data.todo.id.unwrap();
        let req = test::TestRequest::get()
            .uri(&format!("/todos/{}", id))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn get_todo_by_id_not_found_test() {
        let todo_db = AppState::init();
        let app_data = web::Data::new(todo_db);
        let mut app = test::init_service(
            App::new()
                .app_data(app_data.clone())
                .service(get_todo_by_id),
        )
        .await;
        let id = uuid::Uuid::new_v4().to_string();
        let req = test::TestRequest::get()
            .uri(&format!("/todos/{}", id))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }

    // Test creating and updating a todo by id
    #[actix_web::test]
    async fn update_todo_by_id_test() {
        let todo_db = AppState::init();
        let app_data = web::Data::new(todo_db);
        let mut app = test::init_service(
            App::new()
                .app_data(app_data.clone())
                .service(create_todo)
                .service(update_todo_by_id),
        )
        .await;
        let body = CreateTodoSchema {
            title: "Test Todo".to_string(),
            content: "Test Todo Content".to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/todos")
            .set_json(&body)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
        let todo: SingleTodoResponse = test::read_body_json(resp).await;
        let id = todo.data.todo.id.unwrap();
        let body = UpdateTodoSchema {
          title: None,
          content: None,
          completed: Some(true),
        };
        let req = test::TestRequest::patch()
            .uri(&format!("/todos/{}", id))
            .set_json(&body)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
