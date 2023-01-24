use std::collections::HashMap;

// словарь
// для экономии памяти на хранение повторяющихся названий сущностей
// например, названия стран и городов, имен, фамилий и т.д.
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

    // возвращает id записи, если запись в словаре существует,
    // в противном случае создает и возвращает id созданной записи
    pub fn put(&mut self, key: String) -> u32 {
        let val = self.map.get(&key);
        if val.is_some() {
            return val.unwrap().clone();
        }

        self.map.insert(key.clone(), self.vec.len() as u32);
        self.vec.push(key);

        return self.vec.len() as u32;
    }

    pub fn get_by_idx(&self, idx: usize) -> String {
        self.vec[idx].clone()
    }

    pub fn exist(&self, s: &str) -> bool {
        let val = self.map.get(s);
        if val.is_some() {
            return true;
        }
        return false;
    }
}
