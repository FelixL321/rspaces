use core::panic;
use std::{sync::Arc, thread};

use rspaces::{
    new_template, new_tuple, FieldType, LocalSpace, RemoteSpace, Repository, Space, Template,
    Tuple, TupleField,
};

fn main() {
    let gate_uri = "127.0.0.1:9001";
    let remote_uri = "127.0.0.1:9001/aspace";

    //Create new repo
    let repo = Arc::new(Repository::new());

    //Add gate to the repository
    Repository::add_gate(
        Arc::clone(&repo),
        String::from("gate1"),
        gate_uri.to_string(),
    )
    .expect("Could not connect");

    //Create a space and put it into the repository
    let space = Arc::new(LocalSpace::new_sequential());
    repo.add_space("aspace".to_string(), Arc::clone(&space));

    thread::spawn(move || {
        //Connecting to remote space
        let space = match RemoteSpace::new(remote_uri.to_string()) {
            Ok(s) => s,
            Err(e) => panic!("failed to connect to remote space, msg: {}", e),
        };

        //Creating fields
        let greeting = "GREETING".to_string();
        let hello = "Hello".to_string();

        //Creating tuple and putting it into the remote space
        match space.put(new_tuple!(greeting, hello)) {
            Ok(_) => {}
            Err(e) => panic!("Could not put tuple in remote space, msg: {}", e),
        }
        println!("T1 Done!");
    });
    thread::spawn(move || {
        //Connecting to remote space
        let space = match RemoteSpace::new(remote_uri.to_string()) {
            Ok(s) => s,
            Err(e) => panic!("failed to connect to remote space, msg: {}", e),
        };

        //Creating fields
        let name = "NAME".to_string();
        let world = "World".to_string();

        //Creating tuple and putting it into the remote space
        match space.put(new_tuple!(name, world)) {
            Ok(_) => {}
            Err(e) => panic!("Could not put tuple in remote space, msg: {}", e),
        }
        println!("T2 Done!");
    });

    //Getting tuples from the space
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
    repo.close_gate("aspace".to_string());
}
