use std::any::Any;

pub trait Space {
    fn get(&mut self) -> Tuple;
    fn put(&mut self, tuple: Tuple);
}

pub struct Tuple {
    fields: Vec<Box<dyn Any>>,
}

impl Tuple {
    pub fn new(fields: Vec<Box<dyn Any>>) -> Tuple {
        Tuple { fields }
    }
    pub fn get_field<T: 'static>(&self, index: usize) -> Option<&T> {
        self.fields.get(index)?.downcast_ref::<T>()
    }
    pub fn size(&self) -> usize {
        self.fields.len()
    }
}
