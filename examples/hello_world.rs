use rspaces::{new_template, new_tuple, FieldType, LocalSpace, Space, Template, Tuple, TupleField};

fn main() {
    let space = LocalSpace::new_sequential();
    space.put(new_tuple!("Hello World!".to_string())).unwrap();
    let tuple = space.get(new_template!("s".to_string().formal())).unwrap();
    println!("{}", tuple.get_field::<String>(0).unwrap());
}
