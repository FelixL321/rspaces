use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;

use crate::space::Query;
use crate::space::Space;
use crate::space::Tuple;

pub struct SequentialSpace {
    v: Mutex<Vec<Tuple>>,
    listeners: Mutex<Vec<Sender<bool>>>,
}

impl SequentialSpace {
    pub fn new() -> SequentialSpace {
        SequentialSpace {
            v: Mutex::new(Vec::new()),
            listeners: Mutex::new(Vec::new()),
        }
    }
}

impl Space for SequentialSpace {
    fn get(&self, query: &Query) -> Option<Tuple> {
        loop {
            match self.getp(&query) {
                Some(t) => return Some(t),
                None => {
                    let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
                    {
                        let mut l = self.listeners.lock().unwrap();
                        l.push(tx);
                    }
                    let _ = rx.recv();
                }
            };
        }
    }
    fn getp(&self, query: &Query) -> Option<Tuple> {
        let mut v = self.v.lock().unwrap();
        if let Some(index) = v.iter().position(|t| query.query(t)) {
            Some(v.swap_remove(index))
        } else {
            None
        }
    }
    fn put(&self, tuple: Tuple) {
        let mut v = self.v.lock().unwrap();
        v.push(tuple);
        let mut l = self.listeners.lock().unwrap();
        for tx in l.iter() {
            tx.send(true);
        }
    }
    fn queryp(&self, query: &Query) -> Option<Tuple> {
        let v = self.v.lock().unwrap();
        if let Some(index) = v.iter().position(|t| query.query(t)) {
            let ret = (*v.get(index).unwrap()).clone();
            Some(ret)
        } else {
            None
        }
    }
    fn query(&self, query: &Query) -> Option<Tuple> {
        loop {
            match self.queryp(&query) {
                Some(t) => return Some(t),
                None => {
                    let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
                    {
                        let mut l = self.listeners.lock().unwrap();
                        l.push(tx);
                    }
                    let _ = rx.recv();
                }
            };
        }
    }
}
