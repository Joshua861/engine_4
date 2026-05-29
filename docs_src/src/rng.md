# RNG

Most games need random numbers. You can generate them with these functions:

- `get_next_counter`: returns 0 the first time you call it, returns the number
one more than the last time you called it on all following times
- `rand`: generic function that returns a random value of the type you pass it, as
long as that type supports random generation.
- `rand_bool`: Return a bool with a probability p of being true.
- `rand_choice`: returns a random element from an array, if the array is empty it panics.
- `maybe_rand_choice`: returns none if choices are empty, returns random choice otherwise
- `rand_color`: returns a random color
- `rand_f32`: generates f32 between -1 and 1
- `rand_range`: generates a random number between two values
- `rand_ratio`: return a bool with a probability of numerator/denominator of being true.
- `rand_usize`: returns a random usize
- `rand_vec2`: returns a random vec2
- `rand_vec3`: returns a random vec3
- `rand_vec4`: returns a random vec4

- `id!()`: returns a constant random number. baked at compile time, won't change
  each time it is run, but will change if it is used in multiple places. This
  will make more sense if you know how macros work in Rust.
  
  ```rust
  id!() != id!();
  
  let mut numbers = vec![];
  for _ in 0..5 {
      numbers.push(id!());
  }
  
  // every number in numbers is equal
  ```

---

See: [rng module](https://docs.rs/sge/latest/sge/prelude/rng/index.html)
