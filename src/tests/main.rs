// Write Tests for all of the above functions
#[cfg(test)]
mod tests {
    mod handler;

    use super::*;
    use actix_web::test;
    use actix_web::{http, web};
    use std::sync::{Arc, Mutex};

    #[actix_web::test]
    async fn test_health_check() {
        let app_state = web::Data::new(AppState {
            todo_db: Arc::new(Mutex::new(Vec::new())),
        });

        let req = test::TestRequest::get().uri("/api/v1/health-check").to_request();
        let resp = health_checker_handler(app_state.clone(), req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, "{\"status\":\"success\",\"message\":\"Everything is ok!\"}");
    }

    // #[actix_rt::test]
    // async fn test_get_todos() {
    //     let app_state = web::Data::new(AppState {
    //         todo_db: Mutex::new(vec![
    //             Todo {
    //                 id: Some("1".to_string()),
    //                 title: "First Todo".to_string(),
    //                 content: "First Todo Content".to_string(),
    //                 completed: false,
    //                 createdAt: Some(Utc::now()),
    //                 updatedAt: Some(Utc::now()),
    //             },
    //             Todo {
    //                 id: Some("2".to_string()),
    //                 title: "Second Todo".to_string(),
    //                 content: "Second Todo Content".to_string(),
    //                 completed: false,
    //                 createdAt: Some(Utc::now()),
    //                 updatedAt: Some(Utc::now()),
    //             },
    //             Todo {
    //                 id: Some("3".to_string()),
    //                 title: "Third Todo".to_string(),
    //                 content: "Third Todo Content".to_string(),
    //                 completed: false,
    //                 createdAt: Some(Utc::now()),
    //                 updatedAt: Some(Utc::now()),
    //             },
    //         ]),
    //     });

    //     let req = test::TestRequest::get().uri("/api/v1/todos").to_request();
    //     let resp = get_todos(app_state.clone(), req).await;
    //     assert_eq!(resp.status(), http::StatusCode::OK);

    //     let body = test::read_body(resp).await;
    //     let body = std::str::from_utf8(&body).unwrap();
    //     assert_eq!(
    //         body,
    //         "{\"status\":\"success\",\"results\":3,\"todos\":[{\"
    //         id\":\"1\",\"title\":\"First Todo\",\"content\":\"First Todo Content\",\"completed\":false,\"createdAt\":\"2021-05-31T12:00:00Z\",\"updatedAt\":\"2021-05-31T12:00:00Z\"},{\"id\":\"2\",\"title\":\"Second Todo\",\"content\":\"Second Todo Content\",\"completed\":false,\"createdAt\":\"2021-05-31T12:00:00Z\",\"updatedAt\":\"2021-05-31T12:00:00Z\"},{\"id\":\"3\",\"title\":\"Third Todo\",\"content\":\"Third Todo Content\",\"completed\":false,\"createdAt\":\"2021-05-31T12:00:00Z\",\"updatedAt\":\"2021-05-31T12:00:00Z\"}]}"
    //     );
    // }

    // #[actix_rt::test]
    // async fn test_create_todo() {
    //     let app_state = web::Data::new(AppState {
    //         todo_db: Mutex::new(vec![]),
    //     });

    //     let req = test::TestRequest::post()
    //         .uri("/api/v1/todos")
    //         .set_json(&json!({
    //             "title": "First Todo",
    //             "content": "First Todo Content",
    //             "completed": false
    //         }))
    //         .to_request();
    //     let resp = create_todo(app_state.clone(), req).await;
    //     assert_eq!(resp.status(), http::StatusCode::CREATED);

    //     let body = test::read_body(resp).await;
    //     let body = std::str::from_utf8(&body).unwrap();
    //     assert_eq!(
    //         body,
    //         "{\"status\":\"success\",\"data\":{\"todo\":{\"id\":\"1\",\"title\":\"First Todo\",\"content\":\"First Todo Content\",\"completed\":false,\"createdAt\":\"2021-05-31T12:00:00Z\",\"updatedAt\":\"2021-05-31T12:00:00Z\"}}}"
    //     );
    // }

    // #[actix_rt::test]
    // async fn test_get_todo_by_id() {
    //     let app_state = web::Data::new(AppState {
    //         todo_db: Mutex::new(vec![
    //             Todo {
    //                 id: Some("1".to_string()),
    //                 title: "First Todo".to_string(),
    //                 content: "First Todo Content".to_string(),
    //                 completed: false,
    //                 createdAt: Some(Utc::now()),
    //                 updatedAt: Some(Utc::now()),
    //             },
    //             Todo {
    //                 id: Some("2".to_string()),
    //                 title: "Second Todo".to_string(),
    //                 content: "Second Todo Content".to_string(),
    //                 completed: false,
    //                 createdAt: Some(Utc::now()),
    //                 updatedAt: Some(Utc::now()),
    //             },
    //             Todo {
    //                 id: Some("3".to_string()),
    //                 title: "Third Todo".to_string(),
    //                 content: "Third Todo Content".to_string(),
    //                 completed: false,
    //                 createdAt: Some(Utc::now()),
    //                 updatedAt: Some(Utc::now()),
    //             },
    //         ]),
    //     });

