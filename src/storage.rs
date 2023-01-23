use std::usize;

use crate::{dict::Dict, model};

pub struct Storage {
    pub users: Vec<model::User>,
    pub visits: Vec<model::Visit>,
    pub locations: Vec<model::Location>,

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
    ) {
        self.users[id] = model::User {
            email: String::from(email),
            first_name: self.first_names.put(String::from(first_name)),
            last_name: self.last_names.put(String::from(last_name)),
            birth_date,
            gender: gender.into(),
            visits: Vec::new(),
        };
    }

    pub fn store_visit(&mut self, id: usize, user: u32, location: u32, visited_at: i32, mark: u8) {
        self.visits[id] = model::Visit {
            user,
            location,
            visited_at,
            mark,
        };

        let user = &mut self.users[user as usize];
        let location = &self.locations[location as usize];

        let user_visit = model::UserVisit {
            id,
            visited_at,
            country: location.country,
            distance: location.distance,
        };

        // вставка в сортированный (по visited_at полю) вектор
        let idx = user.visits.partition_point(|x| x.visited_at < visited_at);
        user.visits.insert(idx, user_visit);
    }

    pub fn store_location(
        &mut self,
        id: usize,
        country: &str,
        city: &str,
        place: &str,
        distance: u32,
    ) {
        self.locations[id] = model::Location {
            country: self.countries.put(String::from(country)),
            city: self.cities.put(String::from(city)),
            place: self.places.put(String::from(place)),
            distance,
        };
    }
}
