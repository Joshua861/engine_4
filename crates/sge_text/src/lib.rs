use std::{collections::HashMap, hash::Hash};

use bitmap::{BitmapFont, BitmapFontSettings};
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
use sge_color::Color;
use sge_error_union::ErrorUnion;
use sge_macros::gen_ref_type;
use sge_math::transform::Transform2D;
use sge_rendering::{api::draw_texture_to_ex, d2::Renderer2D, dq2d, wdq2d};
use sge_texture_atlas::{SpriteKey, TextureAtlas};
use sge_textures::TextureRef;
use sge_vectors::{IVec2, Vec2, vec2};

pub mod api;
pub mod bitmap;
pub mod icons;
pub mod rich_text;
pub mod typeface;
pub mod vector;
pub mod wrapped_text;

pub use api::*;
pub use typeface::*;

use vector::VectorFont;
pub use wrapped_text::*;

pub const DEFAULT_FONT_SIZE: usize = TextDrawParams::DEFAULT.font_size;

enum FontInner {
    Bitmap(BitmapFont),
    Vector(VectorFont),
}

impl FontSource for FontInner {
    #[inline]
    fn character_info(&mut self, glyph: Glyph) -> CharacterInfo {
        match self {
            Self::Vector(v) => v.character_info(glyph),
            Self::Bitmap(b) => b.character_info(glyph),
        }
    }

    #[inline]
    fn font_metrics(&self, font_size: usize) -> FontMetrics {
        match self {
            Self::Vector(v) => v.font_metrics(font_size),
            Self::Bitmap(b) => b.font_metrics(font_size),
        }
    }

    #[inline]
    fn texture_atlas_mut(&mut self) -> &mut TextureAtlas {
        match self {
            Self::Vector(v) => v.texture_atlas_mut(),
            Self::Bitmap(b) => b.texture_atlas_mut(),
        }
    }

    #[inline]
    fn texture_atlas(&self) -> &TextureAtlas {
        match self {
            Self::Vector(v) => v.texture_atlas(),
            Self::Bitmap(b) => b.texture_atlas(),
        }
    }

    #[inline]
    fn contains(&self, glyph: Glyph) -> bool {
        match self {
            Self::Vector(v) => v.contains(glyph),
            Self::Bitmap(b) => b.contains(glyph),
        }
    }
}

trait FontSource {
    fn character_info(&mut self, glyph: Glyph) -> CharacterInfo;
    fn font_metrics(&self, font_size: usize) -> FontMetrics;
    fn texture_atlas(&self) -> &TextureAtlas;
    fn texture_atlas_mut(&mut self) -> &mut TextureAtlas;
    fn contains(&self, glyph: Glyph) -> bool;

    fn set_minify_filter(&mut self, filter_mode: MinifySamplerFilter) {
        self.texture_atlas_mut().set_minify_filter(filter_mode);
    }

    fn set_magnify_filter(&mut self, filter_mode: MagnifySamplerFilter) {
        self.texture_atlas_mut().set_magnify_filter(filter_mode);
    }

    fn use_linear_filtering(&mut self) {
        self.texture_atlas_mut().use_linear_filtering();
    }

    fn use_nearest_filtering(&mut self) {
        self.texture_atlas_mut().use_nearest_filtering();
    }

    fn texture(&mut self) -> TextureRef {
        self.texture_atlas_mut().texture().unwrap()
    }

    fn draw_sprite(
        &mut self,
        key: SpriteKey,
        color: Color,
        position: Vec2,
        renderer: Renderer2D,
    ) -> Option<()> {
        let sprite = self.texture_atlas().get(key)?;
        let sprite_size = sprite.rect.size();
        let scale = vec2(sprite_size.x as f32, sprite_size.y as f32);

        let transform = Transform2D::from_scale_translation(scale, position);
        let texture = self.texture();

        draw_texture_to_ex(
            texture,
            transform,
            color,
            Some(sprite.rect.as_rect().into()),
            renderer,
        );

        Some(())
    }

    fn measure_text(&mut self, text: &str, font_size: usize) -> TextDimensions {
        if text.is_empty() {
            return TextDimensions::default();
        }

        let metrics = self.font_metrics(font_size);
        let mut width = 0.0f32;

        for c in text.chars() {
            let glyph = Glyph {
                size: font_size,
                character: c,
            };

            let info = self.character_info(glyph);
            width += info.advance;
        }

        let size = vec2(width, metrics.line_height());
        TextDimensions {
            size,
            final_cursor_pos: size,
        }
    }
}

pub struct Font {
    font: FontInner,
    cache: HashMap<(usize, String), Vec2>,
}

impl Font {
    fn vector_from_bytes(bytes: &[u8]) -> Result<Self, LoadFontError> {
        Ok(Self {
            font: FontInner::Vector(VectorFont::from_bytes(bytes)?),
            cache: HashMap::new(),
        })
    }

