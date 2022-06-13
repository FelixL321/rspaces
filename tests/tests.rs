#[cfg(test)]
mod tests {
    use core::time;
    use rspaces::{
        create_template, space_put, FieldType, Repository, Space, Template, TemplateType, Tuple,
        TupleField,
    };
    use serde::{Deserialize, Serialize};
    use std::{any::Any, sync::Arc, thread};

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
        let space = Space::new_sequential();
        let a: i32 = 5;
        let b = 'b';
        let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
        let tuple = Tuple::new(fields);
        space.put(tuple);
        let q = create_template!(5.actual(), 'a'.formal());
        let t = space.get(&q);
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
    }

    #[test]
    fn space_search_failing() {
        let space = Space::new_sequential();
        let a = 5;
        let b = 'b';
        let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
        let tuple = Tuple::new(fields);
        space.put(tuple);
        let mut q = Template::new();
        q.fields.push(5.actual());
        q.fields.push(true.formal());
        if let Some(_t) = space.getp(&q) {
            assert!(false, "We found touple and we should not");
        } else {
            assert!(true);
        }
    }
    #[test]
    fn multithread() {
        let sender = Arc::new(Space::new_sequential());
        let reciever = Arc::clone(&sender);
        thread::spawn(move || {
            let a = 5;
            let b = 'b';
            let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
            let tuple = Tuple::new(fields);
            sender.put(tuple);
        });

        let q = create_template!(5.actual(), 'a'.formal());
        let t = reciever.get(&q);
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
    }

    #[test]
    fn multithread_nonblock() {
        let sender = Arc::new(Space::new_sequential());
        let reciever = Arc::clone(&sender);
        thread::spawn(move || {
            let a = 5;
            let b = 'b';
            let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
            let tuple = Tuple::new(fields);
            sender.put(tuple);
        });

        let mut q = Template::new();
        q.fields.push(5.actual());
        q.fields.push('a'.formal());
        loop {
            if let Some(t) = reciever.getp(&q) {
                assert_eq!(5, *t.get_field::<i32>(0).unwrap());
                assert_eq!('b', *t.get_field::<char>(1).unwrap());
                return;
            }
        }
    }

    #[test]
    fn multithread_query() {
        let sender = Arc::new(Space::new_sequential());
        let reciever = Arc::clone(&sender);
        thread::spawn(move || {
            let a = 5;
            let b = 'b';
            let tuple = Tuple::new(vec![Box::new(a), Box::new(b)]);
            sender.put(tuple);
        });

        let q = create_template!(5.actual(), 'a'.formal());
        let t = reciever.query(&q);
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
    }
    #[test]
    fn multithread_query_nonblock() {
        let sender = Arc::new(Space::new_sequential());
        let reciever = Arc::clone(&sender);
        thread::spawn(move || {
            let a = 5;
            let b = 'b';
            let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
            let tuple = Tuple::new(fields);
            sender.put(tuple);
        });

        let mut q = Template::new();
        q.fields.push(5.actual());
        q.fields.push('a'.formal());
        let mut c = 0;
        loop {
            c += 1;
            if let Some(t) = reciever.queryp(&q) {
                assert_eq!(5, *t.get_field::<i32>(0).unwrap());
                assert_eq!('b', *t.get_field::<char>(1).unwrap());
                println!("{}", c);
                return;
            }
        }
    }

    #[test]
    fn getall() {
        let sender = Arc::new(Space::new_sequential());
        let reciever = Arc::clone(&sender);
        let handle = thread::spawn(move || {
            let a = 5;
            let b = 'b';
            let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
            let tuple = Tuple::new(fields);
            sender.put(tuple);
            let a = 5;
            let b = 'c';
            let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
            let tuple = Tuple::new(fields);
            sender.put(tuple);
        });
        handle.join().expect("thread didnt join");
        let mut q = Template::new();
        q.fields.push(5.actual());
        q.fields.push('a'.formal());
        let tvec = reciever.getall(&q);
        let t = tvec.get(0).expect("should be touple");
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
        let t = tvec.get(1).expect("should be touple");
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('c', *t.get_field::<char>(1).unwrap());
    }

    #[test]
    fn queryall() {
        let sender = Arc::new(Space::new_sequential());
        let reciever = Arc::clone(&sender);
        let handle = thread::spawn(move || {
            let a = 5;
            let b = 'b';
            let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
            let tuple = Tuple::new(fields);
            sender.put(tuple);
            let a = 5;
            let b = 'c';
            let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
            let tuple = Tuple::new(fields);
            sender.put(tuple);
        });
        handle.join().expect("thread didnt join");
        let mut q = Template::new();
        q.fields.push(5.actual());
        q.fields.push('b'.formal());
        let tvec = reciever.queryall(&q);
        let t = tvec.get(0).expect("should be touple");
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
        let t = tvec.get(1).expect("should be touple");
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('c', *t.get_field::<char>(1).unwrap());
    }

    #[test]
    fn macro_test() {
        let space = Space::new_sequential();
        space_put!(space, (5, 'b'));
        let q = create_template!(5.actual(), 'a'.formal());
        let t = space.get(&q);
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
    }

    #[test]
    fn ordering_sequential() {
        let sender = Arc::new(Space::new_pile());
        let reciever = Arc::clone(&sender);
        thread::spawn(move || {
            space_put!(sender, ('a', 'b'));
            space_put!(sender, (5, 'b'));
            space_put!(sender, (4, 'b'));
        });

        let q = create_template!(5.actual(), 'a'.formal());
        let t = reciever.get(&q);
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());

        let q = create_template!(4.actual(), 'a'.formal());
        let t = reciever.get(&q);
        assert_eq!(4, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
    }

    #[test]
    fn ordering_queue() {
        let sender = Arc::new(Space::new_queue());
        let reciever = Arc::clone(&sender);
        thread::spawn(move || {
            space_put!(sender, (5, 'b'));
            space_put!(sender, (4, 'b'));
        });

        let ten_millis = time::Duration::from_millis(100);
        thread::sleep(ten_millis);
        let q = create_template!(4.actual(), 'a'.formal());
        if let Some(_t) = reciever.getp(&q) {
            assert!(false, "Found tuple and should not")
        } else {
            assert!(true);
        }

        let q = create_template!(5.actual(), 'a'.formal());
        let t = reciever.get(&q);
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());

        let q = create_template!(4.actual(), 'a'.formal());
        let t = reciever.get(&q);
        assert_eq!(4, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
    }

    #[test]
    fn ordering_stack() {
        let sender = Arc::new(Space::new_stack());
        let reciever = Arc::clone(&sender);
        thread::spawn(move || {
            space_put!(sender, (4, 'b'));
            space_put!(sender, (5, 'b'));
        });

        let ten_millis = time::Duration::from_millis(100);
        thread::sleep(ten_millis);
        let q = create_template!(4.actual(), 'a'.formal());
        if let Some(_t) = reciever.getp(&q) {
            assert!(false, "Found tuple and should not")
        } else {
            assert!(true);
        }
        let q = create_template!(5.actual(), 'a'.formal());
        let t = reciever.get(&q);
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());

        let q = create_template!(4.actual(), 'a'.formal());
        let t = reciever.get(&q);
        assert_eq!(4, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
    }

    #[test]
    fn ordering_pile() {
        let sender = Arc::new(Space::new_pile());
        let reciever = Arc::clone(&sender);
        thread::spawn(move || {
            space_put!(sender, (4, 'b'));
            space_put!(sender, (5, 'b'));
            space_put!(sender, ('a', 'b'));
        });

        let q = create_template!(5.actual(), 'a'.formal());
        let t = reciever.get(&q);
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());

        let q = create_template!(4.actual(), 'a'.formal());
        let t = reciever.get(&q);
        assert_eq!(4, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
    }

    #[test]
    fn repository() {
        let repo = Arc::new(Repository::new());
        let space1 = Arc::new(Space::new_sequential());
        let space2 = Arc::new(Space::new_sequential());
        repo.add_space(String::from("space1"), Arc::clone(&space1));
        repo.add_space(String::from("space2"), Arc::clone(&space2));
        let repoarc = Arc::clone(&repo);
        thread::spawn(move || {
            let space1 = repoarc
                .get_space(String::from("space1"))
                .expect("Should have found space");
            let space2 = repoarc
                .get_space(String::from("space2"))
                .expect("Should have found space");
            space_put!(space1, (4, 'b'));
            space_put!(space2, (5, 'b'));
        });
        let q = create_template!(5.actual(), 'a'.formal());
        let t = space2.get(&q);
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());

        let q = create_template!(4.actual(), 'a'.formal());
        let t = space1.get(&q);
        assert_eq!(4, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
    }

    #[test]
    fn multiple_repository() {
        let repo1 = Arc::new(Repository::new());
        let repo2 = Arc::new(Repository::new());
        let space1 = Arc::new(Space::new_sequential());
        let space2 = Arc::new(Space::new_sequential());
        let space3 = Arc::new(Space::new_sequential());
        repo1.add_space(String::from("space1"), Arc::clone(&space1));
        repo1.add_space(String::from("space2"), Arc::clone(&space2));
        repo2.add_space(String::from("space2"), Arc::clone(&space2));
        repo2.add_space(String::from("space3"), Arc::clone(&space3));
        let repoarc = Arc::clone(&repo1);
        thread::spawn(move || {
            let space1 = repoarc
                .get_space(String::from("space1"))
                .expect("Should have found space");
            let space2 = repoarc
                .get_space(String::from("space2"))
                .expect("Should have found space");
            space_put!(space1, (4, 'b'));
            space_put!(space2, (5, 'b'));
        });
        let repoarc = Arc::clone(&repo2);
        thread::spawn(move || {
            let space2 = repoarc
                .get_space(String::from("space2"))
                .expect("Should have found space");
            let space3 = repoarc
                .get_space(String::from("space3"))
                .expect("Should have found space");
            space_put!(space2, (4, 'b'));
            space_put!(space3, (5, 'b'));
        });

        let q = create_template!(4.actual(), 'a'.formal());
        let t = space2.get(&q);
        assert_eq!(4, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());

        let q = create_template!(5.actual(), 'a'.formal());
        let t = space2.get(&q);
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());

        let q = create_template!(4.actual(), 'a'.formal());
        let t = space1.get(&q);
        assert_eq!(4, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());

        let q = create_template!(5.actual(), 'a'.formal());
        let t = space3.get(&q);
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
    }

    #[test]
    fn repository_delete() {
        let repo = Arc::new(Repository::new());
        let space1 = Arc::new(Space::new_sequential());
        let space2 = Arc::new(Space::new_sequential());
        repo.add_space(String::from("space1"), Arc::clone(&space1));
        repo.add_space(String::from("space2"), Arc::clone(&space2));
        repo.del_space(String::from("space1"));
        match repo.get_space(String::from("space1")) {
            Some(_) => assert!(false, "space should have been deleted"),
            None => assert!(true),
        }
    }

    #[test]
    fn seri_test() {
        let a: i32 = 5;
        let b = 'b';
        let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
        let tuple = Tuple::new(fields);
        let v = serde_json::to_string(&tuple).unwrap();
        print!("{}", v);
        let x: Tuple = serde_json::from_str(&v).unwrap();
        let ap = x.get_field::<i32>(0).expect("could not cast");
        assert_eq!(5, *ap);
        let bp = tuple.get_field::<char>(1).expect("could not cast");
        assert_eq!('b', *bp);
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq)]
    struct TestStruct {
        x: i32,
        y: f64,
    }

    #[typetag::serde]
    impl TupleField for TestStruct {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn box_clone(&self) -> Box<dyn TupleField> {
            Box::new((*self).clone())
        }
        fn query(&self, element: &Box<dyn TupleField>, matching: &TemplateType) -> bool {
            match matching {
                TemplateType::Actual => match (*element).as_any().downcast_ref::<Self>() {
                    Some(e) => *self == *e,
                    None => false,
                },
                TemplateType::Formal => match (*element).as_any().downcast_ref::<Self>() {
                    Some(_) => true,
                    None => false,
                },
            }
        }
    }

    #[test]
    fn seri_test_custom() {
        let a: i32 = 5;
        let b = TestStruct { x: 27, y: 65.7 };
        let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
        let tuple = Tuple::new(fields);
        let v = serde_json::to_string(&tuple).unwrap();
        println!("{}", v);
        let x: Tuple = serde_json::from_str(&v).unwrap();
        let ap = x.get_field::<i32>(0).expect("could not cast");
        assert_eq!(5, *ap);
        let bp = tuple.get_field::<TestStruct>(1).expect("could not cast");
        assert_eq!(27, (*bp).x);
        assert_eq!(65.7, (*bp).y);
    }

    #[test]
    fn seri_test_template() {
        let space = Space::new_sequential();
        let a = 5;
        let b = 'b';
        let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
        let tuple = Tuple::new(fields);
        space.put(tuple);
        let q = create_template!(5.actual(), 'a'.formal());
        let q_json = serde_json::to_string(&q).unwrap();
        print!("{}", q_json);
        let template: Template = serde_json::from_str(&q_json).unwrap();

        let t = space.get(&template);
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
    }
}
