use std::{sync::Arc, thread};

use rspaces::{new_template, new_tuple, FieldType, LocalSpace, Space, Template, Tuple, TupleField};

fn main() {
    //Creating new space
    let space = Arc::new(LocalSpace::new_sequential());

    //Creating a clone of the reference counter so that it can be send to multiple threads
    let spaceclone = Arc::clone(&space);

    //Spawning new thread
    thread::spawn(move || {
        let greeting = "GREETING".to_string();
        let hello = "Hello".to_string();

        //Putting a new tuple in the space
        spaceclone.put(new_tuple!(greeting, hello)).unwrap();
        println!("T1 Done!");
    });

    //Creating a new clone of the reference counter so that it can be send to multiple threads
    let spaceclone2 = Arc::clone(&space);

    //Spawning new thread
    thread::spawn(move || {
        let name = "NAME".to_string();
        let world = "World".to_string();

        //Putting a new tuple in the space
        spaceclone2.put(new_tuple!(name, world)).unwrap();
        println!("T2 Done!");
    });

    //Creating templates and getting tuples from the space
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

    let greeting = greetingtuple.get_field::<String>(1);
    let name = nametuple.get_field::<String>(1);
    println!("{} {}!", greeting, name);
}
