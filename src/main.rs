mod handler;
mod model;
mod response;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, App, HttpServer, web};
use serde::Serialize;
use model::AppState;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info")
    }
    env_logger::init();

    let todo_db = AppState::init();
    let app_data = web::Data::new(todo_db);

    println!("Starting Actix Server Boilerplate...");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8000")
            .allowed_origin("http://localhost:8000/")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(app_data.clone())
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}