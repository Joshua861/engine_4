# Rich Text

There is support for drawing text of multiple sizes/colors/formats/fonts in one
block of text through rich text.

Blocks of rich text can be created manually, or parsed from a HTML like
language using `rich_text(str)`. Note that this parsing is expensive, and you should make sure that it
is not being run every frame. Store the parsed result and use that every frame.

Quotes around argument values are optional as long as the value does not
contain spaces (e.g. #abc).

```html
<font size=50>This text is <font color=red3>red</font>, and this text is <font color=blue3>blue</font>
<font color=#abc>You <font size=30>may make your</font> text any</font> <font color="rgb 1.0 1.0 1.0">color <hl color=slate9>you want</hl></font>
<font bold color="oklch 0.7 0.1184 119">Check the docume</font><font italic color=blue2>ntation for</font> <b>rich_text()</b> for more.
<i>Lorem <ol color=green5>ipsum dolor</ol> <ul>sit amet consectetur</ul> adipiscing elit.</i>
<ul color="red5">You <st>can</st> nest <font color=red5 size=70 bold>styles</font><noul> inside </noul>of    eachother</ul></font>
```

![How this rich text looks when rendered](./rich_text.jpg)

Rich text can be drawn with `.draw` or `.draw_world`, and the text will be
wrapped within the area provided, and printed to stdout (the terminal), with
most of the formatting applied.

Rich text is used by the logging system.

See: [`/examples/rich_text.rs`](https://github.com/LilyRL/sge/blob/master/examples/rich_text.rs)

See: [`rich_text`](https://docs.rs/sge/latest/sge/prelude/text/fn.rich_text.html).
