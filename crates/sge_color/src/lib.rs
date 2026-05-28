use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use egui_glium::egui_winit::egui::Color32;
use palette::{Hsl, IntoColor, LinSrgb, LinSrgba, Oklch, Srgb, Srgba};
use rkyv::{Archive, Deserialize, Serialize};
use sge_vectors::Vec4;
use u8::ColorU8;

mod data;
pub mod schemes;
#[cfg(test)]
mod tests;
pub mod u8;

pub use data::*;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Archive, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            (self.r * 255.0).round() as u8,
            (self.g * 255.0).round() as u8,
            (self.b * 255.0).round() as u8,
            (self.a * 255.0).round() as u8
        ))
    }
}

const fn convert(n: f32) -> u8 {
    (n * 255.0).round() as u8
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub const fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub const fn from_rgb_u8(r: u8, g: u8, b: u8) -> Self {
        const fn convert(n: u8) -> f32 {
            (n * 255) as f32
        }
        Self::new(convert(r), convert(g), convert(b))
    }

    pub fn from_gpu(v: [f32; 4]) -> Self {
        Self::from_rgba(v[0], v[1], v[2], v[3])
    }

    pub fn to_u8(&self) -> (u8, u8, u8) {
        (convert(self.r), convert(self.g), convert(self.b))
    }

    pub fn from_string(s: impl AsRef<str>) -> Option<Self> {
        str_to_color(s)
    }

    pub const fn to_rgba_u8(&self) -> (u8, u8, u8, u8) {
        (
            convert(self.r),
            convert(self.g),
            convert(self.b),
            convert(self.a),
        )
    }

    pub const fn from_rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        const fn convert(n: u8) -> f32 {
            n as f32 / 255.0
        }
        Self::from_rgba(convert(r), convert(g), convert(b), convert(a))
    }
    pub const fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    pub const fn for_gpu(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
    pub const fn to_vec4(&self) -> Vec4 {
        Vec4::new(self.r, self.g, self.b, self.a)
    }

    pub fn from_vec4(v: Vec4) -> Self {
        Self::from_rgba(v.x, v.y, v.z, v.w)
    }

    pub fn splat(v: f32) -> Self {
        Self::new(v, v, v)
    }

    pub fn grey(v: f32) -> Self {
        Self::splat(v)
    }

    pub const fn with_alpha(mut self, a: f32) -> Self {
        self.a = a;
        self
    }

    pub const fn with_red(mut self, r: f32) -> Self {
        self.r = r;
        self
    }

    pub const fn with_green(mut self, g: f32) -> Self {
        self.g = g;
        self
    }

    pub const fn with_blue(mut self, b: f32) -> Self {
        self.b = b;
        self
    }

    pub const fn with_alpha8(mut self, a: u8) -> Self {
        self.a = a as f32 / 255.0;
        self
    }

    pub const fn with_red8(mut self, r: u8) -> Self {
        self.r = r as f32 / 255.0;
        self
    }

    pub const fn with_green8(mut self, g: u8) -> Self {
        self.g = g as f32 / 255.0;
        self
    }

    pub const fn with_blue8(mut self, b: u8) -> Self {
        self.b = b as f32 / 255.0;
        self
    }

    pub fn inverted(self) -> Self {
        Self::from_rgba(1.0 - self.r, 1.0 - self.g, 1.0 - self.b, self.a)
    }

    fn to_hsl(self) -> (f32, f32, f32) {
        let lin_rgb = LinSrgb::new(self.r, self.g, self.b);
        let srgb: Srgb = lin_rgb.into_color();
        let hsl: Hsl = srgb.into_color();
        (
            hsl.hue.into_positive_degrees(),
            hsl.saturation,
            hsl.lightness,
        )
    }

    pub const fn hex(hex: u32) -> Self {
        let red = (hex & 0xFF0000) >> 16;
        let green = (hex & 0x00FF00) >> 8;
        let blue = hex & 0x0000FF;

        Self::from_rgba_u8(red as u8, green as u8, blue as u8, 255)
    }

    pub const fn hex_alpha(hex: u32) -> Self {
        let red = (hex & 0xFF000000) >> 24;
        let green = (hex & 0x00FF0000) >> 16;
        let blue = (hex & 0x0000FF00) >> 8;
        let alpha = hex & 0x000000FF;

        Self::from_rgba_u8(red as u8, green as u8, blue as u8, alpha as u8)
    }

    pub const fn to_hex(&self) -> u32 {
        let (r, g, b, a) = self.to_rgba_u8();
        ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32)
    }

    pub fn to_hex_string(&self) -> String {
        let (r, g, b, a) = self.to_rgba_u8();
        format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
    }

    pub fn from_hsla(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> Self {
        let hsl = Hsl::new(hue, saturation, lightness);
        let srgb: Srgb = hsl.into_color();
        let lin_rgb: LinSrgb = srgb.into_color();
        Self::from_rgba(lin_rgb.red, lin_rgb.green, lin_rgb.blue, alpha)
    }

    pub fn from_hsl(hue: f32, saturation: f32, lightness: f32) -> Self {
        Self::from_hsla(hue, saturation, lightness, 1.0)
    }

    pub fn to_oklch(&self) -> (f32, f32, f32) {
        let lin_rgb = LinSrgb::new(self.r, self.g, self.b);
        let oklch: Oklch = lin_rgb.into_color();
        (oklch.l, oklch.chroma, oklch.hue.into_positive_degrees())
    }

    pub fn from_oklch_with_alpha(lightness: f32, chroma: f32, hue: f32, alpha: f32) -> Self {
        let (r, g, b, a) =
            oklch_to_lin_srgba(lightness as f64, chroma as f64, hue as f64, alpha as f64);
        Self::from_rgba(r as f32, g as f32, b as f32, a as f32)
    }

    pub fn from_oklch(lightness: f32, chroma: f32, hue: f32) -> Self {
        let (r, g, b) = oklch_to_lin_srgb(lightness as f64, chroma as f64, hue as f64);
        Self::new(r as f32, g as f32, b as f32)
    }

    pub fn lighten(self, factor: f32) -> Self {
        let (h, s, l) = self.to_hsl();
        let new_l = (l + factor * (1.0 - l)).clamp(0.0, 1.0);
        Self::from_hsla(h, s, new_l, self.a)
    }

    pub fn darken(self, factor: f32) -> Self {
        let (h, s, l) = self.to_hsl();
        let new_l = (l - factor * l).clamp(0.0, 1.0);
        Self::from_hsla(h, s, new_l, self.a)
    }

    pub fn saturate(self, factor: f32) -> Self {
        let (h, s, l) = self.to_hsl();
        let new_s = (s + factor * (1.0 - s)).clamp(0.0, 1.0);
        Self::from_hsla(h, new_s, l, self.a)
    }

    pub fn desaturate(self, factor: f32) -> Self {
        let (h, s, l) = self.to_hsl();
        let new_s = (s - factor * s).clamp(0.0, 1.0);
        Self::from_hsla(h, new_s, l, self.a)
    }

    pub fn hue_rotate(self, degrees: f32) -> Self {
        let (h, s, l) = self.to_hsl();
        let new_h = (h + degrees).rem_euclid(360.0);
        Self::from_hsla(new_h, s, l, self.a)
    }

    pub fn lighten_oklch(self, factor: f32) -> Self {
        let (l, c, h) = self.to_oklch();
        let new_l = (l + factor * (1.0 - l)).clamp(0.0, 1.0);
        Self::from_oklch_with_alpha(new_l, c, h, self.a)
    }

    pub fn darken_oklch(self, factor: f32) -> Self {
        let (l, c, h) = self.to_oklch();
        let new_l = (l - factor * l).clamp(0.0, 1.0);
        Self::from_oklch_with_alpha(new_l, c, h, self.a)
    }

    pub fn hue_rotate_oklch(self, degrees: f32) -> Self {
        let (l, c, h) = self.to_oklch();
        let new_h = (h + degrees).rem_euclid(360.0);
        Self::from_oklch_with_alpha(l, c, new_h, self.a)
    }

    pub fn to_pixel(self) -> ColorU8 {
        ColorU8::from_rgba_f32(self.r, self.g, self.b, self.a)
    }

    #[inline]
    pub fn blend_two(a: Self, b: Self, fac: f32) -> Self {
        let f = fac.clamp(0.0, 1.0);
        Self::from_rgba(
            a.r + (b.r - a.r) * f,
            a.g + (b.g - a.g) * f,
            a.b + (b.b - a.b) * f,
            a.a + (b.a - a.a) * f,
        )
    }

    #[inline]
    pub fn blend_two_halfway(a: Self, b: Self) -> Self {
        Self::blend_two(a, b, 0.5)
    }

    #[inline]
    pub fn blend(self, other: Self, fac: f32) -> Self {
        Self::blend_two(self, other, fac)
    }

    #[inline]
    pub fn blend_halfway(self, other: Self) -> Self {
        Self::blend_two_halfway(self, other)
    }

    pub fn to_linear(self) -> Self {
        let lin: LinSrgba = Srgba::new(self.r, self.g, self.b, self.a).into_color();
        Self::from_rgba(lin.red, lin.green, lin.blue, lin.alpha)
    }

    pub fn from_usize(n: usize) -> Self {
        let r = n & 0b11111111;
        let g = (n >> 8) & 0b11111111;
        let b = (n >> 16) & 0b11111111;
        let a = (n >> 24) & 0b11111111;
        Self::from_rgba_u8(r as u8, g as u8, b as u8, a as u8)
    }

    pub fn from_usize_no_alpha(n: usize) -> Self {
        let r = n & 0b11111111;
        let g = (n >> 8) & 0b11111111;
        let b = (n >> 16) & 0b11111111;
        Self::from_rgba_u8(r as u8, g as u8, b as u8, 255)
    }
}

