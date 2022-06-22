use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{gate::Gate, Space};

pub struct Repository {
    spaces: Mutex<HashMap<String, Arc<dyn Space>>>,
    gates: Mutex<HashMap<String, Arc<Gate>>>,
}

impl Repository {
    /// Creates new repository
    pub fn new() -> Repository {
        Repository {
            spaces: Mutex::new(HashMap::new()),
            gates: Mutex::new(HashMap::new()),
        }
    }
    pub fn add_space<T: Space + 'static>(&self, name: String, space: Arc<T>) {
        let mut s = self.spaces.lock().unwrap();
        s.insert(name, space);
    }
    pub fn get_space(&self, name: String) -> Option<Arc<dyn Space>> {
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
    /// Adds a new gate for a repository
    ///
    /// # Arguments
    /// First argument is a Arc reference to the repository for which a gate should be opened
    ///
    /// Second argument is the gates identifier for this repo
    ///
    /// Third argument is a connection string used to bind to a specific socket address.
    ///
    ///
    /// # Example
    /// ```
    /// # use rspaces::*
    /////Create new repo
    ///let repo = Arc::new(Repository::new());
    ///
    /////Add gate to the repository running on localhost port 3800
    ///Repository::add_gate(
    ///    Arc::clone(&repo),
    ///    String::from("gate1"),
    ///    "127.0.0.1:3800".to_string(),
    ///)
    /// ```
    pub fn add_gate(repo: Arc<Repository>, name: String, addr: String) -> std::io::Result<()> {
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
        let gates = self.gates.lock().unwrap();
        let gate = match gates.get(&name) {
            Some(gate) => gate,
            None => return,
        };
        let sender = gate.handle.lock().unwrap();
        sender.send(()).unwrap();
        let mut handle = gate.join.lock().unwrap();
        let _ = handle.take().unwrap().join();
    }
}
