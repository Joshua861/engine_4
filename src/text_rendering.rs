use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Deref, DerefMut},
};

use bevy_math::{IVec2, Rect, Vec2};
use fontdue::{Metrics, layout::TextStyle};
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};

use crate::{
    api::{default_font, dpi_scaling},
    color::{Color, u8::Pixel},
    draw_queue_2d::DrawQueue2D,
    get_state,
    image::Image,
    prelude::{SpriteKey, TextureAtlas, Transform2D},
    textures::TextureRef,
};

pub struct EngineFont {
    font: fontdue::Font,
    atlas: TextureAtlas,
    characters: HashMap<Glyph, CharacterInfo>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CharacterInfo {
    #[allow(unused)]
    pub offset: IVec2,
    pub advance: f32,
    pub sprite: SpriteKey,
}

#[derive(Default)]
pub struct TextDimensions {
    pub size: Vec2,
    // pub offset_y: f32,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Glyph {
    character: char,
    size: usize, // using usize because i just dont like the idea of using a hashmap of f32 keys, right? that sounds bad right?
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct FontRef(pub usize);

#[allow(unused)]
impl FontRef {
    pub(crate) fn get(&self) -> &'static EngineFont {
        &get_state().storage.fonts[self.0]
    }

    pub(crate) fn get_mut(&self) -> &'static mut EngineFont {
        &mut get_state().storage.fonts[self.0]
    }

    pub fn new() -> Self {
        let id = get_state().storage.fonts.len();
        Self(id)
    }
}

impl Default for FontRef {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for FontRef {
    type Target = EngineFont;
    fn deref(&self) -> &Self::Target {
        &get_state().storage.fonts[self.0]
    }
}

impl DerefMut for FontRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut get_state().storage.fonts[self.0]
    }
}

impl EngineFont {
    pub(crate) fn load_from_bytes(bytes: &[u8]) -> anyhow::Result<EngineFont> {
        Self::load_from_bytes_with_atlas(TextureAtlas::new()?, bytes)
    }

    pub(crate) fn load_from_bytes_with_atlas(
        atlas: TextureAtlas,
        bytes: &[u8],
    ) -> anyhow::Result<Self> {
        Ok(Self {
            font: fontdue::Font::from_bytes(bytes, fontdue::FontSettings::default())
                .map_err(|e| anyhow::anyhow!(e))?,
            characters: HashMap::new(),
            atlas,
        })
    }

    #[allow(unused)]
    pub(crate) fn descent(&self, font_size: f32) -> f32 {
        self.font
            .horizontal_line_metrics(font_size)
            .unwrap()
            .descent
    }

    fn rasterize_glyph(&self, glyph: Glyph) -> (Metrics, Vec<u8>) {
        self.font.rasterize(glyph.character, glyph.size as f32)
    }

    pub(crate) fn cache_glyph(&mut self, glyph: Glyph) {
        if self.contains(glyph) {
            return;
        }

        let (metrics, bitmap) = self.rasterize_glyph(glyph);
        let sprite = self.atlas.cache_sprite(&Image::new(
            metrics.width,
            metrics.height,
            bitmap
                .iter()
                .map(|coverage| Pixel::from_rgba(255, 255, 255, *coverage))
                .collect(),
        ));
        let advance = metrics.advance_width;
        let offset = (metrics.xmin, metrics.ymin).into();

        let character_info = CharacterInfo {
            advance,
            offset,
            sprite,
        };

        self.characters.insert(glyph, character_info);
    }

    pub(crate) fn contains(&self, glyph: Glyph) -> bool {
        self.characters.contains_key(&glyph)
    }

    pub(crate) fn measure_text(
        &mut self,
        text: impl AsRef<str>,
        font_size: f32,
        do_dpi_scaling: bool,
    ) -> TextDimensions {
        let text = text.as_ref();

        if text.is_empty() {
            return TextDimensions::default();
        }

        let dpi_scaling = if do_dpi_scaling { dpi_scaling() } else { 1.0 };
        let font_size = (font_size * dpi_scaling).ceil();
        let mut layout =
            fontdue::layout::Layout::new(fontdue::layout::CoordinateSystem::PositiveYDown);
        layout.append(&[&self.font], &TextStyle::new(text, font_size, 0));

        let mut width = 0.0;

        for position in layout.glyphs() {
            let glyph = Glyph {
                character: position.parent,
                size: font_size as usize,
            };
            if !self.contains(glyph) {
                self.cache_glyph(glyph);
            }
            let char_info = self.characters[&glyph];
            width += char_info.advance;
        }

        TextDimensions {
            size: Vec2::new(width, layout.height()),
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
            self.cache_glyph(Glyph {
                character: *character,
                size,
            });
        }
    }

    pub fn set_minify_filter(&mut self, filter_mode: MinifySamplerFilter) {
        self.atlas.set_minify_filter(filter_mode);
    }

    pub fn set_magnify_filter(&mut self, filter_mode: MagnifySamplerFilter) {
        self.atlas.set_magnify_filter(filter_mode);
    }

    pub fn use_linear_filtering(&mut self) {
        self.atlas.use_linear_filtering();
    }

