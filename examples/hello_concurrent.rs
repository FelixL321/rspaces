use std::{sync::Arc, thread};

use rspaces::{new_template, new_tuple, FieldType, LocalSpace, Space, Template, Tuple, TupleField};

fn main() {
    let space = Arc::new(LocalSpace::new_sequential());
    let spaceclone = Arc::clone(&space);
    thread::spawn(move || {
        let greeting = "GREETING".to_string();
        let hello = "Hello".to_string();
        spaceclone.put(new_tuple!(greeting, hello)).unwrap();
        println!("T1 Done!");
    });
    let spaceclone2 = Arc::clone(&space);
    thread::spawn(move || {
        let name = "NAME".to_string();
        let world = "World".to_string();
        spaceclone2.put(new_tuple!(name, world)).unwrap();
        println!("T2 Done!");
    });

    let greetingtuple = space
        .get(new_template!(
            "GREETING".to_string().actual(),
            "s".to_string().formal()
        ))
        .unwrap();
    let nametuple = space
        .get(new_template!(
            "NAME".to_string().actual(),
            "s".to_string().formal()
        ))
        .unwrap();

    let greeting = greetingtuple.get_field::<String>(1).unwrap();
    let name = nametuple.get_field::<String>(1).unwrap();
    println!("{} {}!", greeting, name);
}
