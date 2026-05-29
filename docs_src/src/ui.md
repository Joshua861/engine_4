# UI

Finally... I've been waiting a long time to write this chapter. This is the #1
best part of the engine, no contest. THE USER INTERFACE LIBRARY!!!

The UI library is split into 2 sections. There is a library of `base` unstyled
elements that you can use to create more complex user interfaces of any style,
and a set of styles widgets from a few categories, at the moment there is `flat`
(the most comprehensive), `material` (meant to mimic Google Material UI 3;
supports theming), and `w95` (meant to mimic Windows 95, not really updated).

Complex layouts can be created by combining the basic `base`
components in a nested arrangement. The base components are designed to be as
simple as possible so as to be very flexible and be able to be used for many
different things by combining them with other elements. For example, instead of
a single `div` element like in HTML, there are smaller simpler elements for
adding a fill, adding a border, centering an element, adding padding, etc.

Base elements are designed to be as flexible as possible. For example, the
slider element allows you to create the bar and handle from other UI elements,
meaning you can have it look however you want. For an example of how this works,
check the [source code for the flat slider](https://docs.rs/sge_ui/1.1.4/src/sge_ui/library/flat/slider.rs.html#8)

> "The best way to learn about something is to see and use it for yourself" 
>
> -Sun Tzu

Thanks Sun, for this reason, I will point you to the `ui_showcase` example. If
you dont remember how to run examples, check the introduction chapter. This
example has a list of every UI element, and shows them in action. I would
reccomend you look through this with the code for the example open in another
window to see how everything is used.

Here's an awful looking screenshot from the `ui` example.

![Showcase of ui elements](./ui.jpg)

---

See: [ui module](https://docs.rs/sge/latest/sge/prelude/ui/index.html)
