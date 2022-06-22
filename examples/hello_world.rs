use rspaces::{new_template, new_tuple, FieldType, LocalSpace, Space, Template, Tuple, TupleField};

fn main() {
    //Create space
    let space = LocalSpace::new_sequential();

    //Put a new tuple containing a string into the space
    space.put(new_tuple!("Hello World!".to_string())).unwrap();

    //Create a new template to get a tuple containing a single string from the space
    let tuple = space.get(new_template!("s".to_string().formal())).unwrap();
    println!("{}", tuple.get_field::<String>(0));
}
