use serde::{Deserialize, Serialize};
use typetag::serde;

use crate::{Tuple, TupleField};

#[typetag::serde(tag = "Template")]
pub trait TemplateField {
    fn query(&self, element: &Box<dyn TupleField>, matching: &TemplateType) -> bool;
}

#[derive(Serialize, Deserialize, Clone)]
pub enum TemplateType {
    Actual,
    Formal,
}

#[derive(Serialize, Deserialize, Clone)]
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
