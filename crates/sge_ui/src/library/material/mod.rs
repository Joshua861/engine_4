use material_colors::{color::Argb, theme::ThemeBuilder};

use super::UiRef;
use sge_color::Color;

sge_global::global!(MaterialTheme, theme_state);

struct MaterialTheme {
    dark_scheme: Scheme,
    light_scheme: Scheme,
    dark: bool,
}

pub const CARD_RADIUS: f32 = 12.0;
pub const CARD_PADDING: f32 = 16.0;

pub const INPUT_RADIUS: f32 = 8.0;
pub const INPUT_PADDING: f32 = 20.0;

pub const PROGRESS_HEIGHT: f32 = 4.0;

pub struct Scheme {
    pub primary: Color,
    pub on_primary: Color,
    pub primary_container: Color,
    pub on_primary_container: Color,
    pub inverse_primary: Color,
    pub primary_fixed: Color,
    pub primary_fixed_dim: Color,
    pub on_primary_fixed: Color,
    pub on_primary_fixed_variant: Color,
    pub secondary: Color,
    pub on_secondary: Color,
    pub secondary_container: Color,
    pub on_secondary_container: Color,
    pub secondary_fixed: Color,
    pub secondary_fixed_dim: Color,
    pub on_secondary_fixed: Color,
    pub on_secondary_fixed_variant: Color,
    pub tertiary: Color,
    pub on_tertiary: Color,
    pub tertiary_container: Color,
    pub on_tertiary_container: Color,
    pub tertiary_fixed: Color,
    pub tertiary_fixed_dim: Color,
    pub on_tertiary_fixed: Color,
    pub on_tertiary_fixed_variant: Color,
    pub error: Color,
    pub on_error: Color,
    pub error_container: Color,
    pub on_error_container: Color,
    pub surface_dim: Color,
    pub surface: Color,
    pub surface_tint: Color,
    pub surface_bright: Color,
    pub surface_container_lowest: Color,
    pub surface_container_low: Color,
    pub surface_container: Color,
    pub surface_container_high: Color,
    pub surface_container_highest: Color,
    pub on_surface: Color,
    pub on_surface_variant: Color,
    pub outline: Color,
    pub outline_variant: Color,
    pub inverse_surface: Color,
    pub inverse_on_surface: Color,
    pub surface_variant: Color,
    pub background: Color,
    pub on_background: Color,
    pub shadow: Color,
    pub scrim: Color,
}

impl Scheme {
    pub fn from_material_scheme(material: material_colors::scheme::Scheme) -> Self {
        Self {
            primary: color(material.primary),
            on_primary: color(material.on_primary),
            primary_container: color(material.primary_container),
            on_primary_container: color(material.on_primary_container),
            inverse_primary: color(material.inverse_primary),
            primary_fixed: color(material.primary_fixed),
            primary_fixed_dim: color(material.primary_fixed_dim),
            on_primary_fixed: color(material.on_primary_fixed),
            on_primary_fixed_variant: color(material.on_primary_fixed_variant),
            secondary: color(material.secondary),
            on_secondary: color(material.on_secondary),
            secondary_container: color(material.secondary_container),
            on_secondary_container: color(material.on_secondary_container),
            secondary_fixed: color(material.secondary_fixed),
            secondary_fixed_dim: color(material.secondary_fixed_dim),
            on_secondary_fixed: color(material.on_secondary_fixed),
            on_secondary_fixed_variant: color(material.on_secondary_fixed_variant),
            tertiary: color(material.tertiary),
            on_tertiary: color(material.on_tertiary),
            tertiary_container: color(material.tertiary_container),
            on_tertiary_container: color(material.on_tertiary_container),
            tertiary_fixed: color(material.tertiary_fixed),
            tertiary_fixed_dim: color(material.tertiary_fixed_dim),
            on_tertiary_fixed: color(material.on_tertiary_fixed),
            on_tertiary_fixed_variant: color(material.on_tertiary_fixed_variant),
            error: color(material.error),
            on_error: color(material.on_error),
            error_container: color(material.error_container),
            on_error_container: color(material.on_error_container),
            surface_dim: color(material.surface_dim),
            surface: color(material.surface),
            surface_tint: color(material.surface_tint),
            surface_bright: color(material.surface_bright),
            surface_container_lowest: color(material.surface_container_lowest),
            surface_container_low: color(material.surface_container_low),
            surface_container: color(material.surface_container),
            surface_container_high: color(material.surface_container_high),
            surface_container_highest: color(material.surface_container_highest),
            on_surface: color(material.on_surface),
            on_surface_variant: color(material.on_surface_variant),
            outline: color(material.outline),
            outline_variant: color(material.outline_variant),
            inverse_surface: color(material.inverse_surface),
            inverse_on_surface: color(material.inverse_on_surface),
            surface_variant: color(material.surface_variant),
            background: color(material.background),
            on_background: color(material.on_background),
            shadow: color(material.shadow),
            scrim: color(material.scrim),
        }
    }
}

pub fn scheme() -> &'static Scheme {
    let state = get_theme_state();
    if state.dark {
        &state.dark_scheme
    } else {
        &state.light_scheme
    }
}

pub fn set_theme_color(color: Color) {
    let theme = ThemeBuilder::with_source(argb(color)).build();
    let state = get_theme_state();
    state.dark_scheme = Scheme::from_material_scheme(theme.schemes.dark);
    state.light_scheme = Scheme::from_material_scheme(theme.schemes.light);
}

pub fn set_theme_dark(dark: bool) {
    let state = get_theme_state();
    state.dark = dark;
}

pub fn get_theme_dark() -> bool {
    get_theme_state().dark
}

pub(crate) fn init() {
    let c = argb(Color::SLATE_500);
    let theme = ThemeBuilder::with_source(c).build();
    set_theme_state(MaterialTheme {
        dark_scheme: Scheme::from_material_scheme(theme.schemes.dark),
        light_scheme: Scheme::from_material_scheme(theme.schemes.light),
        dark: false,
    });
}

fn argb(color: Color) -> Argb {
    let (r, g, b, a) = color.to_rgba_u8();
    Argb::new(a, r, g, b)
}

fn color(argb: Argb) -> Color {
    Color::from_rgba_u8(argb.red, argb.green, argb.blue, argb.alpha)
}

const fn rem(n: f32) -> usize {
    (n * 16.0).round() as usize
}

mod card;
pub use card::*;

mod text;
pub use text::*;

mod progress;
pub use progress::*;

mod loading_spinner;
pub use loading_spinner::*;

mod input;
pub use input::*;

mod drawer;
pub use drawer::*;
