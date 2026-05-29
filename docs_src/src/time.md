# Time

You can use `time()` to get the time the program has been running in seconds,
and `delta_time()` to get the time since last frame in seconds.

There is also `physics_time()` and `physics_delta_time()` which uses the
'physics timer'. The physics timer can be sped up, slowed down, and paused using
`set_physics_speed`, `pause_physics_timer` and `play_physics_timer`. This
isn't related to the physics system, and wont speed it up.

There are also some convenience methods you can use, like `once_per_second` or
`once_per_n_seconds` which only returns true once every some timeframe, and
`oscillate` which moves between two values once per second, `oscillate_t` lets
you provide your own time value, so you can speed it up or whatever.

---

See: [time module](https://docs.rs/sge/latest/sge/prelude/time/index.html) for
full list of functions
