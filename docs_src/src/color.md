# Color type

SGE comes with a featureful `Color` type. You can create a `Color` using
`Color::from_rgb(r: f32, g: f32, b: f32)`, where rgb range from 0.0 to 1.0, or
`Color::from_rgb_u8(r: u8, g: u8, b: u8)`, where rgb range from 0 to 255. Both
of these functions have corresponding `from_rgba` variants that allow you to
also specify an opacity/alpha value.

In addition, the `Color` type has associated constants for every [CSS
color](https://www.w3schools.com/cssref/css_colors.php), and [Tailwind color](https://tailwindcss.com/docs/colors).

Check the [reference documentation](https://docs.rs/sge/latest/sge/prelude/struct.Color.html) for the color type for more detail.

See also: [color related documentation](https://docs.rs/sge/latest/sge/prelude/color/index.html)
