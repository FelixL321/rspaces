# rSpaces
# Introduction

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

## Differences to jSpaces
rspaces only provide support for ipv4 over tcp in case of remote spaces. This results in the gate/remote_space syntax a bit different. The following creates a new gate for a repository.



```rust
let connection_string = "127.0.0.1:31415".to_string();
repo.add_space(String::from("space1"), Arc::clone(&space));
Repository::add_gate(Arc::clone(&repo), String::from("gate1"), conn_string);
```

And the following will create a new remote space connecting to the above space.
```rust
let connection_string = "127.0.0.1:31415/space1".to_string();
let remote_space = RemoteSpace::new(connection_string);
```



## Known issues
Using rustc v1.58-v1.60 will result in error messages for library macros when other errors is found by the compiler. To remove the errors, either fix the other errors, or update rust to atleast v1.61