use std::{collections::HashSet, usize};

use crate::{dict::Dict, model};
use chrono::{DateTime, NaiveDateTime, Utc};

// время генерации данных
const CURR_TIMESTAMP: i64 = 1544576406;

pub struct Storage {
    pub users: Vec<model::User>,
    pub visits: Vec<model::Visit>,
    pub locations: Vec<model::Location>,

    // сет для проверки дублей email'ов при добавлении пользователя
    emails: HashSet<String>,

    pub last_names: Dict,
    pub first_names: Dict,
    pub countries: Dict,
    pub cities: Dict,
    pub places: Dict,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            users: Vec::new(),
            visits: Vec::new(),
            locations: Vec::new(),

            emails: HashSet::new(),

            last_names: Dict::new(),
            first_names: Dict::new(),
            countries: Dict::new(),
            cities: Dict::new(),
            places: Dict::new(),
        }
    }

    pub fn store_user(
        &mut self,
        id: usize,
        email: &str,
        first_name: &str,
        last_name: &str,
        birth_date: i32,
        gender: &str,
    ) -> Result<(), String> {
        if self.emails.contains(email) {
            return Err("email is already exist".into());
        }

        let now_date_time: DateTime<Utc> = DateTime::from_utc(
            NaiveDateTime::from_timestamp_opt(CURR_TIMESTAMP, 0).unwrap(),
            Utc,
        );
        let birth_date_time = DateTime::from_utc(
            NaiveDateTime::from_timestamp_opt(birth_date.into(), 0).unwrap(),
            Utc,
        );

        let user = model::User {
            email: String::from(email),
            first_name: self.first_names.put(String::from(first_name)),
            last_name: self.last_names.put(String::from(last_name)),
            birth_date,
            age: now_date_time.years_since(birth_date_time).unwrap() as u8,
            gender: gender.into(),
            visits: Vec::new(),
        };

        if matches!(user.gender, model::Gender::None) {
            return Err("gender is unknown".into());
        }

        if self.users.len() == id {
            self.users.push(model::User::default());
        }

        self.users[id] = user;
        self.emails.insert(email.to_string());

        Ok(())
    }

    pub fn store_visit(&mut self, id: u32, user: u32, location: u32, visited_at: i32, mark: u8) {
        if self.visits.len() == id as usize {
            self.visits.push(model::Visit::default());
        }

        self.visits[id as usize] = model::Visit {
            user,
            location,
            visited_at,
            mark,
        };

        let user = &mut self.users[user as usize];

        let user_visit = model::UserVisit {
            id,
            visited_at,
            location,
        };

        // вставка в сортированный вектор визитов пользователя
        let user_visit_idx = user.visits.partition_point(|x| x.visited_at < visited_at);
        user.visits.insert(user_visit_idx, user_visit);

        let location_visit = model::LocationVisit {
            visit_id: id,
            visited_at
        };

        let location = &mut self.locations[location as usize];

        // вставка в сортированный вектор визитов локации
        let location_visit_idx = location.visits.partition_point(|x| x.visited_at < visited_at);
        location.visits.insert(location_visit_idx, location_visit)
    }

    pub fn store_location(
        &mut self,
        id: usize,
        country: &str,
        city: &str,
        place: &str,
        distance: u32,
    ) {
        if self.locations.len() == id as usize {
            self.locations.push(model::Location::default());
        }

        self.locations[id] = model::Location {
            country: self.countries.put(String::from(country)),
            city: self.cities.put(String::from(city)),
            place: self.places.put(String::from(place)),
            distance,
            visits: Vec::new(),
        };
    }
}
