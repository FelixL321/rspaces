use std::{any::Any, marker::PhantomData};

pub trait Space {
    fn get(&mut self, query: Query) -> Option<Tuple>;
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

pub trait QueryField{
    fn query(&self, element: &Box<dyn Any>) -> bool;
}

pub struct Query{
    pub fields: Vec<Box<dyn QueryField>>,
}

impl Query{
    pub fn new() -> Query{
        Query { fields: Vec::new() }
    }
}

impl Query{
    pub fn query(&self, tuple: &Tuple) -> bool{
        let mut res = false;
        for (q, e) in self.fields.iter().zip(tuple.fields.iter()){
            res = q.query(e);
        }
        res
    }
}

pub trait Queries : Sized + PartialEq{
    fn formal() -> Box<FormalField<Self>>{
        Box::new(FormalField { data: PhantomData })
    }
    fn actual(self) -> Box<ActualField<Self>>{
        Box::new(ActualField{ data: self})
    }
}
impl<T: PartialEq> Queries for T{}

pub struct FormalField<T>{
    data: PhantomData<T>,
}

impl<T: 'static> QueryField for FormalField<T>{
    fn query(&self, element: &Box<dyn Any>) -> bool{
        match element.downcast_ref::<T>() {
            None => false,
            Some(_) => true,
        }
    }
}

pub struct ActualField<T: PartialEq>{
    data: T,
}

impl<T: PartialEq + 'static> QueryField for ActualField<T>{
    fn query(&self, element: &Box<dyn Any>) -> bool{
        match element.downcast_ref::<T>() {
            None => false,
            Some(a) => {
                if *a == self.data {
                    true
                }else{
                    false
                }
            },
        }
    }
}