# Audio

Sounds can be loaded using one of the following:

- `include_sound!`: Bake bytes of sound file into binary, and load them from
  that. This comes with the benefit of working without using any outside files
  from the executable, meaning you only need to give someone one file to play
  your game.
- `load_sound_sync`: synchronously load sound from file path.
- `load_sound_from_bytes_sync`: synchronously load sound from bytes.
- `load_sound`: asynchronously load sound from file path.
- `load_sound_from_bytes`: asynchronously load sound from bytes.

All of these functions return a `SoundRef`, which is just a wrapper around an
integer, and can be passed around/copied at almost 0 cost. This reference is
also guaranteed to always be valid, so long as you don't use any of the `unsafe`
functions associated with `Ref` types.

Sounds can be played in one of two ways:

1. `play_sound`: simply plays a sound reference without blocking.
1. `play_sound_ex`: plays a sound and returns a `SoundBuilder` object, allowing
   you to add effects to the sound before playing with `.start()`.
   
   Here is an example of how you might use this:
   
   ```rust
   play_sound_ex(sound)
       .fade_in(Duration::from_millis(800))
       .volume(0.5)
       .start();
   ```
   
   You can find a list of effects in the [`SoundBuilder` documentation](https://docs.rs/sge/latest/sge/prelude/audio/struct.SoundBuilder.html).
   
See: [`/examples/simple_sound.rs`](https://github.com/LilyRL/sge/blob/master/examples/simple_sound.rs)

See: [sound module documentation](https://docs.rs/sge/latest/sge/prelude/audio/index.html)
