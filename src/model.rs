use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq)]
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
    pub age: u8,
    pub gender: Gender,
    pub visits: Vec<UserVisit>, // sorted by visited_at
}

#[derive(Default)]
pub struct Visit {
    pub user: u32,
    pub location: u32,
    pub mark: u8,
    pub visited_at: i32,
}

pub struct UserVisit {
    pub id: u32,
    pub location: u32,
    pub visited_at: i32,
}

#[derive(Default)]
pub struct Location {
    pub country: u32,
    pub city: u32,
    pub place: u32,
    pub distance: u32,

    // sorted by visited_at
    pub visits: Vec<LocationVisit>,
}

pub struct LocationVisit {
    pub visit_id: u32,
    pub visited_at: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct UserJSON {
    pub id: u32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub birth_date: i32,
}

#[derive(Deserialize, Debug)]
pub struct UsersDataJSON {
    pub users: Vec<UserJSON>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct VisitJSON {
    pub id: u32,
    pub user: u32,
    pub location: u32,
    pub mark: u8,
    pub visited_at: i32,
}

#[derive(Deserialize, Debug)]
pub struct VisitsDataJSON {
    pub visits: Vec<VisitJSON>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct LocationJSON {
    pub id: u32,
    pub distance: u32,
    pub city: String,
    pub country: String,
    pub place: String,
}

#[derive(Deserialize, Debug)]
pub struct LocationsDataJSON {
    pub locations: Vec<LocationJSON>,
}

#[derive(Debug, Deserialize)]
pub struct UserVisitsParams {
    pub fromDate: Option<i32>,
    pub toDate: Option<i32>,
    pub country: Option<String>,
    pub toDistance: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct LocationAvgParams {
    pub fromDate: Option<i32>,
    pub toDate: Option<i32>,
    pub fromAge: Option<u32>,
    pub toAge: Option<u32>,
    pub gender: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserVisitJSON {
    pub mark: u8,
    pub visited_at: i32,
    pub place: String,
}

#[derive(Debug, Serialize)]
pub struct UserVisitsJSON {
    pub visits: Vec<UserVisitJSON>,
}

#[derive(Debug, Serialize)]
pub struct LocationAverageJSON {
    pub avg: f64,
}
