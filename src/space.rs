use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;

use rand::thread_rng;
use rand::Rng;

use crate::drain_filter::drain_filter;
use crate::Template;
use crate::Tuple;

pub trait Space {
    /**
    Finds a tuple matching the template in the space, removes it from the space and returns it.

    Will block the current thread until a tuple is found

    # Example
    ```
    # use rspaces::*;
    # let space = Space::new_sequential();
    //Put the tuple (5, 'a') in the space
    space_put!(space, (5, 'a'));

    // Create a query template for the tuple with a 5 followed by a char
    let template = create_template!(5.actual(), char::formal());

    //Query the space for the tuple
    let tuple = space.get(&template);

    assert_eq!(5, *tuple.get_field::<i32>(0).unwrap());
    assert_eq!('a', *tuple.get_field::<char>(1).unwrap());

    ```
    */
    fn get(&self, template: &Template) -> Tuple;
    /**
    Finds a tuple matching the template in the space, removes it from the space and returns it.

    This does not blcok the current thread and therefore returns an option, as theres no garantuee for finding a tuple

    # Example
    ```
    # use rspaces::*;
    # let space = Space::new_sequential();
    //Put the tuple (5, 'a') in the space
    space_put!(space, (5, 'a'));

    // Create a query template for the tuple with a 5 followed by a char
    let template = create_template!(5.actual(), char::formal());

    //Query the space for the tuple
    if let Some(tuple) = space.getp(&template) {
        assert_eq!(5, *tuple.get_field::<i32>(0).unwrap());
        assert_eq!('a', *tuple.get_field::<char>(1).unwrap());
    } else {
        assert!(false);
    }

    ```
    */
    fn getp(&self, template: &Template) -> Option<Tuple>;

    /**
    Puts the given tuple into the tuple space
    # Example
    ```
    # use rspaces::*;
    # let space = Space::new_sequential();
    let a = 5;
    let b = 'b';
    let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
    let tuple = Tuple::new(fields);
    space.put(tuple);
    ```
    Alternatively the same can be done with a macro
    ```
    # use rspaces::*;
    # let space = Space::new_sequential();
    let a = 5;
    let b = 'b';
    space_put!(space, (a, b))
    ```
    */
    fn put(&self, tuple: Tuple);
    /**
    Finds a tuple matching the template in the space, and returns it without removing it.

    This does not blcok the current thread and therefore returns an option, as theres no garantuee for finding a tuple

    # Example
    ```
    # use rspaces::*;
    # let space = Space::new_sequential();
    //Put the tuple (5, 'a') in the space
    space_put!(space, (5, 'a'));

    // Create a query template for the tuple with a 5 followed by a char
    let template = create_template!(5.actual(), char::formal());

    //Query the space for the tuple
    if let Some(tuple) = space.queryp(&template) {
        assert_eq!(5, *tuple.get_field::<i32>(0).unwrap());
        assert_eq!('a', *tuple.get_field::<char>(1).unwrap());
    } else {
        assert!(false);
    }

    ```
    */
    fn queryp(&self, query: &Template) -> Option<Tuple>;

    /**
    Finds a tuple matching the template in the space, and returns it without removing it.

    Will block the current thread until a tuple is found

    # Example
    ```
    # use rspaces::*;
    # let space = Space::new_sequential();
    //Put the tuple (5, 'a') in the space
    space_put!(space, (5, 'a'));

    // Create a query template for the tuple with a 5 followed by a char
    let template = create_template!(5.actual(), char::formal());

    //Query the space for the tuple
    let tuple = space.query(&template);

    assert_eq!(5, *tuple.get_field::<i32>(0).unwrap());
    assert_eq!('a', *tuple.get_field::<char>(1).unwrap());

    ```
    */
    fn query(&self, template: &Template) -> Tuple;
    /**
    Gets all tuples in the space matching the template and removes them from the space
    # Example
    ```
    # use rspaces::*;
    # let space = Space::new_sequential();
    //Put tuples in the space
    space_put!(space, (5, 'a'));
    space_put!(space, (4, 'b'));
    space_put!(space, (4, 'c'));

    // Create a query template for the tuple with a 4 followed by a char
    let template = create_template!(4.actual(), char::formal());

    //Query the space for all tuples matching
    let tuples = space.getall(&template);

    assert_eq!(2, tuples.len());
    for tuple in tuples.iter(){
        assert_eq!(4, *tuple.get_field::<i32>(0).unwrap());
    }

    ```
    */
    fn getall(&self, template: &Template) -> Vec<Tuple>;
    /**
    Gets all tuples in the space matching the template without removing them
    # Example
    ```
    # use rspaces::*;
    # let space = Space::new_sequential();
    //Put tuples in the space
    space_put!(space, (5, 'a'));
    space_put!(space, (4, 'b'));
    space_put!(space, (4, 'c'));

    // Create a query template for the tuple with a 4 followed by a char
    let template = create_template!(4.actual(), char::formal());

    //Query the space for all tuples matching
    let tuples = space.queryall(&template);

    assert_eq!(2, tuples.len());
    for tuple in tuples.iter(){
        assert_eq!(4, *tuple.get_field::<i32>(0).unwrap());
    }

    ```
    */
    fn queryall(&self, template: &Template) -> Vec<Tuple>;
}

enum SpaceType {
    Sequential,
    Queue,
    Stack,
    Pile,
    Random,
}

