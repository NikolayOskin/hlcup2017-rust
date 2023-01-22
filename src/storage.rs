use std::usize;

use crate::dict::Dict;

pub struct Storage {
    pub users: Vec<User>,
    pub visits: Vec<Visit>,
    pub locations: Vec<Location>,

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

    pub fn add_user(
        &mut self,
        id: usize,
        email: &str,
        first_name: &str,
        last_name: &str,
        birth_date: i32,
        gender: &str,
    ) {
        self.users[id] = User {
            email: String::from(email),
            first_name: self.first_names.put(String::from(first_name)),
            last_name: self.last_names.put(String::from(last_name)),
            birth_date,
            gender: gender.into(),
            visits: Vec::new(),
        };
    }

    pub fn add_visit(&mut self, id: usize, user: u32, location: u32, visited_at: i32, mark: u8) {
        self.visits[id] = Visit {
            user,
            location,
            visited_at,
            mark,
        };

        let user = &mut self.users[user as usize];
        let location = &self.locations[location as usize];

        let user_visit = UserVisit {
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
        self.locations[id] = Location {
            country: self.countries.put(String::from(country)),
            city: self.cities.put(String::from(city)),
            place: self.places.put(String::from(place)),
            distance,
        };
    }

    // pub fn shrink_users(&mut self) {
    //     self.users.truncate(self.users_total as usize);
    //     self.users.shrink_to_fit();
    // }

    // pub fn shrink_visits(&mut self) {
    //     self.visits.truncate(self.visits_total as usize);
    //     self.visits.shrink_to_fit();
    // }

    // pub fn shrink_locations(&mut self) {
    //     self.locations.truncate(self.locations_total as usize);
    //     self.locations.shrink_to_fit();
    // }
}

#[derive(Default, Debug)]
pub enum Gender {
    #[default]
    None,
    Male,
    Female,
}

impl From<&str> for Gender {
    fn from(s: &str) -> Self {
        match s {
            "m" => Gender::Male,
            "f" => Gender::Female,
            _ => Gender::None,
        }
    }
}

impl ToString for Gender {
    fn to_string(&self) -> String {
        match self {
            Gender::Male => String::from("m"),
            Gender::Female => String::from("f"),
            _ => String::from(""),
        }
    }
}

#[derive(Default)]
pub struct User {
    pub email: String,
    pub first_name: u32,
    pub last_name: u32,
    pub birth_date: i32,
    pub gender: Gender,
    pub visits: Vec<UserVisit> // отсортирован по visited_at
}

#[derive(Default)]
pub struct Visit {
    pub user: u32,
    pub location: u32,
    pub mark: u8,
    pub visited_at: i32,
}

pub struct UserVisit {
    pub id: usize,
    pub country: u32,
    pub distance: u32,
    pub visited_at: i32,
}

#[derive(Default)]
pub struct Location {
    pub country: u32,
    pub city: u32,
    pub place: u32,
    pub distance: u32,
}
