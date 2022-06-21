# rSpaces

## Installation
Include the rspaces dependency in your Cargo.toml file
```cargo
[dependencies]
rspaces = { git = "https://github.com/FelixL321/rspaces" }
```

### Plugins
Install the rust-analyzer plugin for your favorite IDE/editor for help with rust syntax

## Usage
To create a new space use the constructor for one of the space types.
```rust
let space_sequential = LocalSpace::new_sequential();
let space_pile = LocalSpace::new_pile();
```

rspaces provide 2 ways of creating new tuples. The first includes making a new instance of a fields vector and then converting it to a tuple:
```rust
let fields: Vec<Box<dyn TupleField>> = vec![Box::new(1), Box::new('a')];
let tuple = Tuple::new(fields);
```
rspaces however also provides you a shorthand macro for the same:


## Differences to jSpaces
rspaces only provide support for ipv4 over tcp in case of remote spaces. This results in the gate/remote_space syntax a bit different. The following creates a new gate for a repository.



```rust
let connection_string = "127.0.0.1:31415".to_string();
let repo = Arc::new(Repository::new());
Repository::add_gate(Arc::clone(&repo), String::from("gate1"), connection_string);
```

And the following will create a new remote space connecting to the above space.
```rust
let connection_string = "127.0.0.1:31415/space1".to_string();
let remote_space = RemoteSpace::new(connection_string);
```



## Known issues
Using rustc v1.58-v1.60 will result in error messages for library macros when other errors is found by the compiler. To remove the errors, either fix the other errors, or update rust to atleast v1.61