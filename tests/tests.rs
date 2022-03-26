#[cfg(test)]
mod tests {
    use rspaces::{SequentialSpace, Space, Tuple};
    use std::any::Any;

    #[test]
    fn tuple_test() {
        let a = 5;
        let b = 'b';
        let fields: Vec<Box<dyn Any>> = vec![Box::new(a), Box::new(b)];
        let tuple = Tuple::new(fields);

        let x = tuple.get_field::<i32>(0).expect("could not cast");
        assert_eq!(5, *x);
        let x = tuple.get_field::<char>(1).expect("could not cast");
    }

    #[test]
    fn tuple_test_failing() {
        let a: i32 = 5;
        let b: char = 'b';
        let fields: Vec<Box<dyn Any>> = vec![Box::new(a), Box::new(b)];
        let tuple = Tuple::new(fields);

        match tuple.get_field::<u64>(0) {
            Some(x) => {
                assert!(false, "Got something, when we should not");
            }
            None => assert!(true),
        }
    }
}