    fn bitmap_from_bytes(
        bytes: &[u8],
        settings: &BitmapFontSettings,
    ) -> Result<Self, LoadFontError> {
        Ok(Self {
            font: FontInner::Bitmap(BitmapFont::load_from_bytes(bytes, settings)?),
            cache: HashMap::new(),
        })
    }

    fn measure_text(&mut self, text: String, font_size: usize) -> TextDimensions {
        // FIXME: this clone is probably avoidable
        if let Some(size) = self.cache.get(&(font_size, text.clone())) {
            TextDimensions {
                size: *size,
                final_cursor_pos: *size,
            }
        } else {
            let dimensions = self.font.measure_text(&text, font_size);
            self.cache.insert((font_size, text), dimensions.size);
            dimensions
        }
    }

    pub fn ascii_character_list() -> Vec<char> {
        (0..255).filter_map(::std::char::from_u32).collect()
    }

    pub fn latin_character_list() -> Vec<char> {
        "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890!@#$%^&*(){}[].,:"
            .chars()
            .collect()
    }

    pub fn populate_font_cache(&mut self, characters: &[char], size: usize) {
        for character in characters {
            self.font.character_info(Glyph {
                character: *character,
                size,
            });
        }
    }

    pub fn draw_text_to(
        &mut self,
        text: String,
        position: Vec2,
        color: Color,
        font_size: usize,
        line_spacing: f32,
        renderer: Renderer2D,
    ) -> TextDimensions {
        if text.is_empty() {
            return TextDimensions::default();
        }

        let metrics = self.font.font_metrics(font_size);
        let base_line_height = metrics.line_height();
        let scaled_line_height = base_line_height * line_spacing;

        let space_advance = self
            .font
            .character_info(Glyph {
                character: ' ',
                size: font_size,
            })
            .advance;

        let mut chars = text.chars().peekable();

        let mut x = position.x;
        let mut y = position.y;
        let mut width = 0.0;
        let mut height = 0.0f32;

        while chars.peek() == Some(&' ') {
            chars.next();
            x += space_advance;
            width += space_advance;
        }

        for c in chars {
            if c == '\n' {
                x = position.x;
                y += scaled_line_height;
                height += scaled_line_height;
                continue;
            }

            let glyph = Glyph {
                character: c,
                size: font_size,
            };

            let char_info = self.font.character_info(glyph);

            let baseline_y = y + metrics.ascent;

            let glyph_render_pos = vec2(
                x + char_info.offset.x as f32,
                baseline_y - char_info.offset.y as f32,
            );

            self.font
                .draw_sprite(char_info.sprite, color, glyph_render_pos, renderer);

            x += char_info.advance;
            width += char_info.advance;
        }

        let size = Vec2::new(width, height.max(base_line_height));
        TextDimensions {
            size,
            final_cursor_pos: size,
        }
    }

    pub fn draw_text(
        &mut self,
        text: String,
        position: Vec2,
        color: Color,
        font_size: usize,
        line_spacing: f32,
    ) -> TextDimensions {
        self.draw_text_to(text, position, color, font_size, line_spacing, dq2d())
    }

    pub fn draw_text_world(
        &mut self,
        text: String,
        position: Vec2,
        color: Color,
        font_size: usize,
        line_spacing: f32,
    ) -> TextDimensions {
        self.draw_text_to(text, position, color, font_size, line_spacing, wdq2d())
    }

    pub fn measure_multiline_text(
        &mut self,
        text: String,
        font_size: usize,
        line_spacing: f32,
    ) -> TextDimensions {
        if text.is_empty() {
            return TextDimensions::default();
        }

        let space_info = self.measure_space(font_size);
        let line_height = space_info.offset.y as f32 * line_spacing;

        let mut width = 0.0f32;
        let mut height = 0.0f32;

        let mut lines = text.split('\n');
        if let Some(first_line) = lines.next() {
            width = self.font.measure_text(first_line, font_size).size.x;

            for line in lines {
                height += line_height;
                let line_width = self.font.measure_text(line, font_size).size.x;
                width = width.max(line_width);
            }
        }

        height = height.max(space_info.offset.y as f32);

        let size = Vec2::new(width, height);
        TextDimensions {
            size,
            final_cursor_pos: size,
        }
    }

    pub fn set_minify_filter(&mut self, filter_mode: MinifySamplerFilter) {
        self.font.set_minify_filter(filter_mode);
    }

    pub fn set_magnify_filter(&mut self, filter_mode: MagnifySamplerFilter) {
        self.font.set_magnify_filter(filter_mode);
    }

    pub fn use_linear_filtering(&mut self) {
        self.font.use_linear_filtering();
    }

    pub fn use_nearest_filtering(&mut self) {
        self.font.use_nearest_filtering();
    }

