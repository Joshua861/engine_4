# Architecture

SGE was designed to be flexible and easy to use, even for someone
unfamiliar with Rust, for this reason it does not impose any set structure on
how you must organize your code.

The most basic SGE project is a single file with this structure:

```rust
use sge::*;

#[main("Title for the window")]
async fn main() {
    // do initialization here

    loop {
        // frame loop here
        
        if should_quit() {
            break;
        }
        
        next_frame().await;
    }
    
    // do cleanup here
}
```

For more complex apps it may make more sense to store state in a struct, created
at the start of the main function, and then have a frame loop comprised of just `state.update()`.

The main function can optionally return a result (anyhow is included, use
`anyhow::Result<()>`), which will be unwrapped.

Most functions that return a large amount of data (for example loading a
texture or sound) actually return a reference to the data, so you can pass around your
`TextureRef` (for example), and clone/copy it without worrying about the
performance cost.

We will talk more about why async is needed, what the `#[main]` macro is for,
and how to initialize the engine with custom parameters later.
