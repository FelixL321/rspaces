use std::any::Any;

use serde::{Deserialize, Serialize};

use crate::TemplateType;

#[derive(Clone, Serialize, Deserialize)]
pub struct Tuple {
    pub fields: Vec<Box<dyn TupleField>>,
}

impl Clone for Box<dyn TupleField> {
    fn clone(&self) -> Box<dyn TupleField> {
        (**self).box_clone()
    }
}

impl Tuple {
    /**
    Creates a new tuple from a vector of boxes of tuplefields.

    # Example
    ```
    # use rspaces::*;
    # let space = Space::new_sequential();
    let a = 5;
    let b = 'b';
    let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
    let tuple = Tuple::new(fields);
    ```
    Alternatively a macro can be used to create the tuple and put it into a space directly
    ```
    # use rspaces::*;
    # let space = Space::new_sequential();
    let a = 5;
    let b = 'b';
    space_put!(space, (a, b));
    ```
     */
    pub fn new(fields: Vec<Box<dyn TupleField>>) -> Tuple {
        Tuple { fields }
    }
    /// Get the value from a field of a tuple. Need to be passed the expected datatype in order for a cast.
    /// Will return None if the expected type is not equal to the actual. Othwerwise it will return Some(value)
    ///
    /// # Example
    /// ```
    /// # use rspaces::*;
    /// # let space = Space::new_sequential();
    /// //Create tuple
    /// let a = 5;
    /// let b = 'b';
    /// let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
    /// let tuple = Tuple::new(fields);
    ///
    /// //Get tuple
    /// let five = *tuple.get_field::<i32>(0).unwrap();
    /// let charb = *tuple.get_field::<char>(1).unwrap();
    ///
    /// assert_eq!(5, five);
    /// assert_eq!('b', charb);
    /// ```
    pub fn get_field<T: 'static>(&self, index: usize) -> Option<&T> {
        let b = (*(*self.fields.get(index)?)).as_any().downcast_ref::<T>();
        return b;
    }
    pub fn size(&self) -> usize {
        self.fields.len()
    }
}

#[typetag::serde(tag = "field")]
pub trait TupleField: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn box_clone(&self) -> Box<dyn TupleField>;
    fn query(&self, element: &Box<dyn TupleField>, matching: &TemplateType) -> bool;
}

//Impl blocks as serde typetag wont allow for generic
#[typetag::serde]
impl TupleField for i32 {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_clone(&self) -> Box<dyn TupleField> {
        Box::new((*self).clone())
    }
    fn query(&self, element: &Box<dyn TupleField>, matching: &TemplateType) -> bool {
        match matching {
            TemplateType::Actual => match (*element).as_any().downcast_ref::<Self>() {
                Some(e) => *self == *e,
                None => false,
            },
            TemplateType::Formal => match (*element).as_any().downcast_ref::<Self>() {
                Some(_) => true,
                None => false,
            },
        }
    }
}

#[typetag::serde]
impl TupleField for i64 {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_clone(&self) -> Box<dyn TupleField> {
        Box::new((*self).clone())
    }
    fn query(&self, element: &Box<dyn TupleField>, matching: &TemplateType) -> bool {
        match matching {
            TemplateType::Actual => match (*element).as_any().downcast_ref::<Self>() {
                Some(e) => *self == *e,
                None => false,
            },
            TemplateType::Formal => match (*element).as_any().downcast_ref::<Self>() {
                Some(_) => true,
                None => false,
            },
        }
    }
}
#[typetag::serde]
impl TupleField for i128 {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_clone(&self) -> Box<dyn TupleField> {
        Box::new((*self).clone())
    }
    fn query(&self, element: &Box<dyn TupleField>, matching: &TemplateType) -> bool {
        match matching {
            TemplateType::Actual => match (*element).as_any().downcast_ref::<Self>() {
                Some(e) => *self == *e,
                None => false,
            },
            TemplateType::Formal => match (*element).as_any().downcast_ref::<Self>() {
                Some(_) => true,
                None => false,
            },
        }
    }
}
#[typetag::serde]
impl TupleField for i8 {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_clone(&self) -> Box<dyn TupleField> {
        Box::new((*self).clone())
    }
    fn query(&self, element: &Box<dyn TupleField>, matching: &TemplateType) -> bool {
        match matching {
            TemplateType::Actual => match (*element).as_any().downcast_ref::<Self>() {
                Some(e) => *self == *e,
                None => false,
            },
            TemplateType::Formal => match (*element).as_any().downcast_ref::<Self>() {
                Some(_) => true,
                None => false,
            },
        }
    }
}
#[typetag::serde]
impl TupleField for i16 {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_clone(&self) -> Box<dyn TupleField> {
        Box::new((*self).clone())
    }
    fn query(&self, element: &Box<dyn TupleField>, matching: &TemplateType) -> bool {
        match matching {
            TemplateType::Actual => match (*element).as_any().downcast_ref::<Self>() {
                Some(e) => *self == *e,
                None => false,
            },
            TemplateType::Formal => match (*element).as_any().downcast_ref::<Self>() {
                Some(_) => true,
                None => false,
            },
        }
    }
}

