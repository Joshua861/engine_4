# Storage API

Games often have a lot of unrelated systems running in parallel. This can lead
to lots of confusing code to manage the state of all the different parts of your
code, and extra effort to make state availible to the functions that need to
read/mutate it.

In complex projects, you may choose to use the storage API to mitigate this
complexity, at the cost of it being less clear what parts of the code could be
mutating state.

The storage API lets you store and retrieve custom state structs from a global
store. Just create a unique state type, and use `storage_init_state` to store
it, and `storage_get_state` and `storage_get_state_mut` to retrieve it. 

```rust
struct MyState {
    score: usize,
}

fn main() {
    // ...
    
    let state = MyState { score: 0 };
    storage_store_state(state);
    
    // ...
}

fn show_score(pos: Vec2) {
    let state = storage_get_state::<MyState>();
    draw_text(state.score.to_string(), pos);
}
```

There are some other functions you may want to do with storage listed [here](https://docs.rs/sge/latest/sge/prelude/storage/index.html).

---

See: [storage module](https://docs.rs/sge/latest/sge/prelude/storage/index.html)
