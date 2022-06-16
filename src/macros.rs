#[macro_export]
macro_rules! space_put {
    ( $s: expr, ($( $x:expr ),*) ) => {
        {
            let mut temp_vec : Vec<Box<dyn TupleField>> = Vec::new();
            $(
                temp_vec.push(Box::new($x));
            )*
            let x = Tuple::new(temp_vec);
            $s.put(x);
        }
    };
}

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
