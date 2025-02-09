// src/storage/mod.rs

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Storage {
    state: Arc<Mutex<HashMap<String, Vec<u8>>>>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            state: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let state = self.state.lock().unwrap();
        state.get(key).cloned()
    }

    pub fn set(&self, key: &str, value: Vec<u8>) {
        let mut state = self.state.lock().unwrap();
        state.insert(key.to_string(), value);
    }
}
