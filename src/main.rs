pub mod dict;
pub mod load;
pub mod model;
pub mod storage;
pub mod handlers_get;
pub mod handlers_post;

use actix_web::{web, App, HttpServer};
use std::{process, time::Duration};

struct AppState {
    storage: storage::Storage,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut storage = storage::Storage::new();

    println!("loading data to in-memory storage");

    if let Err(e) = load::run(&mut storage) {
        println!("Run error: {}", e);
        process::exit(1);
    }

    println!(
        "loaded: users {}, visits {}, locations {}",
        storage.users.len(),
        storage.visits.len(),
        storage.locations.len()
    );

    println!("starting web server");

    let state = AppState { storage };

    let data = web::Data::new(state);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(handlers_get::users)
            .service(handlers_get::visits)
            .service(handlers_get::locations)
            .service(handlers_get::user_visits)
    })
    .keep_alive(Duration::from_secs(30))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// // Добавление пользователя
// #[post("/users/new")]
// async fn new_user(data: web::Data<AppState>) -> HttpResponse {
    
// }

// // Обновление пользователя
// #[post("/users/{id}")]
// async fn update_user(data: web::Data<AppState>) -> HttpResponse {
    
// }

// // Добавление визита
// #[post("/visits/new")]
// async fn new_visit(data: web::Data<AppState>) -> HttpResponse {
    
// }

// // Обновление визита
// #[post("/visits/{id}")]
// async fn update_visit(data: web::Data<AppState>) -> HttpResponse {
    
// }

// // Добавление локации
// #[post("/locations/new")]
// async fn new_location(data: web::Data<AppState>) -> HttpResponse {
    
// }

// // Обновление локации
// #[post("/locations/{id}")]
// async fn update_location(data: web::Data<AppState>) -> HttpResponse {
    
// }