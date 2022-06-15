use std::{
    collections::HashMap,
    io::Repeat,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use crate::{gate::Gate, LocalSpace};

pub struct Repository {
    spaces: Mutex<HashMap<String, Arc<LocalSpace>>>,
    gates: Mutex<HashMap<String, Arc<Gate>>>,
}

impl Repository {
    pub fn new() -> Repository {
        Repository {
            spaces: Mutex::new(HashMap::new()),
            gates: Mutex::new(HashMap::new()),
        }
    }
    pub fn add_space(&self, name: String, space: Arc<LocalSpace>) {
        let mut s = self.spaces.lock().unwrap();
        s.insert(name, space);
    }
    pub fn get_space(&self, name: String) -> Option<Arc<LocalSpace>> {
        let s = self.spaces.lock().unwrap();
        println!("get space name: {}", name);
        for key in s.keys() {
            println!("spaces: ({})", key);
        }
        match s.get(&name) {
            Some(s) => Some(Arc::clone(s)),
            None => None,
        }
    }
    pub fn del_space(&self, name: String) {
        let mut s = self.spaces.lock().unwrap();
        s.remove_entry(&name);
    }
    pub fn add_gate(repo: Arc<Repository>, name: String, addr: SocketAddr) -> std::io::Result<()> {
        let clone = Arc::clone(&repo);
        let mut gates = repo.gates.lock().unwrap();
        match Gate::new_gate(addr, clone) {
            Ok(gate) => {
                gates.insert(name, gate);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn close_gate(&self, name: String) {
        let mut gates = self.gates.lock().unwrap();
        let gate = gates.get(&name).unwrap();
        let sender = gate.handle.lock().unwrap();
        sender.send(()).unwrap();
    }
}
