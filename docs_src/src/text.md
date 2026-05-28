# Drawing text

There are 3 types of text drawing functions:
1. `draw_text_*`: for basic text drawing. 
1. `draw_multiline_text_*`: for properly drawing text with newlines in it.
1. `draw_wrapped_text_*`: for drawing text with a max width, that wraps when it
   gets to the edge.

There are also equivalent functions for measuring the area text will take up
when drawn.

There are functions for drawing text quickly, which only take the text and
position, and draw with the default monospaced font. You can use `draw_text_ex`
and `draw_text_custom` to specify the font used, text color, and more.

See:
[`/examples/text.rs`](https://github.com/LilyRL/sge/blob/master/examples/text.rs)
for an example
See: [text module
documentation](https://docs.rs/sge/latest/sge/prelude/text/index.html) for more detail.
