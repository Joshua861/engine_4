#![allow(unused)]

use super::Color;

pub struct ColorScheme {
    pub bg0: Color,
    pub bg1: Color,
    pub bg2: Color,
    pub bg3: Color,
    pub bg4: Color,

    pub fg0: Color,
    pub fg1: Color,
    pub fg2: Color,
    pub fg3: Color,
    pub fg4: Color,

    pub red: Color,
    pub orange: Color,
    pub yellow: Color,
    pub green: Color,
    pub blue: Color,
    pub purple: Color,
    pub aqua: Color,
    pub gray: Color,

    pub light_red: Color,
    pub light_orange: Color,
    pub light_yellow: Color,
    pub light_green: Color,
    pub light_blue: Color,
    pub light_purple: Color,
    pub light_aqua: Color,
    pub light_gray: Color,
}

// here down is slop

impl ColorScheme {
    /// Gruvbox Dark color scheme
    pub const GRUVBOX_DARK: Self = Self {
        // Background shades
        bg0: Color::from_rgba_u8(0x1e, 0x1e, 0x1e, 255), // bg0 - base background
        bg1: Color::from_rgba_u8(0x26, 0x24, 0x23, 255), // bg1 - slightly lighter
        bg2: Color::from_rgba_u8(0x2e, 0x2a, 0x29, 255), // bg2 - line numbers
        bg3: Color::from_rgba_u8(0x3f, 0x39, 0x35, 255), // bg3 - borders
        bg4: Color::from_rgba_u8(0x53, 0x4a, 0x42, 255), // bg4 - cursor line

        // Foreground shades
        fg0: Color::from_rgba_u8(0xcb, 0xb8, 0x90, 255), // fg - default text
        fg1: Color::from_rgba_u8(0xaf, 0x9f, 0x81, 255), // fg1 - slightly faded
        fg2: Color::from_rgba_u8(0x97, 0x87, 0x71, 255), // fg2 - dimmed
        fg3: Color::from_rgba_u8(0x7f, 0x70, 0x61, 255), // gray1 - comments
        fg4: Color::from_rgba_u8(0x68, 0x5c, 0x51, 255), // bg5 - very dim

        // Core accent colors
        red: Color::from_rgba_u8(0xf7, 0x30, 0x28, 255), // bright red
        orange: Color::from_rgba_u8(0xfb, 0x6a, 0x16, 255), // bright orange
        yellow: Color::from_rgba_u8(0xf7, 0xb1, 0x25, 255), // bright yellow
        green: Color::from_rgba_u8(0xaa, 0xb0, 0x1e, 255), // bright green
        blue: Color::from_rgba_u8(0x0e, 0x53, 0x65, 255), // blue dark
        purple: Color::from_rgba_u8(0xc7, 0x70, 0x89, 255), // bright purple
        aqua: Color::from_rgba_u8(0x7d, 0xb6, 0x69, 255), // bright aqua
        gray: Color::from_rgba_u8(0x7f, 0x70, 0x61, 255), // gray

        // Light/highlighted variants
        light_red: Color::from_rgba_u8(0xf7, 0x30, 0x28, 255),
        light_orange: Color::from_rgba_u8(0xfb, 0x6a, 0x16, 255),
        light_yellow: Color::from_rgba_u8(0xfa, 0xee, 0xbb, 255), // yellow2 - highlight
        light_green: Color::from_rgba_u8(0x35, 0x6a, 0x46, 255),  // green deep
        light_blue: Color::from_rgba_u8(0x0e, 0x53, 0x65, 255),
        light_purple: Color::from_rgba_u8(0x7b, 0x2b, 0x5e, 255), // purple dark
        light_aqua: Color::from_rgba_u8(0x71, 0x95, 0x86, 255),   // teal
        light_gray: Color::from_rgba_u8(0xaf, 0x9f, 0x81, 255),
    };

    /// Gruvbox Light color scheme
    pub const GRUVBOX_LIGHT: Self = Self {
        // Background shades (inverted - lighter backgrounds)
        bg0: Color::from_rgba_u8(0xf8, 0xf4, 0xcd, 255), // lightest - base bg
        bg1: Color::from_rgba_u8(0xef, 0xdf, 0xae, 255), // yellow1 - slightly darker
        bg2: Color::from_rgba_u8(0xe6, 0xd4, 0xa3, 255), // alt highlight
        bg3: Color::from_rgba_u8(0xcb, 0xb8, 0x90, 255), // medium
        bg4: Color::from_rgba_u8(0xaf, 0x9f, 0x81, 255), // darker bg element

        // Foreground shades (inverted - darker text)
        fg0: Color::from_rgba_u8(0x26, 0x24, 0x23, 255), // dark text
        fg1: Color::from_rgba_u8(0x3f, 0x39, 0x35, 255), // slightly lighter
        fg2: Color::from_rgba_u8(0x53, 0x4a, 0x42, 255), // medium text
        fg3: Color::from_rgba_u8(0x68, 0x5c, 0x51, 255), // comments
        fg4: Color::from_rgba_u8(0x7f, 0x70, 0x61, 255), // faded text

        // Core accent colors (adjusted for light background)
        red: Color::from_rgba_u8(0x89, 0x00, 0x09, 255), // red dark
        orange: Color::from_rgba_u8(0x9d, 0x28, 0x07, 255), // orange dark
        yellow: Color::from_rgba_u8(0xa5, 0x63, 0x11, 255), // yellow dark
        green: Color::from_rgba_u8(0x66, 0x62, 0x0d, 255), // green dark
        blue: Color::from_rgba_u8(0x0e, 0x53, 0x65, 255), // blue dark
        purple: Color::from_rgba_u8(0x7b, 0x2b, 0x5e, 255), // purple dark
        aqua: Color::from_rgba_u8(0x35, 0x6a, 0x46, 255), // green deep
        gray: Color::from_rgba_u8(0x68, 0x5c, 0x51, 255), // gray

        // Light/highlighted variants (brighter for light mode)
        light_red: Color::from_rgba_u8(0xf7, 0x30, 0x28, 255),
        light_orange: Color::from_rgba_u8(0xfb, 0x6a, 0x16, 255),
        light_yellow: Color::from_rgba_u8(0xf7, 0xb1, 0x25, 255),
        light_green: Color::from_rgba_u8(0xaa, 0xb0, 0x1e, 255),
        light_blue: Color::from_rgba_u8(0x0e, 0x53, 0x65, 255),
        light_purple: Color::from_rgba_u8(0xc7, 0x70, 0x89, 255),
        light_aqua: Color::from_rgba_u8(0x7d, 0xb6, 0x69, 255),
        light_gray: Color::from_rgba_u8(0x97, 0x87, 0x71, 255),
    };
}
