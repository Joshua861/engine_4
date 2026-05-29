# Window

There are [numerous
functions](https://docs.rs/sge/latest/sge/prelude/window/index.html) for
querying and changing the properties of the window, for example if it is
in fullscreen or how big it is in pixels.

See: [window module documentation](https://docs.rs/sge/latest/sge/prelude/window/index.html)

## Cursor icons

You may set the current cursor icon to use with one of [these
functions](https://docs.rs/sge/latest/sge/prelude/cursor_icons/index.html), it
will only last for one frame, so you should re-set the cursor icon every frame.

```rust
// in frame loop/function called every frame
if is_hovered {
    use_pointer_cursor_icon();
}
```
