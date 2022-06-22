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


## Differences to jSpaces
 - rspaces only provide support for ipv4 over tcp in case of remote spaces.
 - Gates only work for one repository meaning only one repository can communicate per socket.


## Future work
- Add better encapsulation
- Put on crates.io for easier access and documentation
- Allow gates to be used for more than one repository
- More macros for reducing boiler plate code

## Known issues
Using rustc v1.58-v1.60 will result in error messages for library macros when other errors is found by the compiler. To remove the errors, either fix the other errors, or update rust to atleast v1.61