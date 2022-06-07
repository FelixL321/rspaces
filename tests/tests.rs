#[cfg(test)]
mod tests {
    use rspaces::{Queries, Query, SequentialSpace, Space, Tuple, TupleField};
    use std::{sync::Arc, thread};

    #[test]
    fn anytest() {
        let x: Box<dyn TupleField> = Box::new(20);
        let y = (*x).as_any().downcast_ref::<i32>();
        assert_eq!(20, *y.unwrap());
    }
    #[test]
    fn tuple_test() {
        let a = 5;
        let b = 'b';
        let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
        let tuple = Tuple::new(fields);

        let x = tuple.get_field::<i32>(0).expect("could not cast");
        assert_eq!(5, *x);
        let _x = tuple.get_field::<char>(1).expect("could not cast");
    }

    #[test]
    fn tuple_test_failing() {
        let a: i32 = 5;
        let b: char = 'b';
        let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
        let tuple = Tuple::new(fields);

        match tuple.get_field::<u64>(0) {
            Some(_x) => {
                assert!(false, "Got something, when we should not");
            }
            None => assert!(true),
        }
    }

    #[test]
    fn space_search() {
        let space = SequentialSpace::new();
        let a = 5;
        let b = 'b';
        let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
        let tuple = Tuple::new(fields);
        space.put(tuple);
        let mut q = Query::new();
        q.fields.push(5.actual());
        q.fields.push(char::formal());
        if let Some(t) = space.get(&q) {
            assert!(true);
            println!("{}", t.get_field::<i32>(0).unwrap());
        } else {
            assert!(false, "Didnt find tuple...");
        }
    }

    #[test]
    fn space_search_failing() {
        let space = SequentialSpace::new();
        let a = 5;
        let b = 'b';
        let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
        let tuple = Tuple::new(fields);
        space.put(tuple);
        let mut q = Query::new();
        q.fields.push(5.actual());
        q.fields.push(i32::formal());
        if let Some(_t) = space.getp(&q) {
            assert!(false, "We found touple and we should not");
        } else {
            assert!(true);
        }
    }
    #[test]
    fn multithread() {
        let sender = Arc::new(SequentialSpace::new());
        let reciever = Arc::clone(&sender);
        thread::spawn(move || {
            let a = 5;
            let b = 'b';
            let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
            let tuple = Tuple::new(fields);
            sender.put(tuple);
        });

        let mut q = Query::new();
        q.fields.push(5.actual());
        q.fields.push(char::formal());
        if let Some(t) = reciever.get(&q) {
            assert!(true);
            assert_eq!(5, *t.get_field::<i32>(0).unwrap());
            assert_eq!('b', *t.get_field::<char>(1).unwrap());
        } else {
            assert!(false, "Didnt find tuple...");
        }
    }
    #[test]
    fn multithread_nonblock() {
        let sender = Arc::new(SequentialSpace::new());
        let reciever = Arc::clone(&sender);
        thread::spawn(move || {
            let a = 5;
            let b = 'b';
            let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
            let tuple = Tuple::new(fields);
            sender.put(tuple);
        });

        let mut q = Query::new();
        q.fields.push(5.actual());
        q.fields.push(char::formal());
        let mut c = 0;
        loop {
            c += 1;
            if let Some(t) = reciever.getp(&q) {
                assert_eq!(5, *t.get_field::<i32>(0).unwrap());
                assert_eq!('b', *t.get_field::<char>(1).unwrap());
                println!("{}", c);
                return;
            }
        }
    }
}
