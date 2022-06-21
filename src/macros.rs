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
macro_rules! create_template {
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
