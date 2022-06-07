use std::{any::Any, marker::PhantomData};

pub trait Space<'a> {
    fn get(&self, query: &Query<'a>) -> Option<Tuple<'a>>;
    fn getp(&self, query: &Query<'a>) -> Option<Tuple<'a>>;
    //fn query(&self, query: &Query<'a>) -> Option<Tuple<'a>>;
    //fn queryp(&self, query: &Query<'a>) -> Option<Tuple<'a>>;
    fn put(&self, tuple: Tuple<'a>);
}

pub struct Tuple<'a> {
    fields: Vec<Box<dyn TupleField + 'a>>,
}

impl<'a> Clone for Box<dyn TupleField> {
    fn clone(&self) -> Box<dyn TupleField> {
        Box::new(self.clone())
    }
}

impl<'a> Tuple<'a> {
    pub fn new(fields: Vec<Box<dyn TupleField>>) -> Tuple<'a> {
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

pub trait QueryField<'a> {
    fn query(&self, element: &Box<dyn TupleField + 'a>) -> bool;
}

pub struct Query<'a> {
    pub fields: Vec<Box<dyn QueryField<'a>>>,
}

impl<'a> Query<'a> {
    pub fn new() -> Query<'a> {
        Query { fields: Vec::new() }
    }
}

impl<'a> Query<'a> {
    pub fn query(&self, tuple: &Tuple<'a>) -> bool {
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

impl<'a, T: 'static> QueryField<'a> for FormalField<T> {
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

impl<'a, T: PartialEq + 'static> QueryField<'a> for ActualField<T> {
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

impl<T: Any + Send + Sync> TupleField for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait TupleField: Send + Sync {
    fn as_any(&self) -> &dyn Any;
}
