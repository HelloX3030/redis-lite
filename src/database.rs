use std::collections::HashMap;
use crate::store::Store;

pub struct Database {
    map: HashMap::<String, String>,
}

impl Database {
    pub fn new() -> Database {
        Database { map: HashMap::new() }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn get(&mut self, key: &str) -> Option<&mut String> {
        self.map.get_mut(key)
    }

    pub fn print(&self) {
        for (key, value) in self.map.iter(){
            println!("[{}]: {}", key, value);
        }
    }

    pub fn delete(&mut self, key: &str) {
        self.map.remove(key);
    }
}

impl Store for Database {
    fn store(&self) {
        println!("Store Called!");
    }
}
