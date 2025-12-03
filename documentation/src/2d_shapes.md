# 2D shapes

The engine has many functions for drawing 2D shapes to the screen, such as
squares, rectangles, lines, and circles. Here is the full list.

- `draw_circle`
- `draw_circle_outline`
- `draw_circle_outline_world`
- `draw_circle_with_outline`
- `draw_circle_with_outline_world`
- `draw_circle_world`
- `draw_custom_shape`
- `draw_custom_shape_world`
- `draw_ellipse`
- `draw_ellipse_outline`
- `draw_ellipse_outline_world`
- `draw_ellipse_with_outline`
- `draw_ellipse_with_outline_world`
- `draw_ellipse_world`
- `draw_hexagon`
- `draw_hexagon_pointy`
- `draw_hexagon_pointy_world`
- `draw_hexagon_world`
- `draw_line`
- `draw_line_world`
- `draw_poly`
- `draw_poly_outline`
- `draw_poly_outline_world`
- `draw_poly_world`
- `draw_rect`
- `draw_rect_outline`
- `draw_rect_outline_world`
- `draw_rect_world`
- `draw_shape`
- `draw_shape_world`
- `draw_square`
- `draw_square_outline`
- `draw_square_outline_world`
- `draw_square_world`
- `draw_tri`
- `draw_tri_outline`
- `draw_tri_outline_world`
- `draw_tri_world`
- `clear_screen`

Functions ending with \_world draw based on the Camera2D, which will be covered
later. Those that do not draw based on pixel coordinates from the top left of
the window.

Most of these are pretty self explanatory. The only ones that may not be clear
is poly, which is a N-agon, such as pentagon or decagon, or a custom shape,
which creates a shape out of a list of vertices (points) that make it up.

One other point of interest is that circles are pixel perfect in this engine.
Meaning that no matter what scale they are rendered at, they will still be
perfect circles.

All these shapes can be represented by structs. An arbitrary shape struct can
be drawn with the `draw_shape` and `draw_shape_world` functions, as demonstrated
in the `interpolation.rs` example.

Check the `demo.rs` example for more information.
