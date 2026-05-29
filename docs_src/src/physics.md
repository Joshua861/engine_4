# Physics API

The physics API is based on [`rapier2d`](https://docs.rs/rapier2d/latest/rapier2d/).

Create a [`PhysicsWorld`](https://docs.rs/sge/latest/sge/prelude/physics/struct.PhysicsWorld.html) with `PhysicsWorld::new()`. This returns a `WorldRef` that you call `.update()` on each frame to step the simulation.

Rapier2D uses positive y up, but SGE uses positive y down, so coordinates
converted before being returned, so that the position in the world can be the
same as the position onscreen. A conversion rate of 100 pixels per Rapier2D
meter is used.

```rust
#[main("Physics")]
fn main() {
    let mut world = PhysicsWorld::new();

    loop {
        world.update();

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
```

## Creating Objects

There are three types of physics objects:

- **Dynamic**: affected by gravity and forces, collides with everything.
- **Fixed**: immovable, used for walls, floors, and static geometry.
- **Kinematic**: moved manually (e.g. via `set_position`), but participates in collision detection.

```rust
let dynamic_obj = world.create_dynamic(Bounds::Circle(20.0));
let wall = world.create_fixed(Bounds::Rect(Vec2::new(1000.0, 50.0)));
let platform = world.create_kinematic(Bounds::Rect(Vec2::new(200.0, 20.0)));
```

Each of these also has a `_with` variant that accepts a `ColliderConfig` for customizing physical material properties:

```rust
let bouncy = world.create_dynamic_with(
    Bounds::Circle(15.0),
    ColliderConfig::default()
        .restitution(0.9)
        .friction(0.1)
        .density(2.0),
);
```

Objects can be removed with `.remove()`.

Once created, objects can be manipulated using the methods on the `ObjectRef` struct.

```rust
let obj = world
    .create_dynamic(Bounds::Circle(15.0))
    .with_position(Vec2::new(200.0, 100.0))
    .with_velocity(Vec2::new(150.0, 0.0))
    .with_ccd(); // continuous collision detection

// Reading state
let pos = obj.get_position();
let vel = obj.get_velocity();
let rot = obj.get_rotation(); // all rotations are in radians
let mass = obj.get_mass();

obj.set_position(Vec2::new(300.0, 200.0));
obj.set_velocity(Vec2::new(0.0, -100.0));
obj.set_rotation(std::f32::consts::PI * 0.25);

obj.add_velocity(Vec2::new(50.0, 0.0));
obj.add_force(Vec2::new(0.0, -500.0));
obj.move_by(Vec2::new(5.0, 0.0));

obj.set_angvel(2.0); // rad/s
obj.add_angvel(0.5);
```


## Bounds

`Bounds` describes the shape of a collider. The available variants are:

- `Bounds::Circle(radius)`
- `Bounds::Rect(Vec2)`: full size from center
- `Bounds::Capsule { half_height, radius }`: vertical capsule
- `Bounds::CapsuleX { half_width, radius }`: horizontal capsule
- `Bounds::Triangle(a, b, c)`: relative to the object's position
- `Bounds::ConvexHull(points)`: computed from a point cloud
- `Bounds::Polyline(points)`: a series of connected line segments, useful for terrain
- `Bounds::Line { a, b }`
- `Bounds::Compound(children)`: multiple shapes combined, each with an offset

## Sensors

Sensors are not physical objects, other objects will pass right through them,
but they still check if something is colliding with them.

```rust
let sensor = world
    .create_fixed_with(Bounds::Circle(80.0), ColliderConfig::default().sensor(true))
    .with_position(Vec2::new(400.0, 300.0));

if sensor.is_colliding() {
    // something is inside the sensor
}
```

## Collisions

```rust
if obj.is_colliding() { ... }

if obj.is_colliding_with(other) { ... }

if let Some(points) = obj.check_collision_with(other) {
    let normal = points.normal; 
    let depth = points.depth; 
}

for info in obj.collisions() {
    let other: ObjectRef = info.other;
    let normal: Vec2 = info.points.normal;
    match info.event {
        CollisionType::Started => { /* wasn't colliding last frame, colliding now */  }
        CollisionType::Ongoing => { /* colliding last frame, still colliding now */  }
        CollisionType::Stopped => { /* was colliding last frame, not anymore */ }
    }
}
```

## World Settings

```rust
world.set_gravity(980.0);  // in pixels/world units, so 9.8 will be very slow
let g = world.get_gravity();
```

## Debug Visualization

You can draw outlines of all colliders and collision normals for debugging:

```rust
world.draw_colliders();
// or
world.draw_colliders_world();
```

Colliders are drawn in red, objects currently in collision are highlighted in yellow with arrows showing the collision normals.

---
   
See: [physics module](https://docs.rs/sge/latest/sge/prelude/physics/index.html)

See also: [`/examples/physics.rs`](https://github.com/LilyRL/sge/blob/master/examples/physics.rs)
