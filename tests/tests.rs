#[cfg(test)]
mod tests {
    use rspaces::{SequentialSpace, Space, Tuple, Query, Queries};
    use std::any::Any;
    use super::*;

    #[test]
    fn tuple_test() {
        let a = 5;
        let b = 'b';
        let fields: Vec<Box<dyn Any>> = vec![Box::new(a), Box::new(b)];
        let tuple = Tuple::new(fields);

        let x = tuple.get_field::<i32>(0).expect("could not cast");
        assert_eq!(5, *x);
        let _x = tuple.get_field::<char>(1).expect("could not cast");
    }

    #[test]
    fn tuple_test_failing() {
        let a: i32 = 5;
        let b: char = 'b';
        let fields: Vec<Box<dyn Any>> = vec![Box::new(a), Box::new(b)];
        let tuple = Tuple::new(fields);

        match tuple.get_field::<u64>(0) {
            Some(_x) => {
                assert!(false, "Got something, when we should not");
            }
            None => assert!(true),
        }
    }

    #[test]
    fn space_search(){
        let mut space = SequentialSpace::new();
        let a = 5;
        let b = 'b';
        let fields: Vec<Box<dyn Any>> = vec![Box::new(a), Box::new(b)];
        let tuple = Tuple::new(fields);
        space.put(tuple);
        let mut q = Query::new();
        q.fields.push(5.actual());
        q.fields.push(char::formal());
        if let Some(t) = space.get(q){
            assert!(true);
        }else{
            assert!(false,"Didnt find tuple...");
        }

    }

    #[test]
    fn space_search_failing(){
        let mut space = SequentialSpace::new();
        let a = 5;
        let b = 'b';
        let fields: Vec<Box<dyn Any>> = vec![Box::new(a), Box::new(b)];
        let tuple = Tuple::new(fields);
        space.put(tuple);
        let mut q = Query::new();
        q.fields.push(5.actual());
        q.fields.push(i32::formal());
        if let Some(t) = space.get(q){
            assert!(false, "We found touple and we should not");
        }else{
            assert!(true);
        }

    }
}