/**
A tuple space for storing tuples and retrieving tuples

# Example

```
# use rspaces::*;
//Create Space
let space = Space::new_sequential();

//Put the tuple (5, 'a') in the space
space_put!(space, (5, 'a'));

// Create a query template for the tuple with a 5 followed by a char
let template = create_template!(5.actual(), char::formal());

//Query the space for the tuple
let tuple = space.get(&template);

assert_eq!(5, *tuple.get_field::<i32>(0).unwrap());
assert_eq!('a', *tuple.get_field::<char>(1).unwrap());

```
*/
pub struct LocalSpace {
    v: Mutex<Vec<Tuple>>,
    listeners: Mutex<Vec<Sender<bool>>>,
    spacetype: SpaceType,
}

//Constructors
impl LocalSpace {
    /**
    Create a new sequential space

    get and query will return the oldest tuple matching the template

    */
    pub fn new_sequential() -> LocalSpace {
        LocalSpace {
            v: Mutex::new(Vec::new()),
            listeners: Mutex::new(Vec::new()),
            spacetype: SpaceType::Sequential,
        }
    }
    /**
    Create a new queue space

    get and query will return the oldest tuple if it matches the template

    */
    pub fn new_queue() -> LocalSpace {
        LocalSpace {
            v: Mutex::new(Vec::new()),
            listeners: Mutex::new(Vec::new()),
            spacetype: SpaceType::Queue,
        }
    }
    /**
    Create a new stack space

    get and query will return the newest tuple if it matcehs the template

    */
    pub fn new_stack() -> LocalSpace {
        LocalSpace {
            v: Mutex::new(Vec::new()),
            listeners: Mutex::new(Vec::new()),
            spacetype: SpaceType::Stack,
        }
    }
    /**
    Create a new sequential space

    get and query will return the newest tuple matching the template

    */
    pub fn new_pile() -> LocalSpace {
        LocalSpace {
            v: Mutex::new(Vec::new()),
            listeners: Mutex::new(Vec::new()),
            spacetype: SpaceType::Pile,
        }
    }
    /**
    Create a new sequential space

    get and query will return a random tuple matching the template

    */
    pub fn new_random() -> LocalSpace {
        LocalSpace {
            v: Mutex::new(Vec::new()),
            listeners: Mutex::new(Vec::new()),
            spacetype: SpaceType::Random,
        }
    }

    fn look(&self, query: &Template, destroy: bool) -> Option<Tuple> {
        let mut v = self.v.lock().unwrap();
        let index: usize;
        match self.spacetype {
            SpaceType::Sequential => {
                if let Some(i) = v.iter().position(|t| query.query(t)) {
                    index = i;
                } else {
                    return None;
                }
            }
            SpaceType::Queue => {
                if v.len() > 0 && query.query(v.get(0).unwrap()) {
                    index = 0;
                } else {
                    return None;
                }
            }
            SpaceType::Pile => {
                if let Some(i) = v.iter().rev().position(|t| query.query(t)) {
                    index = v.len() - i - 1;
                } else {
                    return None;
                }
            }
            SpaceType::Stack => {
                if v.len() > 0 && query.query(v.get(v.len() - 1).unwrap()) {
                    index = v.len() - 1;
                } else {
                    return None;
                }
            }
            SpaceType::Random => {
                let matches = self.queryall(query);
                if matches.len() == 0 {
                    return None;
                }
                let mut rng = thread_rng();
                index = rng.gen_range(0..matches.len());
            }
        }
        match destroy {
            true => Some(v.remove(index)),
            false => {
                let ret = (*v.get(index).unwrap()).clone();
                Some(ret)
            }
        }
    }
}

//API
impl Space for LocalSpace {
    fn get(&self, template: &Template) -> Tuple {
        loop {
            match self.getp(&template) {
                Some(t) => return t,
                None => {
                    let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
                    {
                        let mut l = self.listeners.lock().unwrap();
                        l.push(tx);
                    }
                    let _ = rx.recv();
                }
            };
        }
    }

    fn getp(&self, template: &Template) -> Option<Tuple> {
        self.look(template, true)
    }

    fn put(&self, tuple: Tuple) {
        let mut v = self.v.lock().unwrap();
        v.push(tuple);
        let mut l = self.listeners.lock().unwrap();
        let mut remove = Vec::new();
        for i in 0..l.len() {
            let tx = l.get(i).unwrap();
            match tx.send(true) {
                Err(e) => panic!("panic: {:?}", e),
                Ok(_) => remove.push(i),
            }
        }
        for i in remove.iter() {
            l.remove(*i);
        }
    }

    fn queryp(&self, query: &Template) -> Option<Tuple> {
        self.look(query, false)
    }

    fn query(&self, template: &Template) -> Tuple {
        loop {
            match self.queryp(&template) {
                Some(t) => return t,
                None => {
                    let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
                    {
                        let mut l = self.listeners.lock().unwrap();
                        l.push(tx);
                    }
                    let _ = rx.recv();
                }
            };
        }
    }

    fn getall(&self, template: &Template) -> Vec<Tuple> {
        let mut v = self.v.lock().unwrap();
        drain_filter(&mut v, |t| template.query(t)).collect::<Vec<_>>()
    }

    fn queryall(&self, template: &Template) -> Vec<Tuple> {
        let v = self.v.lock().unwrap();
        let viter = v.iter().filter(|t| template.query(t));
        let mut res = Vec::new();
        for t in viter {
            res.push(t.clone());
        }
        res
    }
}
