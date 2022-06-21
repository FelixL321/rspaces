use core::panic;
use std::{sync::Arc, thread};

use rspaces::{
    new_template, new_tuple, FieldType, LocalSpace, RemoteSpace, Repository, Space, Template,
    Tuple, TupleField,
};

fn main() {
    let gate_uri = "127.0.0.1:9001";
    let remote_uri = "127.0.0.1:9001/aspace";

    let repo = Arc::new(Repository::new());
    Repository::add_gate(
        Arc::clone(&repo),
        String::from("gate1"),
        gate_uri.to_string(),
    )
    .expect("Could not connect");
    let space = Arc::new(LocalSpace::new_sequential());
    repo.add_space("aspace".to_string(), Arc::clone(&space));

    thread::spawn(move || {
        let space = match RemoteSpace::new(remote_uri.to_string()) {
            Ok(s) => s,
            Err(e) => panic!("failed to connect to remote space, msg: {}", e),
        };
        let greeting = "GREETING".to_string();
        let hello = "Hello".to_string();
        match space.put(new_tuple!(greeting, hello)) {
            Ok(_) => {}
            Err(e) => panic!("Could not put tuple in remote space, msg: {}", e),
        }
        println!("T1 Done!");
    });
    thread::spawn(move || {
        let space = match RemoteSpace::new(remote_uri.to_string()) {
            Ok(s) => s,
            Err(e) => panic!("failed to connect to remote space, msg: {}", e),
        };
        let name = "NAME".to_string();
        let world = "World".to_string();
        match space.put(new_tuple!(name, world)) {
            Ok(_) => {}
            Err(e) => panic!("Could not put tuple in remote space, msg: {}", e),
        }
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
    repo.close_gate("aspace".to_string());
}
