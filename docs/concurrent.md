# Concurrent usage

Concurrent hello world example [Here](./../examples/hello_concurrent.rs)

## Arc
When dealing with multiple threads you have to wrap your structs in an Arc which is a threadsafe reference counter. For rspaces this means that you have to put your spaces in an arc before sending them to another thread.
```rust
    //Creating new space
    let space = Arc::new(LocalSpace::new_sequential());

    //Creating a clone of the reference counter so that it can be send to multiple threads
    let spaceclone = Arc::clone(&space);

    //Spawning new thread
    thread::spawn(move || {
        do_space_actions(spaceclone);
    });
```
