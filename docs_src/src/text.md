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

## Fonts

Fonts can be loaded with
[`create_ttf_font`](https://docs.rs/sge/latest/sge/prelude/text/fn.create_ttf_font.html).

SGE comes with [JetBrains Mono](https://www.jetbrains.com/lp/mono/) as it's
default font, and when the `extra_fonts` feature is enabled (by default), also 5
variants of the [Inter](https://rsms.me/inter/) typeface, for regular, bold,
italic, bold italic, and display.

See:
[`/examples/text.rs`](https://github.com/LilyRL/sge/blob/master/examples/text.rs)
for an example

See: [text module
documentation](https://docs.rs/sge/latest/sge/prelude/text/index.html) for more detail.
