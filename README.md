# rSpaces

## Installation
Include the rspaces dependency in your Cargo.toml file
```cargo
[dependencies]
rspaces = { git = "https://github.com/FelixL321/rspaces" }
```

### Plugins
Install the rust-analyzer plugin for your favorite IDE/editor for help with rust syntax and to provide with tooltips when hovering over library functions

## Usage
1. [Basic usage](./docs/basic.md)
2. [Concurrent usage](./docs/concurrent.md)
3. [Distributed usage](./docs/distributed.md)
4. [Seriliazation](./docs/serilization.md)

## Documentation
For more in depth documentation you can clone the github repository and run `cargo doc` and then open up the generated html in your browser for a docs.rs site.

## Differences to jSpaces
 - rspaces only provide support for ipv4 over tcp in case of remote spaces.
 - Gates only work for one repository meaning only one repository can communicate per socket.


## Future work
- Add better encapsulation
- Put on crates.io for easier access and documentation
- Allow gates to be used for more than one repository
- More macros for reducing boiler plate code

## Known issues
Introducing errors to your code will result in the compiler marking the `new_tuple!()` macro and the `.formal()` and `.actual()` methods with errors as well if using any form of integer. 