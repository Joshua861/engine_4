# Textures

Textures can be loaded using one of the following:

- `include_texture!`: Bake bytes of image file into binary, and load them from
  that. This comes with the benefit of working without using any outside files
  from the executable, meaning you only need to give someone one file to play
  your game.
- `load_texture_sync`: synchronously load texture from file path.
- `load_texture_from_bytes_sync`: synchronously load texture from bytes.
- `load_texture`: asynchronously load texture from file path.
- `load_texture_from_bytes`: asynchronously load texture from bytes.

All of these functions return a `TextureRef`, which is just a wrapper around an
integer, and can be passed around/copied at almost 0 cost. This reference is
also guaranteed to always be valid, so long as you don't use any of the `unsafe`
functions associated with `Ref` types.

Textures can be drawn by using one of the following:

- `draw_texture(_world)`: simply draws a texture at some position at some scale.
- `draw_texture_scaled(_world)`: allows you to draw the texture at any scale,
  without respecting the original aspect ratio.
- `draw_texture_ex`: contains additional options like an arbitrary transform,
  tint, and the option to only draw a region of the whole texture.
  
See: [texture module documentation](https://docs.rs/sge/latest/sge/prelude/textures/index.html)
