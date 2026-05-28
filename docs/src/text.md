# Drawing text

There are 3 types of text drawing functions:
1. `draw_text_*`: for basic text drawing. 
1. `draw_multiline_text_*`: for properly drawing text with newlines in it.
1. `draw_wrapped_text_*`: for drawing text with a max width, that wraps when it
   gets to the edge.

There are also equivalent functions for measuring the area text will take up
when drawn.

See: [`/examples/text.rs`](https://github.com/LilyRL/sge/blob/master/examples/text.rs)
