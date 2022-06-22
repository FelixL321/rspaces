# Basic usage


Basic hello world example [Here](./../examples/hello_world.rs)

## Tuple
rspaces provide 2 ways of creating new tuples. The first includes making a new instance of a fields vector and then converting it to a tuple:
```rust
let fields: Vec<Box<dyn TupleField>> = vec![Box::new(1), Box::new('a')];
let tuple = Tuple::new(fields);
```
rspaces however also provides you a shorthand macro for the same:
```rust
let tuple = new_tuple!(1, 'a');
```
Only valid tuplefields can be put into a tuple. Currently this includes all basic types except string slices `&str` (You can still use `String`)

The following shows how to get a field out of a tuple
```rust
let tuple = new_tuple!(1, 'a');
let a = tuple.get_field::<char>(1);
```

You have to provide the expected type of the element at the position you are getting. This function will panic if the index is not valid or if the type does not match the actual type of the element

## Space
To create a new space use the constructor for one of the space types.
```rust
let space_sequential = LocalSpace::new_sequential();
let space_queue = LocalSpace::new_queue();
let space_stack = LocalSpace::new_stack();
let space_pile = LocalSpace::new_pile();
let 
```
To put a tuple into the space use the following:

```rust
space.put(tuple);
```

## Template
rspaces also comes with a macro for creating templates. First however you need to define the types of the template fields. Here rspaces provides `.formal` and `.actual` for all valid tuplefields. 

```rust
//Formal templatefield that just matches on the type
let formal = 'a'.formal()

//Actual templatefield that matches on type and content
let actual = 'a'.actual()

//Create the template
let tuple = new_template!(formal, actual);

//Or do it on a single line.
let tuple = new_template!('a'.formal(), 'a'.actual());
```

## Get/Query
rspaces provide the standard api for getting/querying tuples with a template `t` in a space:

```rust
space.get(t);
space.getp(t);
space.query(t);
space.queryp(t);
space.getall(t);
space.queryall(t);
```

