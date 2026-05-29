# Color type

SGE comes with a `Color` type, with rgba as `f32` values from 0.0 to 1.0.

## Creating colors

```rust
// float (0.0–1.0)
Color::from_rgb(1.0, 0.5, 0.0)
Color::from_rgba(1.0, 0.5, 0.0, 0.8)

// u8 (0–255)
Color::from_rgb_u8(255, 128, 0)
Color::from_rgba_u8(255, 128, 0, 200)

// hsl
Color::from_hsl(30.0, 1.0, 0.5)
Color::from_hsla(30.0, 1.0, 0.5, 0.8)

// oklch
Color::from_oklch(0.7, 0.15, 142.0)
Color::from_oklch_with_alpha(0.7, 0.15, 142.0, 0.8)

// From hex constant (compile-time)
Color::hex(0xFF8800)
Color::hex_alpha(0xFF8800FF)

// from a string at runtime (rgb, hsl, oklch, hex, and named colors)
Color::from_string("oklch 0.7 0.15 142")
Color::from_string("#FF8800")
Color::from_string("rgb 1.0 0.5 0.0")
Color::from_string("rebecca purple")

// every CSS and Tailwind color is available as a constant
Color::RED_500
Color::NEUTRAL_900
Color::CYAN_400
```

## Modifying colors

```rust
color.with_alpha(0.5)
color.with_red(1.0)
color.with_alpha8(128) // u8 

color.lighten(0.2)
color.darken(0.3)
// or oklch
color.lighten_oklch(0.2)
color.darken_oklch(0.1)

color.saturate(0.5)
color.desaturate(0.3)
color.hue_rotate(90.0)         // degrees, HSL
color.hue_rotate_oklch(45.0)  // degrees, oklch

color.inverted()

Color::blend_two(Color::RED_500, Color::BLUE_500, 0.5)
color.blend(other, 0.3)
color.blend_halfway(other)

Color::grey(0.5) // brightness
```

## Palettes

Tailwind color palettes are availible from 50 to 950.

```rust
let shades = Color::RED.shades();           //  light to dark
let shades = Color::BLUE.reversed_shades(); // dark to light
```

## Converting colors

```rust
color.to_oklch()        // (lightness, chroma, hue)
color.to_hex_string()   // "#RRGGBBAA"
color.to_hex()          // u32
color.to_color_u8()     // ColorU8 (for images)
color.to_linear()       // sRGB to linear RGB
```

## Color schemes

There are some built in
[colorschemes](https://docs.rs/sge/latest/sge/prelude/color/struct.ColorScheme.html#impl-ColorScheme-1)
you can use if you like.

---

Check the [reference documentation](https://docs.rs/sge/latest/sge/prelude/struct.Color.html) for the full list of methods.

See also: [color related documentation](https://docs.rs/sge/latest/sge/prelude/color/index.html)
