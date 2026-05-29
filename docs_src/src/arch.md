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
        // if you don't include this, it will be cleared to black by default.
        // if you don't want to clear the screen, use `dont_clear_screen()` instead
        clear_screen(Color::BLACK)
    
        // frame loop here
        // do anything you want to run once per frame, like drawing some shapes
        
        if should_quit() {
            // you can do whatever you want here
            // i just find it makes more sense to break out of the loop
            // and do the cleanup at the bottom of the function
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
