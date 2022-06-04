use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;

use crate::space::Query;
use crate::space::Space;
use crate::space::Tuple;

pub struct SequentialSpace<'a> {
    v: Mutex<Vec<Tuple<'a>>>,
    listeners: Mutex<Vec<Sender<bool>>>,
}

impl<'a> SequentialSpace<'a> {
    pub fn new() -> SequentialSpace<'a> {
        SequentialSpace {
            v: Mutex::new(Vec::new()),
            listeners: Mutex::new(Vec::new()),
        }
    }
}

impl<'a> Space<'a> for SequentialSpace<'a> {
    fn get(&self, query: Query<'a>) -> Option<Tuple<'a>> {
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
    fn getp(&self, query: &Query<'a>) -> Option<Tuple<'a>> {
        let mut v = self.v.lock().unwrap();
        if let Some(index) = v.iter().position(|t| query.query(t)) {
            Some(v.swap_remove(index))
        } else {
            None
        }
    }
    fn put(&self, tuple: Tuple<'a>) {
        let mut v = self.v.lock().unwrap();
        v.push(tuple);
        let mut l = self.listeners.lock().unwrap();
        for tx in l.iter() {
            tx.send(true);
        }
    }
}
