# Some rendering details

2D rendering works in layers. When you use `new_draw_queues()` or use a
post-processing effect, a new set of draw queues will be created, meaning that anything new that is drawn will be
drawn over everything drawn previously, no matter what. 

In the case of a
post-processing effect this is almost always what you want, but if it isn't you
may need to be careful of what order your drawing functions run. If this is a
problem for you, decouple your update functions from drawing functions that
don't mutate state, so that drawing functions can run in any order.


Within a single draw queue, objects drawn in screen-space will always be drawn
above objects drawn in world space. You can overwrite this using a new set of
draw queues.

## Scissors

A scissor is basically a filter to a rectangle of the screen, it will prevent
rendering of any pixels outside of that rectangle for the duration of that
scissor being active. You can add and remove them with `push_scissor` and
`pop_scissor`. Pushing a scissor when another scissor is already active will set
the scissor to the intersection of both of them, so things will only be drawn
if they are inside both rectangles.

```rust
// this will draw a semi-circle
push_scissor(Area::new(vec2(window_center().x, 0.0), window_size()).to_rect());
draw_circle(window_center(), 500.0, Color::WHITE);
pop_scissor();
```
