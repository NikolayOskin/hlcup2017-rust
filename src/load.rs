use std::{error::Error, fs, path::Path, io::Read};
use zip::ZipArchive;

use crate::{model, storage::Storage};

const DATA_FILE: &str = "data.zip";
const OPTIONS_FILE: &str = "options.txt";
const TEMP_DIR: &str = "tmp/data";
const DATA_DIR: &str = "data";

// разархивирует data.zip файл, читает timestamp временеи генерации данных (из options.txt)
// считает кол-во записей по каждой сущности (user, visit, location)
// сохраняет записи в in-memory storage
pub fn run(storage: &mut Storage) -> Result<(), Box<dyn Error>> {
    storage.timestamp = get_timestamp();

    extract_json_files()?;

    // считаем кол-во каждого типа сущности, чтобы преаллоцировать векторы с необходимым capacity
    let locations_cnt = count_locations()?;
    let users_cnt = count_users()?;
    let visits_cnt = count_visits()?;

    store_locations(storage, locations_cnt)?;
    store_users(storage, users_cnt)?;
    store_visits(storage, visits_cnt)?;

    Ok(())
}

fn get_timestamp() -> i64 {
    let mut file = fs::File::open(TEMP_DIR.to_owned() + "/" + OPTIONS_FILE).unwrap();

    let mut buf = "".to_string();

    _ = file.read_to_string(&mut buf).unwrap();

    let timestamp_str = buf.lines().next().unwrap();
    let timestamp: i64 = timestamp_str.parse().unwrap();

    timestamp
}

fn extract_json_files() -> Result<(), Box<dyn Error>> {
    let exist = Path::new(DATA_DIR).exists();
    if !exist {
        fs::create_dir_all(DATA_DIR)?;
    }

    let file = fs::File::open(TEMP_DIR.to_owned() + "/" + DATA_FILE)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        if file.name().ends_with("json") {
            let outpath = Path::new(DATA_DIR).join(file.mangled_name());
            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

fn store_users(storage: &mut Storage, users_count: u32) -> Result<(), Box<dyn Error>> {
    storage
        .users
        .resize_with(users_count as usize + 1, model::User::default);

    let files = fs::read_dir(DATA_DIR)?;

    for file in files {
        let path = file?.path();

        let extens = path.extension().ok_or("extension error")?.to_str();

        if extens == Some("json") {
            let f_name = path.to_string_lossy();
            if f_name.contains(&"users") {
                let data = fs::read_to_string(path.as_os_str())?;
                let users_file_data: model::UsersDataJSON = serde_json::from_str(&data)?;

                for user in users_file_data.users {
                    let _ = storage.store_user(
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

    Ok(())
}

pub fn count_users() -> Result<u32, Box<dyn Error>> {
    let files = fs::read_dir(DATA_DIR)?;
    let mut cnt: u32 = 0;

    for file in files {
        let path = file?.path();

        let extens = path.extension().ok_or("extension error")?.to_str();

        if extens == Some("json") {
            let f_name = path.to_string_lossy();
            if f_name.contains(&"users") {
                let data = fs::read_to_string(path.as_os_str())?;
                let users_file_data: model::UsersDataJSON = serde_json::from_str(&data)?;
                cnt = cnt + users_file_data.users.len() as u32;
            }
        }
    }

    Ok(cnt)
}

fn store_visits(storage: &mut Storage, visits_count: u32) -> Result<(), Box<dyn Error>> {
    storage
        .visits
        .resize_with(visits_count as usize + 1, model::Visit::default);

    let files = fs::read_dir(DATA_DIR)?;

    for file in files {
        let path = file?.path();

        let extens = path.extension().ok_or("extension error")?.to_str();

        if extens == Some("json") {
            let f_name = path.to_string_lossy();
            if f_name.contains(&"visits") {
                let data = fs::read_to_string(path.as_os_str())?;
                let visits_file_data: model::VisitsDataJSON = serde_json::from_str(&data)?;

                for visit in visits_file_data.visits {
                    storage.store_visit(
                        visit.id,
                        visit.user,
                        visit.location,
                        visit.visited_at,
                        visit.mark,
                    );
                }
            }
        }
    }

    Ok(())
}

pub fn count_visits() -> Result<u32, Box<dyn Error>> {
    let files = fs::read_dir(DATA_DIR)?;
    let mut cnt: u32 = 0;

    for file in files {
        let path = file?.path();

        let extens = path.extension().ok_or("extension error")?.to_str();

        if extens == Some("json") {
            let f_name = path.to_string_lossy();
            if f_name.contains(&"visits") {
                let data = fs::read_to_string(path.as_os_str())?;
                let visits_file_data: model::VisitsDataJSON = serde_json::from_str(&data)?;
                cnt = cnt + visits_file_data.visits.len() as u32;
            }
        }
    }

    Ok(cnt)
}

fn store_locations(storage: &mut Storage, locations_count: u32) -> Result<(), Box<dyn Error>> {
    storage
        .locations
        .resize_with(locations_count as usize + 1, model::Location::default);

    let files = fs::read_dir(DATA_DIR)?;

    for file in files {
        let path = file?.path();

        let extens = path.extension().ok_or("extension error")?.to_str();

        if extens == Some("json") {
            let f_name = path.to_string_lossy();
            if f_name.contains(&"locations") {
                let data = fs::read_to_string(path.as_os_str())?;
                let locations_file_data: model::LocationsDataJSON = serde_json::from_str(&data)?;

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

    Ok(())
}

pub fn count_locations() -> Result<u32, Box<dyn Error>> {
    let files = fs::read_dir(DATA_DIR)?;
    let mut cnt: u32 = 0;

    for file in files {
        let path = file?.path();

        let extens = path.extension().ok_or("extension error")?.to_str();

        if extens == Some("json") {
            let f_name = path.to_string_lossy();
            if f_name.contains(&"locations") {
                let data = fs::read_to_string(path.as_os_str())?;
                let locations_file_data: model::LocationsDataJSON = serde_json::from_str(&data)?;
                cnt = cnt + locations_file_data.locations.len() as u32;
            }
        }
    }

    Ok(cnt)
}
