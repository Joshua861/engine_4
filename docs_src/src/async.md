# Async

You may have been wondering why it is necessary to mark your main function as
`async` and use `next_frame().await`, but this allows us to use coroutines.

## Coroutines

A coroutine is, in this context, a function that can be paused and resumed
later. It is a type of async function, but coroutines never return a value.
They run alongside the rest of your program, on the same thread, and are updated
every frame. If a coroutine calls `.await` on a future, it's execution will be
paused. At the end of the next frame, SGE will check if the future it is waiting
on is ready, and if it is, execution of that function will continue.

This may sound very confusing, but here's an example.

```rust
use sge::*;

#[main("Title")]
async fn main() -> anyhow::Result<()> {
    // here we create a coroutine from a normal `async` function and start it
    let coroutine = start_coroutine(count());

    loop {
        if coroutine.is_done() {
            draw_text("Done", Vec2::ZERO);
        }

        if should_quit() {
            break;
        }

        // when we get to this point in the program, the engine runs everything
        // it needs to do once per frame, like drawing things to the screen, but
        // before that, it loops through all the running coroutines and updates them
        next_frame().await;
    }

    Ok(())
}

// this just draws the number 1 to 100, increasing by 1 every frame, until it finishes
async fn count() {
    for i in 0..100 {
        draw_text(i, Vec2::ZERO);

        // this next_frame future just makes the function wait until the next frame to
        // keep executing. you'll notice it acts exactly the same as in the main function.
        // you can treat these two functions as though they are running in parallel
        next_frame().await;
    }
}
```

This can be used for all sorts of things, for example a cutscene, that you want
to run just until it's done, and then move onto something else. You can check
the status of the spawned coroutine with `coroutine.is_done()`.

See: [`/examples/coroutines.rs`](https://github.com/LilyRL/sge/blob/master/examples/coroutines.rs)

## Other async functions

SGE also has some other async functions worth knowing about, such as ones to wait
a certain amount of time, and for [loading resources](https://lilyrl.github.io/sge/fs.html) from disk/bytes in the
background so it doesn't interrupt the user by freezing the program while it's loading.

See: [Exec module](https://docs.rs/sge/latest/sge/prelude/exec/index.html)

## The main macro

The `#[main()]` macro is just a helper that makes it more simple to initialize
the engine. All it does is replace:

```rust
#[main("Window title")]
async fn main() {
    // do stuff
}
```

With:

```rust
fn main() {
    sge::init("Window title").unwrap();
    
    sge::run_async(async {
        // do stuff
    });
}
```

`init`, obviously, initializes SGE. `run_async` sets up an asynchronous
environment for your code to run in, and makes sure it is updated once per frame.
