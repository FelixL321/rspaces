use serde::{Deserialize, Serialize};
use typetag::serde;

use crate::{Tuple, TupleField};

#[typetag::serde(tag = "Template")]
pub trait TemplateField {
    fn query(&self, element: &Box<dyn TupleField>, matching: &TemplateType) -> bool;
}

#[derive(Serialize, Deserialize)]
pub enum TemplateType {
    Actual,
    Formal,
}

#[derive(Serialize, Deserialize)]
pub struct Template {
    pub fields: Vec<(Box<dyn TupleField>, TemplateType)>,
}

impl Template {
    pub fn new() -> Template {
        Template { fields: Vec::new() }
    }
    pub fn new_fields(fields: Vec<(Box<dyn TupleField>, TemplateType)>) -> Template {
        Template { fields }
    }
}

impl Template {
    pub fn query(&self, tuple: &Tuple) -> bool {
        for (q, e) in self.fields.iter().zip(tuple.fields.iter()) {
            if !q.0.query(e, &q.1) {
                return false;
            }
        }
        true
    }
}

pub trait FieldType: Sized + PartialEq + TupleField + 'static {
    fn formal(self) -> (Box<dyn TupleField>, TemplateType) {
        (Box::new(self), TemplateType::Formal)
    }
    fn actual(self) -> (Box<dyn TupleField>, TemplateType) {
        (Box::new(self), TemplateType::Actual)
    }
}
impl<T: PartialEq + TupleField + 'static> FieldType for T {}
/*
#[derive(Serialize, Deserialize)]
pub struct FormalField<T> {
    data: PhantomData<T>,
}

#[typetag::serde]
impl TemplateField for FormalField<i32> {
    fn query(&self, element: &Box<dyn TupleField>) -> bool {
        match (**element).as_any().downcast_ref::<i32>() {
            None => false,
            Some(_) => true,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ActualField<T: PartialEq> {
    data: T,
}

#[typetag::serde]
impl TemplateField for ActualField<i32> {
    fn query(&self, element: &Box<dyn TupleField>) -> bool {
        match (**element).as_any().downcast_ref::<i32>() {
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

#[typetag::serde]
impl TemplateField for FormalField<char> {
    fn query(&self, element: &Box<dyn TupleField>) -> bool {
        match (**element).as_any().downcast_ref::<char>() {
            None => false,
            Some(_) => true,
        }
    }
}

#[typetag::serde]
impl TemplateField for ActualField<char> {
    fn query(&self, element: &Box<dyn TupleField>) -> bool {
        match (**element).as_any().downcast_ref::<char>() {
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
}*/
