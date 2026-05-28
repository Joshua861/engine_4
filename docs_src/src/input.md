# Input

The input API is very simple, SGE provides functions for querying the current
state of keyboard and mouse buttons.

If you want to run some code whenever the space bar is pressed, you could use
this code inside of your frame loop:

```rust
if key_pressed(KeyCode::Space) {
    // do whatever
}
```

There are also functions for mouse buttons, and keys being held and released:

```rust
key_held(KeyCode::KeyA);
mouse_released(MouseButton::Left);
```

You can also query the position of the cursor:

```rust
pub fn cursor() -> Option<Vec2>; // current position of the cursor, if it is in the window
pub fn last_cursor_pos() -> Vec2; // if the mouse cursor is outside the window, return it's last position
pub fn cursor_diff() -> Vec2; // how much the cursor moved
```

See: [input module
documentation](https://docs.rs/sge/latest/sge/prelude/input/index.html) for more detail.