    pub fn texture(&mut self) -> TextureRef {
        self.font.texture()
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CharacterInfo {
    #[allow(unused)]
    pub offset: IVec2,
    pub advance: f32,
    pub sprite: SpriteKey,
}

#[derive(Clone, Copy, Debug)]
pub struct FontMetrics {
    pub ascent: f32,
    pub descent: f32,
    pub line_gap: f32,
}

impl FontMetrics {
    #[inline]
    pub fn line_height(&self) -> f32 {
        self.ascent - self.descent + self.line_gap
    }
}

#[derive(Default)]
pub struct TextDimensions {
    pub size: Vec2,
    pub final_cursor_pos: Vec2,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Glyph {
    character: char,
    size: usize, // using usize because i just dont like the idea of using a hashmap of f32 keys, right? that sounds bad right?
}

gen_ref_type!(Font, FontRef, fonts);

impl Default for FontRef {
    fn default() -> Self {
        MONO
    }
}

pub fn default_font() -> FontRef {
    FontRef(0)
}

#[derive(ErrorUnion, Debug)]
pub enum LoadFontError {
    Other(&'static str),
    Texture(glium::texture::TextureCreationError),
    Io(std::io::Error),
    ImageDecoding(sge_image::image_rs::ImageError),
    SgeImage(sge_image::SgeImageError),
    Bitmap(bitmap::BitmapFontDecodingError),
}

pub fn create_ttf_font(bytes: &[u8]) -> Result<FontRef, LoadFontError> {
    Font::vector_from_bytes(bytes).map(|f| f.create())
}

#[derive(Clone, Copy)]
pub struct TextDrawParams {
    pub font: Option<FontRef>,
    pub font_size: usize,
    pub color: Color,
    pub position: Vec2,
    pub line_spacing: f32,
}

#[bon::bon]
impl TextDrawParams {
    pub const DEFAULT: Self = Self {
        font: None,
        font_size: 16,
        color: Color::NEUTRAL_100,
        position: Vec2::ZERO,
        line_spacing: 1.0,
    };

    #[builder]
    pub fn builder(
        font: Option<FontRef>,
        font_size: Option<usize>,
        color: Option<Color>,
        position: Option<Vec2>,
        line_spacing: Option<f32>,
    ) -> Self {
        let d = Self::default();
        Self {
            font,
            font_size: font_size.unwrap_or(d.font_size),
            color: color.unwrap_or(d.color),
            position: position.unwrap_or(d.position),
            line_spacing: line_spacing.unwrap_or(d.line_spacing),
        }
    }
}

impl Default for TextDrawParams {
    fn default() -> Self {
        Self::DEFAULT
    }
}

pub const MONO: FontRef = FontRef(0);
#[cfg(feature = "extra_fonts")]
pub const SANS: FontRef = FontRef(1);
#[cfg(feature = "extra_fonts")]
pub const SANS_DISPLAY: FontRef = FontRef(2);
#[cfg(feature = "extra_fonts")]
pub const SANS_ITALIC: FontRef = FontRef(3);
#[cfg(feature = "extra_fonts")]
pub const SANS_BOLD: FontRef = FontRef(4);
#[cfg(feature = "extra_fonts")]
pub const SANS_BOLD_ITALIC: FontRef = FontRef(5);

#[rustfmt::skip]
pub(crate) fn init_fonts() -> Result<(), LoadFontError> {
    load_font_sync(include_bytes!("../assets/jetbrains.ttf")).map(|_| ())?;

    #[cfg(feature = "extra_fonts")]
    load_font_sync(include_bytes!("../assets/inter.ttf")).map(|_| ())?;
    #[cfg(feature = "extra_fonts")]
    load_font_sync(include_bytes!("../assets/inter-display-bold.ttf")).map(|_| ())?;
    #[cfg(feature = "extra_fonts")]
    load_font_sync(include_bytes!("../assets/inter-italic.ttf")).map(|_| ())?;
    #[cfg(feature = "extra_fonts")]
    load_font_sync(include_bytes!("../assets/inter-bold.ttf")).map(|_| ())?;
    #[cfg(feature = "extra_fonts")]
    load_font_sync(include_bytes!("../assets/inter-bold-italic.ttf")).map(|_| ())?;

    Ok(())
}

pub fn load_font_sync(bytes: &[u8]) -> Result<FontRef, LoadFontError> {
    Font::vector_from_bytes(bytes).map(|f| f.create())
}

pub fn load_bitmap_font_sync(
    bytes: &[u8],
    settings: &BitmapFontSettings,
) -> Result<FontRef, LoadFontError> {
    Font::bitmap_from_bytes(bytes, settings).map(|f| f.create())
}

pub fn init() -> Result<(), LoadFontError> {
    init_fonts_storage();
    init_fonts()?;

    Ok(())
}
