use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use crate::{gate::Gate, Space};

pub struct Repository {
    spaces: Mutex<HashMap<String, Arc<Space>>>,
    gates: Mutex<HashMap<String, Arc<Gate>>>,
}

impl Repository {
    pub fn new() -> Repository {
        Repository {
            spaces: Mutex::new(HashMap::new()),
            gates: Mutex::new(HashMap::new()),
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
    pub fn new_gate(self, name: String, addr: SocketAddr) -> std::io::Result<()> {
        let arcself = Arc::new(self);
        let clone = Arc::clone(&arcself);
        let mut gates = (*arcself).gates.lock().unwrap();
        match Gate::new_gate(addr, clone) {
            Ok(gate) => {
                gates.insert(name, gate);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
