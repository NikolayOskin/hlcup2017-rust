pub mod dict;
pub mod load;
pub mod model;
pub mod storage;

use actix_web::{get, http::header, web, App, HttpResponse, HttpServer};
use std::{process, time::Duration};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut s = storage::Storage::new();

    println!("loading data to in-memory storage");

    if let Err(e) = load::run(&mut s) {
        println!("Run error: {}", e);
        process::exit(1);
    }

    println!(
        "loaded: users {}, visits {}, locations {}",
        s.users.len(),
        s.visits.len(),
        s.locations.len()
    );

    println!("starting web server");

    let state = AppState { storage: s };

    let data = web::Data::new(state);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(users)
            .service(visits)
            .service(locations)
            .service(user_visits)
    })
    .keep_alive(Duration::from_secs(30))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

struct AppState {
    storage: storage::Storage,
}

#[get("/users/{id}")]
async fn users(data: web::Data<AppState>, path: web::Path<(u32,)>) -> HttpResponse {
    let id = path.into_inner().0 as usize;

    let user = &data.storage.users[id.clone()];

    let user_json = model::UserJSON {
        id: id as u32,
        email: user.email.clone(),
        first_name: data
            .storage
            .first_names
            .get_by_idx(user.first_name.clone() as usize),
        last_name: data
            .storage
            .last_names
            .get_by_idx(user.last_name.clone() as usize),
        gender: user.gender.to_string(),
        birth_date: user.birth_date,
    };

    let serialized = serde_json::to_string(&user_json).unwrap();

    HttpResponse::Ok()
        .insert_header(header::ContentType::json())
        .body(serialized)
}

#[get("/visits/{id}")]
async fn visits(data: web::Data<AppState>, path: web::Path<(u32,)>) -> HttpResponse {
    let id = path.into_inner().0 as usize;

    let visit = &data.storage.visits[id.clone()];

    let visit_json = model::VisitJSON {
        id: id as u32,
        location: visit.location.clone(),
        user: visit.user.clone(),
        mark: visit.mark.clone(),
        visited_at: visit.visited_at.clone(),
    };

    let serialized = serde_json::to_string(&visit_json).unwrap();

    HttpResponse::Ok()
        .insert_header(header::ContentType::json())
        .body(serialized)
}

#[get("/locations/{id}")]
async fn locations(data: web::Data<AppState>, path: web::Path<(u32,)>) -> HttpResponse {
    let id = path.into_inner().0 as usize;

    let location = &data.storage.locations[id.clone()];

    let location_json = model::LocationJSON {
        id: id as u32,
        country: data
            .storage
            .countries
            .get_by_idx(location.country.clone() as usize),
        city: data
            .storage
            .cities
            .get_by_idx(location.city.clone() as usize),
        place: data
            .storage
            .places
            .get_by_idx(location.place.clone() as usize),
        distance: location.distance.clone(),
    };

    let serialized = serde_json::to_string(&location_json).unwrap();

    HttpResponse::Ok()
        .insert_header(header::ContentType::json())
        .body(serialized)
}

#[get("/users/{id}/visits")]
async fn user_visits(
    data: web::Data<AppState>,
    path: web::Path<(u32,)>,
    params: web::Query<model::UserVisitsParams>,
) -> HttpResponse {
    let id = path.into_inner().0 as usize;

    if id > data.storage.users.len() - 1 {
        return HttpResponse::NotFound().finish();
    }

    let user = &data.storage.users[id as usize];

    let country_exist = params.country.is_some();
    let from_date_exist = params.fromDate.is_some();
    let to_date_exist = params.toDate.is_some();
    let to_distance_exist = params.toDistance.is_some();

    if country_exist {
        let country = params.country.as_ref().unwrap().as_str();
        if country == "" {
            return HttpResponse::BadRequest().finish();
        }
        if data.storage.countries.exist(country) == false {
            return HttpResponse::NotFound().finish();
        }
    }

    let mut end_idx = user.visits.len() - 1;
    let mut start_idx: usize = 0;

    if to_date_exist {
        // найдем индекс до которого будем итерироваться (binary search)
        end_idx = user
            .visits
            .partition_point(|x| x.visited_at < params.toDate.as_ref().unwrap().clone());
    }
    if from_date_exist {
        // найдем индекс от которого будем итерироваться (binary search)
        start_idx = user
            .visits
            .partition_point(|x| x.visited_at <= params.fromDate.as_ref().unwrap().clone());
    }

    let mut visit_ids: Vec<usize> = Vec::new();

    for i in start_idx..end_idx {
        let user_visit = &user.visits[i];

        if country_exist {
            let country = params.country.as_ref().unwrap().as_str();
            let country_id = data.storage.countries.map.get(country).unwrap().clone();
            if user_visit.country != country_id {
                continue;
            }
        }
        if to_distance_exist {
            if user_visit.distance >= params.toDistance.as_ref().unwrap().clone() {
                continue;
            }
        }

        visit_ids.push(user_visit.id.clone());
    }

    let mut response_json = model::UserVisitsJSON {
        visits: Vec::with_capacity(visit_ids.len()),
    };

    for visit_id in visit_ids {
        let visit = &data.storage.visits[visit_id];
        let location = &data.storage.locations[visit.location as usize];

        response_json.visits.push(model::UserVisitJSON {
            mark: visit.mark,
            visited_at: visit.visited_at,
            place: data.storage.places.get_by_idx(location.place as usize),
        })
    }

    let resp = serde_json::to_string(&response_json);

    HttpResponse::Ok()
        .insert_header(header::ContentType::json())
        .body(resp.unwrap())
}
