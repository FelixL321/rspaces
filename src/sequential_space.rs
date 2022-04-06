use crate::space::Query;
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
    fn get(&mut self, query: Query) -> Option<Tuple> {
        if let Some(index) = self.v.iter().position(|t| query.query(t)){
            Some(self.v.swap_remove(index))
        }else{
            None
        }
    }
    fn put(&mut self, tuple: Tuple) {
        self.v.push(tuple);
    }
}
