use std::any::Any;

use serde::{Deserialize, Serialize};

use crate::{implement_tuplefield_for, TemplateType};

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
    /// # Panic
    /// Function panics if index is not valid in the tuple or if the type supplied is not equal of that of the field.
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
    /// let five = *tuple.get_field::<i32>(0);
    /// let charb = *tuple.get_field::<char>(1);
    ///
    /// assert_eq!(5, five);
    /// assert_eq!('b', charb);
    /// ```
    pub fn get_field<T: 'static>(&self, index: usize) -> &T {
        let b = (*(*self.fields.get(index).expect("Wrong index")))
            .as_any()
            .downcast_ref::<T>();
        return b.unwrap();
    }
}

#[typetag::serde(tag = "field")]
pub trait TupleField: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn box_clone(&self) -> Box<dyn TupleField>;
    fn query(&self, element: &Box<dyn TupleField>, matching: &TemplateType) -> bool;
}

//Impl blocks as serde typetag wont allow for generic
implement_tuplefield_for!(i8);
implement_tuplefield_for!(i16);
implement_tuplefield_for!(i32);
implement_tuplefield_for!(i64);
implement_tuplefield_for!(i128);
implement_tuplefield_for!(u8);
implement_tuplefield_for!(u16);
implement_tuplefield_for!(u32);
implement_tuplefield_for!(u64);
implement_tuplefield_for!(u128);
implement_tuplefield_for!(usize);
implement_tuplefield_for!(isize);
implement_tuplefield_for!(f32);
implement_tuplefield_for!(f64);
implement_tuplefield_for!(char);
implement_tuplefield_for!(String);
implement_tuplefield_for!(bool);
