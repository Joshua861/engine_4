# Creating a window

The most basic program using the engine looks like this.

```rs
use engine_4::prelude::*;

fn main() -> anyhow::Result<()> {
    init("Title for the window")?;
    
    loop {
        if should_quit() {
            break;
        }
        
        next_frame();
    }
    
    Ok(())
}
```

We include everything in the Engine's prelude module, which is probably most of
the stuff you'll need from the engine.

Some of the functions of the engine, such as creating a new texture or loading a
model can fail. You can handle these errors however you like, but for now we are
just returning and exiting the program if we meet one (this can be seen on the
init function (a ? returns the function if the result is Err)). For the moment
the engine mostly uses anyhow Results, which can contain any error type, this
will hopefully be replaced for concrete error types soon.

I wanted to make the structure of your code required as simple as possible. You
just write it like a normal function and you do any initialization at the top,
outside the loop, instead of in some init() function implemented on a struct.

The `should_quit` function tells you if the user has asked for the window to be
closed/process to quit. You can probably do whatever you want here, like ask the
user for conformation they want to quit before breaking out of the loop.

Next frame renders all your stuff to the screen and then waits until the next
frame should begin so that you may process and render the next frame.

The reference documentation for the engine can be found by running `cargo doc`
in the root folder of the engine source code, or on [docs.rs](https://docs.rs/engine_4/latest/engine_4/).