#[typetag::serde]
impl TupleField for u64 {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_clone(&self) -> Box<dyn TupleField> {
        Box::new((*self).clone())
    }
    fn query(&self, element: &Box<dyn TupleField>, matching: &TemplateType) -> bool {
        match matching {
            TemplateType::Actual => match (*element).as_any().downcast_ref::<Self>() {
                Some(e) => *self == *e,
                None => false,
            },
            TemplateType::Formal => match (*element).as_any().downcast_ref::<Self>() {
                Some(_) => true,
                None => false,
            },
        }
    }
}
#[typetag::serde]
impl TupleField for u128 {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_clone(&self) -> Box<dyn TupleField> {
        Box::new((*self).clone())
    }
    fn query(&self, element: &Box<dyn TupleField>, matching: &TemplateType) -> bool {
        match matching {
            TemplateType::Actual => match (*element).as_any().downcast_ref::<Self>() {
                Some(e) => *self == *e,
                None => false,
            },
            TemplateType::Formal => match (*element).as_any().downcast_ref::<Self>() {
                Some(_) => true,
                None => false,
            },
        }
    }
}
#[typetag::serde]
impl TupleField for u8 {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_clone(&self) -> Box<dyn TupleField> {
        Box::new((*self).clone())
    }
    fn query(&self, element: &Box<dyn TupleField>, matching: &TemplateType) -> bool {
        match matching {
            TemplateType::Actual => match (*element).as_any().downcast_ref::<Self>() {
                Some(e) => *self == *e,
                None => false,
            },
            TemplateType::Formal => match (*element).as_any().downcast_ref::<Self>() {
                Some(_) => true,
                None => false,
            },
        }
    }
}
#[typetag::serde]
impl TupleField for u16 {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_clone(&self) -> Box<dyn TupleField> {
        Box::new((*self).clone())
    }
    fn query(&self, element: &Box<dyn TupleField>, matching: &TemplateType) -> bool {
        match matching {
            TemplateType::Actual => match (*element).as_any().downcast_ref::<Self>() {
                Some(e) => *self == *e,
                None => false,
            },
            TemplateType::Formal => match (*element).as_any().downcast_ref::<Self>() {
                Some(_) => true,
                None => false,
            },
        }
    }
}

#[typetag::serde]
impl TupleField for usize {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_clone(&self) -> Box<dyn TupleField> {
        Box::new((*self).clone())
    }
    fn query(&self, element: &Box<dyn TupleField>, matching: &TemplateType) -> bool {
        match matching {
            TemplateType::Actual => match (*element).as_any().downcast_ref::<Self>() {
                Some(e) => *self == *e,
                None => false,
            },
            TemplateType::Formal => match (*element).as_any().downcast_ref::<Self>() {
                Some(_) => true,
                None => false,
            },
        }
    }
}

#[typetag::serde]
impl TupleField for isize {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_clone(&self) -> Box<dyn TupleField> {
        Box::new((*self).clone())
    }
    fn query(&self, element: &Box<dyn TupleField>, matching: &TemplateType) -> bool {
        match matching {
            TemplateType::Actual => match (*element).as_any().downcast_ref::<Self>() {
                Some(e) => *self == *e,
                None => false,
            },
            TemplateType::Formal => match (*element).as_any().downcast_ref::<Self>() {
                Some(_) => true,
                None => false,
            },
        }
    }
}
#[typetag::serde]
impl TupleField for f32 {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_clone(&self) -> Box<dyn TupleField> {
        Box::new((*self).clone())
    }
    fn query(&self, element: &Box<dyn TupleField>, matching: &TemplateType) -> bool {
        match matching {
            TemplateType::Actual => match (*element).as_any().downcast_ref::<Self>() {
                Some(e) => *self == *e,
                None => false,
            },
            TemplateType::Formal => match (*element).as_any().downcast_ref::<Self>() {
                Some(_) => true,
                None => false,
            },
        }
    }
}
#[typetag::serde]
impl TupleField for f64 {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_clone(&self) -> Box<dyn TupleField> {
        Box::new((*self).clone())
    }
    fn query(&self, element: &Box<dyn TupleField>, matching: &TemplateType) -> bool {
        match matching {
            TemplateType::Actual => match (*element).as_any().downcast_ref::<Self>() {
                Some(e) => *self == *e,
                None => false,
            },
            TemplateType::Formal => match (*element).as_any().downcast_ref::<Self>() {
                Some(_) => true,
                None => false,
            },
        }
    }
}
#[typetag::serde]
impl TupleField for char {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_clone(&self) -> Box<dyn TupleField> {
        Box::new((*self).clone())
    }
    fn query(&self, element: &Box<dyn TupleField>, matching: &TemplateType) -> bool {
        match matching {
            TemplateType::Actual => match (*element).as_any().downcast_ref::<Self>() {
                Some(e) => *self == *e,
                None => false,
            },
            TemplateType::Formal => match (*element).as_any().downcast_ref::<Self>() {
                Some(_) => true,
                None => false,
            },
        }
    }
}

#[typetag::serde]
impl TupleField for bool {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_clone(&self) -> Box<dyn TupleField> {
        Box::new((*self).clone())
    }
    fn query(&self, element: &Box<dyn TupleField>, matching: &TemplateType) -> bool {
        match matching {
            TemplateType::Actual => match (*element).as_any().downcast_ref::<Self>() {
                Some(e) => *self == *e,
                None => false,
            },
            TemplateType::Formal => match (*element).as_any().downcast_ref::<Self>() {
                Some(_) => true,
                None => false,
            },
        }
    }
}

#[typetag::serde]
impl TupleField for String {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_clone(&self) -> Box<dyn TupleField> {
        Box::new((*self).clone())
    }
    fn query(&self, element: &Box<dyn TupleField>, matching: &TemplateType) -> bool {
        match matching {
            TemplateType::Actual => match (*element).as_any().downcast_ref::<Self>() {
                Some(e) => *self == *e,
                None => false,
            },
            TemplateType::Formal => match (*element).as_any().downcast_ref::<Self>() {
                Some(_) => true,
                None => false,
            },
        }
    }
}