    //     let req = test::TestRequest::get()
    //         .uri("/api/v1/todos/2")
    //         .to_request();
    //     let resp = get_todo_by_id(app_state.clone(), req).await;
    //     assert_eq!(resp.status(), http::StatusCode::OK);

    //     let body = test::read_body(resp).await;
    //     let body = std::str::from_utf8(&body).unwrap();
    //     assert_eq!(
    //         body,
    //         "{\"status\":\"success\",\"data\":{\"todo\":{\"id\":\"2\",\"title\":\"Second Todo\",\"content\":\"Second Todo Content\",\"completed\":false,\"createdAt\":\"2021-05-31T12:00:00Z\",\"updatedAt\":\"2021-05-31T12:00:00Z\"}}}"
    //     );
    // }

    // #[actix_rt::test]
    // async fn test_update_todo_by_id() {
    //     let app_state = web::Data::new(AppState {
    //         todo_db: Mutex::new(vec![
    //             Todo {
    //                 id: Some("1".to_string()),
    //                 title: "First Todo".to_string(),
    //                 content: "First Todo Content".to_string(),
    //                 completed: false,
    //                 createdAt: Some(Utc::now()),
    //                 updatedAt: Some(Utc::now()),
    //             },
    //             Todo {
    //                 id: Some("2".to_string()),
    //                 title: "Second Todo".to_string(),
    //                 content: "Second Todo Content".to_string(),
    //                 completed: false,
    //                 createdAt: Some(Utc::now()),
    //                 updatedAt: Some(Utc::now()),
    //             },
    //             Todo {
    //                 id: Some("3".to_string()),
    //                 title: "Third Todo".to_string(),
    //                 content: "Third Todo Content".to_string(),
    //                 completed: false,
    //                 createdAt: Some(Utc::now()),
    //                 updatedAt: Some(Utc::now()),
    //             },
    //         ]),
    //     });

    //     let req = test::TestRequest::put()
    //         .uri("/api/v1/todos/2")
    //         .set_json(&json!({
    //             "title": "Second Todo Updated",
    //             "content": "Second Todo Content Updated",
    //             "completed": true
    //         }))
    //         .to_request();
    //     let resp = update_todo_by_id(app_state.clone(), req).await;
    //     assert_eq!(resp.status(), http::StatusCode::OK);

    //     let body = test::read_body(resp).await;
    //     let body = std::str::from_utf8(&body).unwrap();
    //     assert_eq!(
    //         body,
    //         "{\"status\":\"success\",\"data\":{\"todo\":{\"id\":\"2\",\"title\":\"Second Todo Updated\",\"content\":\"Second Todo Content Updated\",\"completed\":true,\"createdAt\":\"2021-05-31T12:00:00Z\",\"updatedAt\":\"2021-05-31T12:00:00Z\"}}}"
    //     );
    // }

    // #[actix_rt::test]
    // async fn test_delete_todo_by_id() {
    //     let app_state = web::Data::new(AppState {
    //         todo_db: Mutex::new(vec![
    //             Todo {
    //                 id: Some("1".to_string()),
    //                 title: "First Todo".to_string(),
    //                 content: "First Todo Content".to_string(),
    //                 completed: false,
    //                 createdAt: Some(Utc::now()),
    //                 updatedAt: Some(Utc::now()),
    //             },
    //             Todo {
    //                 id: Some("2".to_string()),
    //                 title: "Second Todo".to_string(),
    //                 content: "Second Todo Content".to_string(),
    //                 completed: false,
    //                 createdAt: Some(Utc::now()),
    //                 updatedAt: Some(Utc::now()),
    //             },
    //             Todo {
    //                 id: Some("3".to_string()),
    //                 title: "Third Todo".to_string(),
    //                 content: "Third Todo Content".to_string(),
    //                 completed: false,
    //                 createdAt: Some(Utc::now()),
    //                 updatedAt: Some(Utc::now()),
    //             },
    //         ]),
    //     });

    //     let req = test::TestRequest::delete()
    //         .uri("/api/v1/todos/2")
    //         .to_request();
    //     let resp = delete_todo_by_id(app_state.clone(), req).await;
    //     assert_eq!(resp.status(), http::StatusCode::OK);

    //     let body = test::read_body(resp).await;
    //     let body = std::str::from_utf8(&body).unwrap();
    //     assert_eq!(
    //         body,
    //         "{\"status\":\"success\",\"data\":{\"todo\":{\"id\":\"2\",\"title\":\"Second Todo\",\"content\":\"Second Todo Content\",\"completed\":false,\"createdAt\":\"2021-05-31T12:00:00Z\",\"updatedAt\":\"2021-05-31T12:00:00Z\"}}}"
    //     );
    // }

}