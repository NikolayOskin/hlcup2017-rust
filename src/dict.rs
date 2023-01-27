use std::collections::HashMap;

// dictionary
// for storing repeatable entities strings such as: countries, cities, firstnames etc.
#[derive(Clone)]
pub struct Dict {
    pub map: HashMap<String, u32>,
    vec: Vec<String>,
}

impl Dict {
    pub fn new() -> Self {
        Dict {
            map: HashMap::new(),
            vec: Vec::new(),
        }
    }

    // returns id of entry if entry exists
    // otherwise creates an entry and returns its id
    pub fn put(&mut self, key: String) -> u32 {
        let val = self.map.get(&key);
        if val.is_some() {
            return val.unwrap().clone();
        }

        self.vec.push(key.clone());
        self.map.insert(key, self.vec.len() as u32 - 1);

        return self.vec.len() as u32 - 1;
    }

    pub fn get_by_idx(&self, idx: usize) -> String {
        self.vec[idx].to_string()
    }

    pub fn exist(&self, s: &str) -> bool {
        let val = self.map.get(s);
        if val.is_some() {
            return true;
        }
        return false;
    }
}
