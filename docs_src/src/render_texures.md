# Render Textures

You may for whatever reason want to render to a texture instead of the screen,
you can do this with render textures, which are just a collection of a normal color texture and depth
texture.

You can create a texture with `create_empty_render_texture()`.

```rust
let size = UVec2::new(50, 50);
let render_texture = create_empty_render_texture(size.x, size.y)?;

loop {
    clear_screen(Color::BLACK);
    
    // any drawing functions run after this function will draw to the texture
    // instead of the screen. you don't need to pass anything extra into them.
    // be careful to always call end_rendering_to_texture().
    // i would recommend to reduce the chance of bugs that you never call end_rendering
    // in a different part of the code as start.
    start_rendering_to_texture(render_texture);
    
    // actually clearing texture
    clear_screen(Color::WHITE);
    draw_square(vec2(10.0, 10.0), 50.0, Color::SKY_500);
    
    end_rendering_to_texture();
    
    if should_quit() {
        break;
    }

    next_frame().await;
}
```

Some use cases to consider:

- Using a lower resolution for a retro-effect, but with better performance than
  drawing at full resolution and then using `pixelate_screen()`.
- Splitscreen multiplayer, draw once for each player to textures, and then
  position them on the screen.
