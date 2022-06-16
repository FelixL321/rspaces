#[cfg(test)]
mod tests {
    use core::time;
    use rspace_macro::TupleField;
    use rspaces::{
        create_template, new_tuple, space_put, FieldType, LocalSpace, Message, MessageType,
        Repository, Space, Template, TemplateType, Tuple, TupleField,
    };
    use serde::{Deserialize, Serialize};
    use std::{
        any::Any,
        io::{Read, Write},
        net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
        sync::Arc,
        thread,
    };

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
        let space = LocalSpace::new_sequential();
        let a: i32 = 5;
        let b = 'b';
        let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
        let tuple = Tuple::new(fields);
        space.put(tuple).unwrap();
        let q = create_template!(5.actual(), 'a'.formal());
        let t = space.get(q).unwrap();
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
    }

    #[test]
    fn space_search_failing() {
        let space = LocalSpace::new_sequential();
        let a = 5;
        let b = 'b';
        let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
        let tuple = Tuple::new(fields);
        space.put(tuple).unwrap();
        let mut q = Template::new();
        q.fields.push(5.actual());
        q.fields.push(true.formal());
        if let Ok(_t) = space.getp(q) {
            assert!(false, "We found touple and we should not");
        } else {
            assert!(true);
        }
    }
    #[test]
    fn multithread() {
        let sender = Arc::new(LocalSpace::new_sequential());
        let reciever = Arc::clone(&sender);
        thread::spawn(move || {
            let a = 5;
            let b = 'b';
            let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
            let tuple = Tuple::new(fields);
            sender.put(tuple).unwrap();
        });

        let q = create_template!(5.actual(), 'a'.formal());
        let t = reciever.get(q).unwrap();
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
    }

    #[test]
    fn multithread_nonblock() {
        let sender = Arc::new(LocalSpace::new_sequential());
        let reciever = Arc::clone(&sender);
        thread::spawn(move || {
            let a = 5;
            let b = 'b';
            let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
            let tuple = Tuple::new(fields);
            sender.put(tuple).unwrap();
        });

        let mut q = Template::new();
        q.fields.push(5.actual());
        q.fields.push('a'.formal());
        loop {
            if let Ok(t) = reciever.getp(q.clone()) {
                assert_eq!(5, *t.get_field::<i32>(0).unwrap());
                assert_eq!('b', *t.get_field::<char>(1).unwrap());
                return;
            }
        }
    }

    #[test]
    fn multithread_query() {
        let sender = Arc::new(LocalSpace::new_sequential());
        let reciever = Arc::clone(&sender);
        thread::spawn(move || {
            let a = 5;
            let b = 'b';
            let tuple = Tuple::new(vec![Box::new(a), Box::new(b)]);
            sender.put(tuple).unwrap();
        });

        let q = create_template!(5.actual(), 'a'.formal());
        let t = reciever.query(q).unwrap();
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
    }
    #[test]
    fn multithread_query_nonblock() {
        let sender = Arc::new(LocalSpace::new_sequential());
        let reciever = Arc::clone(&sender);
        thread::spawn(move || {
            let a = 5;
            let b = 'b';
            let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
            let tuple = Tuple::new(fields);
            sender.put(tuple).unwrap();
        });

        let mut q = Template::new();
        q.fields.push(5.actual());
        q.fields.push('a'.formal());
        let mut c = 0;
        loop {
            c += 1;
            if let Ok(t) = reciever.queryp(q.clone()) {
                assert_eq!(5, *t.get_field::<i32>(0).unwrap());
                assert_eq!('b', *t.get_field::<char>(1).unwrap());
                println!("{}", c);
                return;
            }
        }
    }

    #[test]
    fn getall() {
        let sender = Arc::new(LocalSpace::new_sequential());
        let reciever = Arc::clone(&sender);
        let handle = thread::spawn(move || {
            let a = 5;
            let b = 'b';
            let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
            let tuple = Tuple::new(fields);
            sender.put(tuple).unwrap();
            let a = 5;
            let b = 'c';
            let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
            let tuple = Tuple::new(fields);
            sender.put(tuple).unwrap();
        });
        handle.join().expect("thread didnt join");
        let mut q = Template::new();
        q.fields.push(5.actual());
        q.fields.push('a'.formal());
        let tvec = reciever.getall(q).unwrap();
        let t = tvec.get(0).expect("should be touple");
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
        let t = tvec.get(1).expect("should be touple");
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('c', *t.get_field::<char>(1).unwrap());
    }

    #[test]
    fn queryall() {
        let sender = Arc::new(LocalSpace::new_sequential());
        let reciever = Arc::clone(&sender);
        let handle = thread::spawn(move || {
            let a = 5;
            let b = 'b';
            let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
            let tuple = Tuple::new(fields);
            sender.put(tuple).unwrap();
            let a = 5;
            let b = 'c';
            let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
            let tuple = Tuple::new(fields);
            sender.put(tuple).unwrap();
        });
        handle.join().expect("thread didnt join");
        let mut q = Template::new();
        q.fields.push(5.actual());
        q.fields.push('b'.formal());
        let tvec = reciever.queryall(q).unwrap();
        let t = tvec.get(0).expect("should be touple");
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
        let t = tvec.get(1).expect("should be touple");
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('c', *t.get_field::<char>(1).unwrap());
    }

    #[test]
    fn macro_test() {
        let space = LocalSpace::new_sequential();
        space_put!(space, (5, 'b'));
        let q = create_template!(5.actual(), 'a'.formal());
        let t = space.get(q).unwrap();
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
    }

    #[test]
    fn ordering_sequential() {
        let sender = Arc::new(LocalSpace::new_pile());
        let reciever = Arc::clone(&sender);
        thread::spawn(move || {
            space_put!(sender, ('a', 'b'));
            space_put!(sender, (5, 'b'));
            space_put!(sender, (4, 'b'));
        });

        let q = create_template!(5.actual(), 'a'.formal());
        let t = reciever.get(q).unwrap();
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());

        let q = create_template!(4.actual(), 'a'.formal());
        let t = reciever.get(q).unwrap();
        assert_eq!(4, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
    }

    #[test]
    fn ordering_queue() {
        let sender = Arc::new(LocalSpace::new_queue());
        let reciever = Arc::clone(&sender);
        thread::spawn(move || {
            space_put!(sender, (5, 'b'));
            space_put!(sender, (4, 'b'));
        });

        let ten_millis = time::Duration::from_millis(100);
        thread::sleep(ten_millis);
        let q = create_template!(4.actual(), 'a'.formal());
        if let Ok(_t) = reciever.getp(q) {
            assert!(false, "Found tuple and should not")
        } else {
            assert!(true);
        }

        let q = create_template!(5.actual(), 'a'.formal());
        let t = reciever.get(q).unwrap();
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());

        let q = create_template!(4.actual(), 'a'.formal());
        let t = reciever.get(q).unwrap();
        assert_eq!(4, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
    }

    #[test]
    fn ordering_stack() {
        let sender = Arc::new(LocalSpace::new_stack());
        let reciever = Arc::clone(&sender);
        thread::spawn(move || {
            space_put!(sender, (4, 'b'));
            space_put!(sender, (5, 'b'));
        });

        let ten_millis = time::Duration::from_millis(100);
        thread::sleep(ten_millis);
        let q = create_template!(4.actual(), 'a'.formal());
        if let Ok(_t) = reciever.getp(q) {
            assert!(false, "Found tuple and should not")
        } else {
            assert!(true);
        }
        let q = create_template!(5.actual(), 'a'.formal());
        let t = reciever.get(q).unwrap();
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());

        let q = create_template!(4.actual(), 'a'.formal());
        let t = reciever.get(q).unwrap();
        assert_eq!(4, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
    }

    #[test]
    fn ordering_pile() {
        let sender = Arc::new(LocalSpace::new_pile());
        let reciever = Arc::clone(&sender);
        thread::spawn(move || {
            space_put!(sender, (4, 'b'));
            space_put!(sender, (5, 'b'));
            space_put!(sender, ('a', 'b'));
        });

        let q = create_template!(5.actual(), 'a'.formal());
        let t = reciever.get(q).unwrap();
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());

        let q = create_template!(4.actual(), 'a'.formal());
        let t = reciever.get(q).unwrap();
        assert_eq!(4, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
    }

    #[test]
    fn repository() {
        let repo = Arc::new(Repository::new());
        let space1 = Arc::new(LocalSpace::new_sequential());
        let space2 = Arc::new(LocalSpace::new_sequential());
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
        let t = space2.get(q).unwrap();
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());

        let q = create_template!(4.actual(), 'a'.formal());
        let t = space1.get(q).unwrap();
        assert_eq!(4, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
    }

    #[test]
    fn multiple_repository() {
        let repo1 = Arc::new(Repository::new());
        let repo2 = Arc::new(Repository::new());
        let space1 = Arc::new(LocalSpace::new_sequential());
        let space2 = Arc::new(LocalSpace::new_sequential());
        let space3 = Arc::new(LocalSpace::new_sequential());
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
        let t = space2.get(q).unwrap();
        assert_eq!(4, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());

        let q = create_template!(5.actual(), 'a'.formal());
        let t = space2.get(q).unwrap();
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());

        let q = create_template!(4.actual(), 'a'.formal());
        let t = space1.get(q).unwrap();
        assert_eq!(4, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());

        let q = create_template!(5.actual(), 'a'.formal());
        let t = space3.get(q).unwrap();
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
    }

    #[test]
    fn repository_delete() {
        let repo = Arc::new(Repository::new());
        let space1 = Arc::new(LocalSpace::new_sequential());
        let space2 = Arc::new(LocalSpace::new_sequential());
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

    #[derive(Serialize, Deserialize, Clone, PartialEq, TupleField)]
    struct TestStruct {
        x: i32,
        y: f64,
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
        let space = LocalSpace::new_sequential();
        let a = 5;
        let b = 'b';
        let fields: Vec<Box<dyn TupleField>> = vec![Box::new(a), Box::new(b)];
        let tuple = Tuple::new(fields);
        space.put(tuple).unwrap();
        let q = create_template!(5.actual(), 'a'.formal());
        let q_json = serde_json::to_string(&q).unwrap();
        print!("{}", q_json);
        let template: Template = serde_json::from_str(&q_json).unwrap();

        let t = space.get(template).unwrap();
        assert_eq!(5, *t.get_field::<i32>(0).unwrap());
        assert_eq!('b', *t.get_field::<char>(1).unwrap());
    }

    #[test]
    fn typing_test() {
        let space = LocalSpace::new_sequential();
        space_put!(space, (5, 7));
        let template = create_template!(5.actual(), 7.actual());
        let tuple = space.query(template).unwrap();
        assert_eq!(5, *tuple.get_field::<i32>(0).unwrap());
        assert_eq!(7, *tuple.get_field::<i32>(1).unwrap());
        let x: i64 = 5;
        let temp2 = create_template!(x.actual(), 7.actual());
        match space.queryp(temp2) {
            Ok(_) => {
                assert!(false, "Should not have found as different data types")
            }
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn message_test() {
        let space = LocalSpace::new_sequential();
        let m = Message {
            action: MessageType::Put,
            tuple: Vec::from([new_tuple!(5, 'b')]),
            template: create_template!(),
        };
        let m_json = serde_json::to_string(&m).expect("should be able to");
        let mut m_from_json: Message = serde_json::from_str(&m_json).expect("please");
        assert_eq!(m_from_json.action, MessageType::Put);
        let tuple = m_from_json.tuple.remove(0);
        assert_eq!(5, *tuple.get_field::<i32>(0).unwrap());
        assert_eq!('b', *tuple.get_field::<char>(1).unwrap());
        space.put(tuple).unwrap();
        let m = Message {
            action: MessageType::Get,
            tuple: Vec::new(),
            template: create_template!(5.actual(), 'a'.formal()),
        };
        let m_json = serde_json::to_string(&m).expect("should be able to");
        let m_from_json: Message = serde_json::from_str(&m_json).expect("please");
        assert_eq!(m_from_json.action, MessageType::Get);
        let template = m_from_json.template;
        let tuple = space.get(template).unwrap();
        assert_eq!(5, *tuple.get_field::<i32>(0).unwrap());
        assert_eq!('b', *tuple.get_field::<char>(1).unwrap());
    }

    #[test]
    fn gate() {
        let repo = Arc::new(Repository::new());
        let space = Arc::new(LocalSpace::new_sequential());
        repo.add_space(String::from("space"), Arc::clone(&space));
        thread::spawn(move || match TcpStream::connect("localhost:3800") {
            Ok(mut stream) => {
                let m = Message {
                    action: MessageType::Get,
                    tuple: Vec::new(),
                    template: create_template!(5.actual(), 'b'.formal()),
                };
                let m_json = serde_json::to_string(&m).unwrap();
                let mut buffer = [0; 1024];
                let spacetext = "space".as_bytes();
                stream.write(spacetext).unwrap();
                stream.flush().expect("should flush");
                let n = stream.read(&mut buffer).unwrap();
                let inc_string = String::from_utf8_lossy(&buffer[..n]);
                assert_eq!("t", inc_string);
                stream.write(m_json.as_bytes()).unwrap();

                let n = stream.read(&mut buffer).unwrap();
                let inc_string = String::from_utf8_lossy(&buffer[..n]);
                let mut message = serde_json::from_str::<Message>(&inc_string).unwrap();
                let tuple = message.tuple.remove(0);
                assert_eq!(5, *tuple.get_field::<i32>(0).unwrap());
                assert_eq!('b', *tuple.get_field::<char>(1).unwrap());
            }
            Err(e) => {
                assert!(false, "{}", e);
            }
        });
        space_put!(space, (5, 'b'));
        Repository::add_gate(
            repo,
            String::from("gate"),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3800),
        )
        .expect("could not connect");
        loop {
            let q = create_template!(5.actual(), 'b'.formal());
            let t = match space.queryp(q) {
                Ok(t) => t,
                Err(_) => break,
            };
            assert_eq!(5, *t.get_field::<i32>(0).unwrap());
            assert_eq!('b', *t.get_field::<char>(1).unwrap());
        }
    }
}