fn hex_char_to_int(c: u8) -> Option<u8> {
    const ZERO: u8 = b'0';
    const A: u8 = b'A';

    if c < ZERO {
        None
    } else if c - ZERO < 10 {
        Some(c - ZERO)
    } else if c < A {
        None
    } else if c - A < 6 {
        Some(c - A + 10)
    } else {
        None
    }
}

fn hex_chars_to_int(a: u8, b: u8) -> Option<u8> {
    Some((hex_char_to_int(a)? << 4) | hex_char_to_int(b)?)
}

/// assumes `#` already stripped
fn parse_hex(bytes: &[u8]) -> Option<Color> {
    if bytes.len() == 3 {
        let r = hex_char_to_int(bytes[0])? << 4;
        let g = hex_char_to_int(bytes[1])? << 4;
        let b = hex_char_to_int(bytes[2])? << 4;
        Some(Color::from_rgba_u8(r, g, b, 255))
    } else if bytes.len() == 6 {
        let r = hex_chars_to_int(bytes[0], bytes[1])?;
        let g = hex_chars_to_int(bytes[2], bytes[3])?;
        let b = hex_chars_to_int(bytes[4], bytes[5])?;
        Some(Color::from_rgba_u8(r, g, b, 255))
    } else if bytes.len() == 4 {
        let r = hex_char_to_int(bytes[0])? << 4;
        let g = hex_char_to_int(bytes[1])? << 4;
        let b = hex_char_to_int(bytes[2])? << 4;
        let a = hex_char_to_int(bytes[3])? << 4;
        Some(Color::from_rgba_u8(r, g, b, a))
    } else if bytes.len() == 8 {
        let r = hex_chars_to_int(bytes[0], bytes[1])?;
        let g = hex_chars_to_int(bytes[2], bytes[3])?;
        let b = hex_chars_to_int(bytes[4], bytes[5])?;
        let a = hex_chars_to_int(bytes[6], bytes[7])?;
        Some(Color::from_rgba_u8(r, g, b, a))
    } else {
        None
    }
}

