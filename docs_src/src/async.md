# Async

Using an `async` main function and `next_frame().await` allows SGE to run coroutines.

## Coroutines

A coroutine is a function that can pause execution and resume later. Coroutines do not return a value. They run alongside the main loop on the same thread and update every frame. 

When a coroutine calls `.await` on a future, it pauses. At the end of the frame, SGE updates all active coroutines. If the future is ready, the coroutine continues executing.

```rust
use sge::*;

#[main("Title")]
async fn main() -> anyhow::Result<()> {
    // Start a coroutine from an async function
    let coroutine = start_coroutine(count());

    loop {
        if coroutine.is_done() {
            draw_text("Done", Vec2::ZERO);
        }

        if should_quit() {
            break;
        }

        // next_frame().await updates engine systems and advances running coroutines
        next_frame().await;
    }

    Ok(())
}

// Draws numbers from 0 to 99, incrementing each frame
async fn count() {
    for i in 0..100 {
        draw_text(i, Vec2::ZERO);

        // Pauses this function until the next frame
        next_frame().await;
    }
}
```

You can use coroutines for tasks that span multiple frames, like cutscenes or timed events. Track the status of a coroutine using coroutine.is_done().

See: [`/examples/coroutines.rs`](https://github.com/LilyRL/sge/blob/master/examples/coroutines.rs)

## Other async functions

SGE also has some other async functions worth knowing about, such as ones to wait
a certain amount of time, and for [loading resources](https://lilyrl.github.io/sge/fs.html) from disk/bytes in the
background so it doesn't interrupt the user by freezing the program while it's loading.

```rust
// Pause for 2.5 seconds
wait_for(2.5).await;

// Pause for 10 frames
wait_for_frames(10).await;
```

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

`init` creates the window and sets everything up. `run_async` sets up an asynchronous
environment for your code to run in, and makes sure it is updated once per frame.
