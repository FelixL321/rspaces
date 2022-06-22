# Basic usage
To create a new space use the constructor for one of the space types.

Basic hello world example [Here](./../examples/hello_world.rs)

## Space
```rust
let space_sequential = LocalSpace::new_sequential();
let space_pile = LocalSpace::new_pile();
```
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

## Template
rspaces comes with two constructors and a macro for creating templates. First you need to define the types of the template fields. Here rspaces provides `.formal` and `.actual` for all valid tuplefields. 

```rust
//Formal templatefield that just matches on the type
let formal = 'a'.formal()

//Actual templatefield that matches on type and content
let actual = 'a'.actual()
```

```rust
let tuple = new_template!(1.formal(), 'a'.actual());
```