fn parse_f32_or_u8(s: &str) -> Option<f32> {
    if let Ok(n) = s.parse::<u8>() {
        Some(n as f32 / 255.0)
    } else {
        s.parse::<f32>().ok()
    }
}

fn parse_f32(s: &str) -> Option<f32> {
    s.parse::<f32>().ok()
}

fn normalize_triple(s: &str) -> String {
    let s = s.replace(" ", ",");
    let s = s.replace(",,", ",");
    let s = s.strip_prefix(',').unwrap_or(s.as_str());
    let s = s.strip_suffix(',').unwrap_or(s);
    s.to_string()
}

/// assumes `rgb(` & `)` have already been stripped
fn parse_rgb_triple(s: &str) -> Option<Color> {
    let s = normalize_triple(s);
    let parts = s.split(",").collect::<Vec<_>>();

    if parts.len() == 3 {
        let r = parse_f32_or_u8(parts[0])?;
        let g = parse_f32_or_u8(parts[1])?;
        let b = parse_f32_or_u8(parts[2])?;
        Some(Color::from_rgb(r, g, b))
    } else if parts.len() == 4 {
        let r = parse_f32_or_u8(parts[0])?;
        let g = parse_f32_or_u8(parts[1])?;
        let b = parse_f32_or_u8(parts[2])?;
        let a = parse_f32_or_u8(parts[3])?;
        Some(Color::from_rgba(r, g, b, a))
    } else {
        None
    }
}