    pub fn use_nearest_filtering(&mut self) {
        self.atlas.use_nearest_filtering();
    }

    pub fn create(self) -> FontRef {
        let state = get_state();
        let id = state.storage.fonts.len();
        state.storage.fonts.push(self);
        FontRef(id)
    }

    pub fn texture(&mut self) -> TextureRef {
        self.atlas.texture().unwrap()
    }
}

pub fn create_ttf_font(bytes: &[u8]) -> anyhow::Result<FontRef> {
    EngineFont::load_from_bytes(bytes).map(|f| f.create())
}

#[derive(Clone, Copy)]
pub struct TextDrawParams {
    pub font: Option<FontRef>,
    pub font_size: usize,
    pub color: Color,
    pub position: Vec2,
    /// scale the font size by the DPI scaling of your monitor
    pub do_dpi_scaling: bool,
}

impl Default for TextDrawParams {
    fn default() -> Self {
        Self {
            font: None,
            font_size: 16,
            color: Color::WHITE,
            do_dpi_scaling: true,
            position: Vec2::ZERO,
        }
    }
}

pub(crate) fn init_fonts() {
    let _ = load_font(include_bytes!("../assets/fonts/jetbrains.ttf"));
}

pub fn draw_text_to(
    text: impl AsRef<str>,
    params: TextDrawParams,
    draw_queue: &mut DrawQueue2D,
) -> TextDimensions {
    let text = text.as_ref();
    let TextDrawParams {
        font,
        font_size,
        color,
        position: pos,
        do_dpi_scaling,
    } = params;

    if text.is_empty() {
        return TextDimensions::default();
    }

    let dpi_scaling = if do_dpi_scaling { dpi_scaling() } else { 1.0 };
    let font_size = (font_size as f32 * dpi_scaling).ceil();
    let mut font = font.unwrap_or(default_font());
    let mut layout = fontdue::layout::Layout::new(fontdue::layout::CoordinateSystem::PositiveYDown);
    layout.append(&[&font.font], &TextStyle::new(text, font_size, 0));

    let mut width = 0.0;

    for glyph_position in layout.glyphs() {
        let glyph = Glyph {
            character: glyph_position.parent,
            size: font_size as usize,
        };

        if !font.contains(glyph) {
            font.cache_glyph(glyph);
        }

        let char_info = font.characters[&glyph];
        let sprite = font.atlas.get(char_info.sprite).unwrap();
        let rect = sprite.rect;
        let rectf: Rect = rect.into();

        width += char_info.advance;

        let transform = Transform2D::from_scale_translation(
            Vec2::new(glyph_position.width as f32, glyph_position.height as f32),
            Vec2::new(glyph_position.x + pos.x, glyph_position.y + pos.y),
        );

        draw_queue.add_sprite(font.atlas.texture().unwrap(), transform, color, Some(rectf));
    }

    TextDimensions {
        size: Vec2::new(width, layout.height()),
    }
}

pub fn draw_text_ex(text: impl AsRef<str>, params: TextDrawParams) -> TextDimensions {
    draw_text_to(text, params, get_state().draw_queue_2d())
}

pub fn draw_text(text: impl AsRef<str>, position: Vec2) -> TextDimensions {
    draw_text_to(
        text,
        TextDrawParams {
            position,
            ..Default::default()
        },
        get_state().draw_queue_2d(),
    )
}

pub fn draw_text_size(text: impl AsRef<str>, position: Vec2, size: usize) -> TextDimensions {
    draw_text_to(
        text,
        TextDrawParams {
            position,
            font_size: size,
            ..Default::default()
        },
        get_state().draw_queue_2d(),
    )
}

pub fn draw_text_world_ex(text: impl AsRef<str>, params: TextDrawParams) -> TextDimensions {
    draw_text_to(text, params, get_state().world_draw_queue_2d())
}

pub fn draw_text_world(text: impl AsRef<str>, position: Vec2) -> TextDimensions {
    draw_text_to(
        text,
        TextDrawParams {
            position,
            ..Default::default()
        },
        get_state().world_draw_queue_2d(),
    )
}

pub fn draw_text_size_world(text: impl AsRef<str>, position: Vec2, size: usize) -> TextDimensions {
    draw_text_to(
        text,
        TextDrawParams {
            position,
            font_size: size,
            ..Default::default()
        },
        get_state().world_draw_queue_2d(),
    )
}

pub fn load_font(bytes: &[u8]) -> Result<FontRef, anyhow::Error> {
    EngineFont::load_from_bytes(bytes).map(|f| f.create())
}

pub fn measure_text(text: impl AsRef<str>) -> TextDimensions {
    measure_text_ex(text, TextDrawParams::default())
}

/// Does not take translatation, rotation or colour into account. Feel free to exclude them.
pub fn measure_text_ex(text: impl AsRef<str>, params: TextDrawParams) -> TextDimensions {
    let TextDrawParams {
        font,
        font_size,
        do_dpi_scaling,
        ..
    } = params;

    font.unwrap_or(default_font())
        .measure_text(text, font_size as f32, do_dpi_scaling)
}
