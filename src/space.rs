use std::{any::Any, marker::PhantomData};

pub trait Space {
    fn get(&self, query: &Query) -> Option<Tuple>;
    fn getp(&self, query: &Query) -> Option<Tuple>;
    fn query(&self, query: &Query) -> Option<Tuple>;
    fn queryp(&self, query: &Query) -> Option<Tuple>;
    fn put(&self, tuple: Tuple);
}

#[derive(Clone)]
pub struct Tuple {
    fields: Vec<Box<dyn TupleField>>,
}

impl Clone for Box<dyn TupleField> {
    fn clone(&self) -> Box<dyn TupleField> {
        self.box_clone()
    }
}

impl Tuple {
    pub fn new(fields: Vec<Box<dyn TupleField>>) -> Tuple {
        Tuple { fields }
    }
    pub fn get_field<T: 'static>(&self, index: usize) -> Option<&T> {
        let b = (*(*self.fields.get(index)?)).as_any().downcast_ref::<T>();
        return b;
    }
    pub fn size(&self) -> usize {
        self.fields.len()
    }
}

pub trait QueryField {
    fn query(&self, element: &Box<dyn TupleField>) -> bool;
}

pub struct Query {
    pub fields: Vec<Box<dyn QueryField>>,
}

impl Query {
    pub fn new() -> Query {
        Query { fields: Vec::new() }
    }
}

impl Query {
    pub fn query(&self, tuple: &Tuple) -> bool {
        let mut res = false;
        for (q, e) in self.fields.iter().zip(tuple.fields.iter()) {
            res = q.query(e);
        }
        res
    }
}

pub trait Queries: Sized + PartialEq {
    fn formal() -> Box<FormalField<Self>> {
        Box::new(FormalField { data: PhantomData })
    }
    fn actual(self) -> Box<ActualField<Self>> {
        Box::new(ActualField { data: self })
    }
}
impl<T: PartialEq> Queries for T {}

pub struct FormalField<T> {
    data: PhantomData<T>,
}

impl<'a, T: 'static> QueryField for FormalField<T> {
    fn query(&self, element: &Box<dyn TupleField + 'a>) -> bool {
        match (**element).as_any().downcast_ref::<T>() {
            None => false,
            Some(_) => true,
        }
    }
}

pub struct ActualField<T: PartialEq> {
    data: T,
}

impl<'a, T: PartialEq + 'static> QueryField for ActualField<T> {
    fn query(&self, element: &Box<dyn TupleField + 'a>) -> bool {
        match (**element).as_any().downcast_ref::<T>() {
            None => false,
            Some(a) => {
                if *a == self.data {
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl<T: Any + Send + Sync + Clone> TupleField for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_clone(&self) -> Box<dyn TupleField> {
        Box::new((*self).clone())
    }
}

pub trait TupleField: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn box_clone(&self) -> Box<dyn TupleField>;
}
