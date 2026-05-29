# Input API

The input API is simple, SGE provides functions for querying the current
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

See: [input module documentation](https://docs.rs/sge/latest/sge/prelude/input/index.html) for more detail.

## Action mapping

There is support for creating named actions, and binding them to keys/mouse
buttons. You can then use equivalent functions to check if they are pressed/released/held.

Actions are most easily created using the `actions!` macro.

```rust
actions! {
    FWD, BACK, RIGHT, LEFT, JUMP
}

// Generated code:
// const FWD: Action = Action::new(0);
// const BACK: Action = Action::new(1);
// const RIGHT: Action = Action::new(2);
// const LEFT: Action = Action::new(3);
// const JUMP: Action = Action::new(4);
```

You can then bind an action to a key by using the `bind` function.

```rust
bind(FWD, KeyCode::KeyW);
bind(BACK, KeyCode::KeyS);
bind(RIGHT, KeyCode::KeyD);
bind(LEFT, KeyCode::KeyA);
bind(JUMP, KeyCode::Space);
```

And use them like any other button.

```rust
if action_pressed(FWD) {
    // move forward
}
```

This makes it easier for you to allow the player to change their preferred
controls for your game, by binding them to different keys, without you needing
to change the rest of your codebase.

---
   
See: [input module documentation](https://docs.rs/sge/latest/sge/prelude/input/index.html) for more detail.

See also: [`/examples/action_mapping.rs`](https://github.com/LilyRL/sge/blob/master/examples/action_mapping.rs)

See also: [clipboard module documentation](https://docs.rs/sge/latest/sge/prelude/clipboard/index.html)
