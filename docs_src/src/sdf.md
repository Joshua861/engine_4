# Advanced Shapes

Shapes that cannot be represented by vertices (i.e.: have smooth edges, like
circles), are drawn using [signed distance
functions](https://en.wikipedia.org/wiki/Signed_distance_function), this means
that they are drawn exactly, no matter how much you zoom in.

You can access the signed distance function object directly to specify
additional effects like a drop shadow, corner radius, or a patterned fill.

For example:

```rust
let object = Sdf::pentagram(c, 50.0)
    .with_fill(Color::RED_500, Color::RED_400, 0.0, 5.0, SdfFill::Grid)
    .with_corner_radius(5.0)
    .with_stroke(2.0, Color::RED_200, SdfStroke::Outside)
    .with_shadow(vec2(10.0, 10.0), 10.0, Color::NEUTRAL_900);

draw_sdf(object);
```

See: [`/exampes/sdf.rs`](https://github.com/LilyRL/sge/blob/master/examples/sdf.rs)
