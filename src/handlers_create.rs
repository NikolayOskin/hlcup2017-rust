use actix_web::{http::header, post, web, HttpResponse};

use crate::{model, AppState};

#[post("/users/new")]
async fn new_user(data: web::Data<AppState>, user: web::Json<model::UserJSON>) -> HttpResponse {
    let mut s = data.storage.write().unwrap();

    if s.users.len() > user.id as usize {
        // user with this id already exists
        return HttpResponse::BadRequest()
            .insert_header(header::ContentType::json())
            .body("{}");
    }

    let res = s.store_user(
        user.id as usize,
        &user.email,
        &user.first_name,
        &user.last_name,
        user.birth_date,
        user.gender.as_str(),
    );

    match res {
        Ok(_) => {
            HttpResponse::Ok()
                .insert_header(header::ContentType::json())
                .body("{}")
        }
        Err(_) => {
            HttpResponse::BadRequest()
                .insert_header(header::ContentType::json())
                .body("{}")
        }
    }
}

#[post("/locations/new")]
async fn new_location(
    data: web::Data<AppState>,
    location: web::Json<model::LocationJSON>,
) -> HttpResponse {
    let mut s = data.storage.write().unwrap();

    if s.locations.len() > location.id as usize {
        // location is already exist
        return HttpResponse::BadRequest()
            .insert_header(header::ContentType::json())
            .body("{}");
    }

    s.store_location(
        location.id as usize,
        &location.country,
        &location.city,
        &location.place,
        location.distance,
    );

    HttpResponse::Ok()
        .insert_header(header::ContentType::json())
        .body("{}")
}

#[post("/visits/new")]
async fn new_visit(data: web::Data<AppState>, visit: web::Json<model::VisitJSON>) -> HttpResponse {
    let mut s = data.storage.write().unwrap();

    if s.visits.len() > visit.id as usize {
        // visit is already exist
        return HttpResponse::BadRequest()
            .insert_header(header::ContentType::json())
            .body("{}");
    }

    s.store_visit(
        visit.id,
        visit.user,
        visit.location,
        visit.visited_at,
        visit.mark,
    );

    HttpResponse::Ok()
        .insert_header(header::ContentType::json())
        .body("{}")
}
