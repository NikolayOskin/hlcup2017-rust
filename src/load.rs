use serde::{Deserialize, Serialize};
use std::{error::Error, fs, path::Path};
use zip::ZipArchive;

use crate::storage::{Storage, User, Visit, Location};

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
struct UsersDataJSON {
    users: Vec<UserJSON>,
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
struct VisitsDataJSON {
    visits: Vec<VisitJSON>,
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
struct LocationsDataJSON {
    locations: Vec<LocationJSON>,
}

pub fn run(data_dir: &str, zip_file: &str, storage: &mut Storage) -> Result<(), Box<dyn Error>> {
    extract_json_files(zip_file, data_dir)?;

    let locations_cnt = count_locations(data_dir)?;
    let users_cnt = count_users(data_dir)?;
    let visits_cnt = count_visits(data_dir)?;

    store_locations(data_dir, storage, locations_cnt)?;
    store_users(data_dir, storage, users_cnt)?;
    store_visits(data_dir, storage, visits_cnt)?;

    Ok(())
}

fn extract_json_files(zip_file: &str, dest: &str) -> Result<(), Box<dyn Error>> {
    let exist = Path::new(dest).exists();
    if !exist {
        fs::create_dir_all(dest)?;
    }

    let file = fs::File::open(zip_file)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        if file.name().ends_with("json") {
            let outpath = Path::new(dest).join(file.mangled_name());
            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

fn store_users(data_dir: &str, storage: &mut Storage, users_count: u32) -> Result<(), Box<dyn Error>> {
    storage.users.resize_with(users_count as usize +1, User::default);

    let files = fs::read_dir(data_dir)?;

    for file in files {
        let path = file?.path();

        let extens = path.extension().ok_or("extension error")?.to_str();

        if extens == Some("json") {
            let f_name = path.to_string_lossy();
            if f_name.contains(&"users") {
                let data = fs::read_to_string(path.as_os_str())?;
                let users_file_data: UsersDataJSON = serde_json::from_str(&data)?;

                for user in users_file_data.users {
                    storage.add_user(
                        user.id as usize,
                        user.email.as_str(),
                        user.first_name.as_str(),
                        user.last_name.as_str(),
                        user.birth_date,
                        user.gender.as_str(),
                    );
                }
            }
        }
    }

    //storage.shrink_users();

    Ok(())
}

pub fn count_users(data_dir: &str) -> Result<u32, Box<dyn Error>> {
    let files = fs::read_dir(data_dir)?;
    let mut cnt: u32 = 0;

    for file in files {
        let path = file?.path();

        let extens = path.extension().ok_or("extension error")?.to_str();

        if extens == Some("json") {
            let f_name = path.to_string_lossy();
            if f_name.contains(&"users") {
                let data = fs::read_to_string(path.as_os_str())?;
                let users_file_data: UsersDataJSON = serde_json::from_str(&data)?;
                cnt = cnt + users_file_data.users.len() as u32;
            }
        }
    }

    Ok(cnt)
}

fn store_visits(data_dir: &str, storage: &mut Storage, visits_count: u32) -> Result<(), Box<dyn Error>> {
    storage.visits.resize_with(visits_count as usize +1, Visit::default);

    let files = fs::read_dir(data_dir)?;

    for file in files {
        let path = file?.path();

        let extens = path.extension().ok_or("extension error")?.to_str();

        if extens == Some("json") {
            let f_name = path.to_string_lossy();
            if f_name.contains(&"visits") {
                let data = fs::read_to_string(path.as_os_str())?;
                let visits_file_data: VisitsDataJSON = serde_json::from_str(&data)?;

                for visit in visits_file_data.visits {
                    storage.add_visit(
                        visit.id as usize,
                        visit.user,
                        visit.location,
                        visit.visited_at,
                        visit.mark,
                    );
                }
            }
        }
    }

    //storage.shrink_visits();

    Ok(())
}

pub fn count_visits(data_dir: &str) -> Result<u32, Box<dyn Error>> {
    let files = fs::read_dir(data_dir)?;
    let mut cnt: u32 = 0;

    for file in files {
        let path = file?.path();

        let extens = path.extension().ok_or("extension error")?.to_str();

        if extens == Some("json") {
            let f_name = path.to_string_lossy();
            if f_name.contains(&"visits") {
                let data = fs::read_to_string(path.as_os_str())?;
                let visits_file_data: VisitsDataJSON = serde_json::from_str(&data)?;
                cnt = cnt + visits_file_data.visits.len() as u32;
            }
        }
    }

    Ok(cnt)
}

fn store_locations(data_dir: &str, storage: &mut Storage, locations_count: u32) -> Result<(), Box<dyn Error>> {
    storage.locations.resize_with(locations_count as usize +1, Location::default);

    let files = fs::read_dir(data_dir)?;

    for file in files {
        let path = file?.path();

        let extens = path.extension().ok_or("extension error")?.to_str();

        if extens == Some("json") {
            let f_name = path.to_string_lossy();
            if f_name.contains(&"locations") {
                let data = fs::read_to_string(path.as_os_str())?;
                let locations_file_data: LocationsDataJSON = serde_json::from_str(&data)?;

                for location in locations_file_data.locations {
                    storage.store_location(
                        location.id as usize,
                        &location.country,
                        &location.city,
                        &location.place,
                        location.distance,
                    );
                }
            }
        }
    }

    //storage.shrink_locations();

    Ok(())
}

pub fn count_locations(data_dir: &str) -> Result<u32, Box<dyn Error>> {
    let files = fs::read_dir(data_dir)?;
    let mut cnt: u32 = 0;

    for file in files {
        let path = file?.path();

        let extens = path.extension().ok_or("extension error")?.to_str();

        if extens == Some("json") {
            let f_name = path.to_string_lossy();
            if f_name.contains(&"locations") {
                let data = fs::read_to_string(path.as_os_str())?;
                let locations_file_data: LocationsDataJSON = serde_json::from_str(&data)?;
                cnt = cnt + locations_file_data.locations.len() as u32;
            }
        }
    }

    Ok(cnt)
}