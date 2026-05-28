# Drawing Shapes

There are two main spaces that shapes can be drawn to.

1. The screen: coordinates are relative to the top left of the screen, and one
   unit always corresponds to one pixel. Positive y is down. Positive x is
   right.
1. The world: coordinates are relative to the position and scale of the
   Camera2D. This means that you can move the camera around to change what is
   shown on screen, and one world unit does not always correspond to one pixel.
   By default, the world coordinate (0, 0) is in the center of the screen.

There are built in functions for drawing different shapes to each of these
spaces. There are functions provided for the following shapes:

- Circles
    - Ellipses
    - Arcs
    - Segments
    - Rings
    - Radial gradients
- Rectangles and squares
    - With rounded corners
- N-sided polygons
- Custom shapes from a series of points
- Lines
    - With caps
    - Dotted
    - Zig zags
    - Arrows
    - Paths
- Curves
    - Quadratic bezier
    - Cubic bezier
- Single pixels/pixel lines
- Arbritrary triangles/quads
- Other more niche shapes like hearts, moons, and n-sided stars.

All shapes can be additionally drawn with an outline.

For example to draw a square, in world space, with an outline, you would use
this code:

```rust
// pub fn draw_square_with_outline_world(
//     top_left: Vec2,
//     size: f32,
//     color: Color,
//     thickness: f32,
//     outline_color: Color,
// )

draw_square_outline_world(vec2(20.0, 30.0), 40.0, Color::RED_500, 2.0, Color::RED_300);
```

See: [`/examples/simple.rs`](https://github.com/LilyRL/sge/blob/master/examples/simple.rs)
