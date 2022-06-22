#[macro_export]
macro_rules! new_tuple {
    ( $( $x:expr ),* ) => {
        {
            let mut _temp_vec : Vec<Box<dyn TupleField>> = Vec::new();
            $(
                _temp_vec.push(Box::new($x));
            )*
            Tuple::new(_temp_vec)
        }
    };

}

#[macro_export]
macro_rules! new_template {
    ( $( $x:expr ),* ) => {
        {
            let mut _q = Template::new();
            $(
                _q.fields.push($x);
            )*
            _q
        }
    };
}

#[macro_export]
macro_rules! implement_tuplefield_for {
    (  $x:ty  ) => {
        #[typetag::serde]
        impl TupleField for $x {
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
    };
}
