use std::{
    io::{stdin, stdout, Write},
    ops::Add,
    sync::Arc,
};

use rspaces::{
    new_template, new_tuple, FieldType, LocalSpace, RemoteSpace, Repository, Space, Template,
    Tuple, TupleField,
};

fn main() {
    println!("Welcome to chat app, type h to host and j to join");

    let mut input_string = String::new();
    while input_string != "h" && input_string != "j" {
        input_string = input();
        if input_string != "h" && input_string != "j" {
            println!("Please try again: {}", input_string);
        }
    }
    if input_string == "h" {
        handle_host();
    } else if input_string == "j" {
        handle_join();
    }
}

//Handle a joining client
fn handle_join() {
    println!("Please type a ip and port to join (For example \"127.0.0.1:3800\")");
    let mut conn_string = input();
    conn_string = conn_string.add("/main");

    //Connecting to remote space
    let space = Arc::new(RemoteSpace::new(conn_string).unwrap());

    handle_coms(space, true);
}

//Handle hosting a new space
fn handle_host() {
    println!("Please type a ip and port to host on (For example \"127.0.0.1:3800\")");
    let conn_string = input();

    //Creating a repo and putting a new space into it
    let space = Arc::new(LocalSpace::new_sequential());
    let repo = Arc::new(Repository::new());
    repo.add_space(String::from("main"), Arc::clone(&space));

    //Adding a gate to the repo
    Repository::add_gate(Arc::clone(&repo), String::from("main"), conn_string)
        .expect("couldnt connect");
    handle_coms(space, false);

    //Remember to close gate
    repo.close_gate("main".to_string());
}

fn handle_coms<T: Space>(space: Arc<T>, mut starting: bool) {
    let x = starting;
    println!("Please enter your name");
    let name = input();
    loop {
        if starting {
            print!("Enter new message (type exit to end chat): ");
            let msg = input();
            space.put(new_tuple!(x, name.clone(), msg.clone())).unwrap();
            if msg == String::from("exit") {
                break;
            }
        } else {
            let t = space
                .get(new_template!(
                    (!x).actual(),
                    String::new().formal(),
                    String::new().formal()
                ))
                .unwrap();
            let name = t.get_field::<String>(1);
            let msg = t.get_field::<String>(2);
            if *msg == String::from("exit") {
                println!("Partner quit, quitting as well");
                break;
            } else {
                println!("{} says: {}", name, msg);
            }
        }
        starting = !starting;
    }
}

//Basic input helper function
fn input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}
