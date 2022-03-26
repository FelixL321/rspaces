use crate::space::Space;
use crate::space::Tuple;

pub struct SequentialSpace {
    v: Vec<Tuple>,
}

impl SequentialSpace {
    pub fn new() -> SequentialSpace {
        SequentialSpace { v: Vec::new() }
    }
}

impl Space for SequentialSpace {
    fn get(&mut self) -> Tuple {
        self.v.pop().unwrap()
    }
    fn put(&mut self, tuple: Tuple) {
        self.v.push(tuple);
    }
}