fn parse_hsl_triple(s: &str) -> Option<Color> {
    let s = normalize_triple(s);
    let parts = s.split(",").collect::<Vec<_>>();

    if parts.len() == 3 {
        let h = parse_f32(parts[0])?;
        let s = parse_f32_or_u8(parts[1])?;
        let l = parse_f32_or_u8(parts[2])?;
        Some(Color::from_hsl(h, s, l))
    } else if parts.len() == 4 {
        let h = parse_f32(parts[0])?;
        let s = parse_f32_or_u8(parts[1])?;
        let l = parse_f32_or_u8(parts[2])?;
        let a = parse_f32_or_u8(parts[3])?;
        Some(Color::from_hsla(h, s, l, a))
    } else {
        None
    }
}

fn parse_oklch_triple(s: &str) -> Option<Color> {
    let s = normalize_triple(s);
    let parts = s.split(",").collect::<Vec<_>>();

    if parts.len() == 3 {
        let l = parse_f32_or_u8(parts[0])?;
        let c = parse_f32_or_u8(parts[1])?;
        let h = parse_f32(parts[2])?;
        Some(Color::from_oklch(l, c, h))
    } else if parts.len() == 4 {
        let l = parse_f32_or_u8(parts[0])?;
        let c = parse_f32_or_u8(parts[1])?;
        let h = parse_f32(parts[2])?;
        let a = parse_f32_or_u8(parts[3])?;
        Some(Color::from_oklch_with_alpha(l, c, h, a))
    } else {
        None
    }
}

/// quite expensive. probably don't use unless you have to
pub fn str_to_color(s: impl AsRef<str>) -> Option<Color> {
    let s = s.as_ref().to_uppercase();
    let s = s.trim();

    if let Some(s) = s.strip_prefix('#') {
        let bytes = s.as_bytes();
        return parse_hex(bytes);
    }

    if let Some(s) = s.strip_prefix("RGB") {
        return parse_rgb_triple(&s.replace("(", "").replace(")", ""));
    }

    if let Some(s) = s.strip_prefix("HSL") {
        return parse_hsl_triple(&s.replace("(", "").replace(")", ""));
    }

    if let Some(s) = s.strip_prefix("OKLCH") {
        return parse_oklch_triple(&s.replace("(", "").replace(")", ""));
    }

    let s = s
        .replace("_", "")
        .replace(" ", "")
        .replace("00", "")
        .replace("950", "9.5")
        .replace("50", "0.5")
        .replace("-", "");
    COLOR_MAP.get(s.as_str()).copied()
}

impl From<Color> for Color32 {
    fn from(value: Color) -> Self {
        let raw = value.to_pixel().raw();
        Color32::from_rgba_unmultiplied(raw[0], raw[1], raw[2], raw[3])
    }
}

fn oklch_to_lin_srgb(lightness: f64, chroma: f64, hue: f64) -> (f64, f64, f64) {
    let oklch = Oklch::new(lightness as f32, chroma as f32, hue as f32);
    let lin_rgb: LinSrgb = oklch.into_color();
    (
        lin_rgb.red as f64,
        lin_rgb.green as f64,
        lin_rgb.blue as f64,
    )
}

fn oklch_to_lin_srgba(lightness: f64, chroma: f64, hue: f64, alpha: f64) -> (f64, f64, f64, f64) {
    let (r, g, b) = oklch_to_lin_srgb(lightness, chroma, hue);
    (r, g, b, alpha)
}

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub v100: Color,
    pub v200: Color,
    pub v300: Color,
    pub v400: Color,
    pub v500: Color,
    pub v600: Color,
    pub v700: Color,
    pub v800: Color,
    pub v900: Color,
    pub v950: Color,
    pub v50: Color,
}

impl Palette {
    /// light to dark
    pub fn shades(&self) -> [Color; 11] {
        [
            self.v50, self.v100, self.v200, self.v300, self.v400, self.v500, self.v600, self.v700,
            self.v800, self.v900, self.v950,
        ]
    }

    /// dark to light
    pub fn reversed_shades(&self) -> [Color; 11] {
        [
            self.v950, self.v900, self.v800, self.v700, self.v600, self.v500, self.v400, self.v300,
            self.v200, self.v100, self.v50,
        ]
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_rgba(
            self.r + rhs.r,
            self.g + rhs.g,
            self.b + rhs.b,
            self.a + rhs.a,
        )
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_rgba(
            self.r - rhs.r,
            self.g - rhs.g,
            self.b - rhs.b,
            self.a - rhs.a,
        )
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
        self.a -= rhs.a;
    }
}
