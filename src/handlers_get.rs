use actix_web::{get, web, HttpResponse, http::header};

use crate::{AppState, model};

// Получение путешественника по id
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

// Получение визита по id
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

// Получение локации по id
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

// Получение визитов пользователя (отсортированные по visited_at) согласно get параметрам 
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

    let mut country = "";

    if country_exist {
        country = params.country.as_ref().unwrap().as_str();

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

    let mut response_json = model::UserVisitsJSON {
        visits: Vec::with_capacity(end_idx-start_idx+1),
    };

    for i in start_idx..end_idx {
        let user_visit = &user.visits[i];
        let location = &data.storage.locations[user_visit.location as usize];

        if country_exist {
            let country_id = data.storage.countries.map.get(country).unwrap().clone();
            if location.country != country_id {
                continue;
            }
        }
        if to_distance_exist {
            if location.distance >= params.toDistance.as_ref().unwrap().clone() {
                continue;
            }
        }

        let visit = &data.storage.visits[user_visit.id as usize];

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