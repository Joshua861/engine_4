# Persistence API

When making a game or application you will inevitably want to store some kind of
state between launches, so that you aren't starting from scratch each time you
run the program. SGE provides a simple high-performance interface to make this easier.

To use the persistence API, create a state struct, and annotate it with the
`#[persistent]` attribute macro.

```rust
#[persistent]
struct State {
    score: u64,
    player_color: Color,
    enemy_state: EnemyState,
}

#[persistent]
struct EnemyState {
    health: f32,
    position: Vec2,
}
```

The persistent macro will generate the functions `state.save(path)`,
`State::load(path)`, `state.to_bytes()`, and `State::from_bytes(bytes)`, for any
struct that has fields that are all also serializable.

Serializable types include:
- Most builtin Rust types, numbers, strings, etc.
- Anything supported by `rkyv` out of the box.
- Most of the SGE types you would want to use this with
    - Vectors
    - Both colour types
- Any other custom struct you write, as long as it is also decorated with `#[persistent]`
    
## Performance

This uses [`rkyv`](https://docs.rs/rkyv/latest/rkyv/) under the hood, which
performs zero-copy serialization, meaning that it is very fast, so you can save
often if you want. Do not save/load every frame.


---

See [`Persistent` trait](https://docs.rs/sge/latest/sge/prelude/persistence/triat.Persistent.html)
   
See: [`/examples/persistence`](https://github.com/LilyRL/sge/blob/master/examples/persistence.rs)
