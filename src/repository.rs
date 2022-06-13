use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::Space;

pub struct Repository {
    spaces: Mutex<HashMap<String, Arc<Space>>>,
}

impl Repository {
    pub fn new() -> Repository {
        Repository {
            spaces: Mutex::new(HashMap::new()),
        }
    }
    pub fn add_space(&self, name: String, space: Arc<Space>) {
        let mut s = self.spaces.lock().unwrap();
        s.insert(name, space);
    }
    pub fn get_space(&self, name: String) -> Option<Arc<Space>> {
        let s = self.spaces.lock().unwrap();
        match s.get(&name) {
            Some(s) => Some(Arc::clone(s)),
            None => None,
        }
    }
    pub fn del_space(&self, name: String) {
        let mut s = self.spaces.lock().unwrap();
        s.remove_entry(&name);
    }
}
