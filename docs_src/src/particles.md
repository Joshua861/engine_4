# Particle System

Create a particle system with `ParticleSystem::new()`, and spawn emitters or one
shot particle spawners when needed, then run `.update()` and `.draw()` or
`.draw_world()` once per frame to see the particles onscreen. You can customize
lots of parameters to change how the particles act.

```rust
#[main("Particles")]
fn main() {
    let mut particles = ParticleSystem::new();
    let batch = ParticleOneshot::builder()
        .shape(&Rect::new_square(Vec2::ZERO, 20.0, Color::YELLOW_500))
        .size_randomness(5.0)
        .color_randomness(Color::new(0.3, 0.1, 0.1))
        .direction_randomness(0.5)
        .speed(40.0)
        .speed_randomness(3.0)
        .rotation_speed_randomness(0.2)
        .end_color(Color::RED_700)
        .acceleration(vec2(0.0, 1.0))
        .acceleration_randomness(vec2(0.0, 0.2))
        .lifetime(2.0)
        .quantity(100)
        .build();

    loop {
        if should_spawn_particles {
            particles.spawn_oneshot(&batch, position)
        }
    
        particles.update();
        particles.draw();

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
```

---
   
See: [particles module](https://docs.rs/sge/latest/sge/prelude/particles/index.html)

See: [`/examples/particles.rs`](https://github.com/LilyRL/sge/blob/master/examples/particles.rs)

See also:
[`/examples/space_game.rs`](https://github.com/LilyRL/sge/blob/master/examples/space_game.rs)
for a more complex example of how particles can be used.
