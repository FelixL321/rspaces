# Concurrent usage

Distributed hello world example [Here](./../examples/hello_distributed.rs)

## Repository
For bundling together spaces you can use a repository. You can put multiple spaces in a repository and then send a reference to that repo to another thread or open up a gate for remote use. Repositories can be created like bellow
```rust
let repo = Arc::new(Repository::new());
let sequential = Arc::new(LocalSpace::new_sequential());
let stack = Arc::new(LocalSpace::new_stack());

repo.add_space(String::from("sequential"), Arc::clone(&sequential));
repo.add_space(String::from("stack"), Arc::clone(&stack));
```

## Gate
Enabling distributed computing with rspaces requires opening up a gate for others to connect to. Gates are added to a single repository, meaning only a socket can only be used for one repository at a time. This is unlike jspaces that allows for a single gate to be used with multiple repositories. rspaces still provides with the option for having several gates for the same repository though.

Adding a gate to repository is demonstrated bellow
```rust
let repo = Arc::new(Repository::new());
Repository::add_gate(
    Arc::clone(&repo),
    String::from("gate"),
    "127.0.0.1:3800".to_string(),
);
```
The first argument denotes the repository for which the gate should be added, the second argument is an identifier for later closing the repository, and the last argument is the address for the socket to use.

### Closing Gates
It is important to remember to close gates, as it will otherwise sometimes result in bad behavior for clients connected to the gate. Closing gates is straight forward by using the previously defined identifier

```rust
repo.close_gate(String::from("gate"));
```
