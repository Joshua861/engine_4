use std::collections::HashMap;

use sge_color::Color;
use sge_image::Image;
use sge_image::image_rs as image;
use sge_image::image_rs::GenericImageView;
use sge_math::transform::Transform2D;
use sge_math::usize_rect::USizeRect;
use sge_rendering::api::draw_texture_to_ex;
use sge_rendering::d2::Renderer2D;
use sge_texture_atlas::Sprite;
use sge_texture_atlas::SpriteKey;
use sge_texture_atlas::TextureAtlas;
use sge_textures::SgeTexture;
use sge_vectors::UVec2;
use sge_vectors::Vec2;
use sge_vectors::uvec2;
use sge_vectors::vec2;
use thiserror::Error;

use crate::FontMetrics;
use crate::LoadFontError;
use crate::{CharacterInfo, FontSource};

pub struct BitmapFont {
    atlas: TextureAtlas,
    characters: HashMap<char, CharacterInfo>,
    char_size_y: u32,
}

#[derive(Clone, Copy, Debug)]
pub enum BitmapFontProcessing {
    Alpha,
    Brightness,
    InverseBrightness,
    FullColor,
}

pub struct BitmapFontSettings {
    pub char_size: UVec2,
    pub advance: u32,
    pub gaps_in: u32,
    pub gaps_out: GapsOut,
    pub processing: BitmapFontProcessing,
    pub layout: String,
}

pub struct GapsOut {
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
}

#[derive(Debug, Error)]
pub enum BitmapFontDecodingError {
    #[error("Layout overflows availible space. Maximum Y, with gaps = {max_y}")]
    LayoutOverflows { max_y: u32 },
}

impl GapsOut {
    pub const ZERO: Self = Self::all(0);

    pub const fn all(gap: u32) -> Self {
        Self::new(gap, gap, gap, gap)
    }

    pub const fn new(top: u32, bottom: u32, left: u32, right: u32) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }

    pub const fn xy(x: u32, y: u32) -> Self {
        Self::new(y, y, x, x)
    }
}

impl BitmapFont {
    pub fn load_from_bytes(
        bytes: &[u8],
        settings: &BitmapFontSettings,
    ) -> Result<Self, LoadFontError> {
        let image = image::load_from_memory(bytes)?;
        let (width, height) = image.dimensions();
        let bytes = image.into_bytes();
        let image = Image::from_bytes(width as usize, height as usize, bytes)?;

        Self::load_from_image(image, settings)
    }

    fn process_image(image: &mut Image, processing: BitmapFontProcessing) {
        match processing {
            BitmapFontProcessing::FullColor => {}
            BitmapFontProcessing::Alpha => {
                for (_, _, color) in unsafe { image.iter_mut() } {
                    let rgba = color.rgba_mut();
                    rgba.r = 255;
                    rgba.g = 255;
                    rgba.b = 255;
                }
            }
            BitmapFontProcessing::Brightness => {
                for (_, _, color) in unsafe { image.iter_mut() } {
                    let rgba = color.rgba_mut();
                    let brightness = rgba.r / 3 + rgba.b / 3 + rgba.g / 3;
                    rgba.r = 255;
                    rgba.g = 255;
                    rgba.b = 255;
                    rgba.a = brightness;
                }
            }
            BitmapFontProcessing::InverseBrightness => {
                for (_, _, color) in unsafe { image.iter_mut() } {
                    let rgba = color.rgba_mut();
                    let brightness = rgba.r / 3 + rgba.b / 3 + rgba.g / 3;
                    let brightness = 255 - brightness;
                    rgba.r = 255;
                    rgba.g = 255;
                    rgba.b = 255;
                    rgba.a = brightness;
                }
            }
        }
    }

    pub fn load_from_image(
        mut image: Image,
        settings: &BitmapFontSettings,
    ) -> Result<Self, LoadFontError> {
        let dim = image.dimensions_u32();
        let max_x = dim.x - settings.gaps_out.right;
        let max_y = dim.y - settings.gaps_out.bottom;

        let mut characters = HashMap::new();

        Self::process_image(&mut image, settings.processing);
        let texture = SgeTexture::from_engine_image(image.clone())?.create();
        let mut atlas = unsafe { TextureAtlas::from_image_and_texture(image, texture) };

        let mut cursor = uvec2(settings.gaps_out.left, settings.gaps_out.top);

        for c in settings.layout.chars() {
            if cursor.x + settings.char_size.x > max_x {
                cursor.x = settings.gaps_out.left;
                cursor.y += settings.gaps_in + settings.char_size.y;
            }

            if cursor.y + settings.char_size.y > max_y {
                return Err(LoadFontError::Bitmap(
                    BitmapFontDecodingError::LayoutOverflows { max_y },
                ));
            }

            let key = unsafe {
                atlas.insert(Sprite::new(USizeRect::new(
                    cursor.x as usize,
                    cursor.y as usize,
                    (cursor.x + settings.char_size.x) as usize,
                    (cursor.y + settings.char_size.y) as usize,
                )))
            };

            let info = CharacterInfo {
                offset: settings.char_size.as_ivec2(),
                advance: settings.advance as f32,
                sprite: key,
            };

            characters.insert(c, info);

            cursor.x += settings.gaps_in + settings.char_size.x;
        }

        Ok(Self {
            atlas,
            characters,
            char_size_y: settings.char_size.y,
        })
    }
}

impl FontSource for BitmapFont {
    fn character_info(&mut self, glyph: crate::Glyph) -> CharacterInfo {
        let scale_factor = (glyph.size as f32 / self.char_size_y as f32).max(1.0);

        let mut info = match self.characters.get(&glyph.character) {
            Some(c) => c.clone(),
            None => self
                .characters
                .get(&' ')
                .expect("bitmap font does not contain space character")
                .clone(),
        };

        info.advance *= scale_factor;

        info.offset.x = (info.offset.x as f32 * scale_factor) as i32;
        info.offset.y = (info.offset.y as f32 * scale_factor) as i32;

        info
    }

    fn texture_atlas(&self) -> &TextureAtlas {
        &self.atlas
    }

    fn texture_atlas_mut(&mut self) -> &mut TextureAtlas {
        &mut self.atlas
    }

    fn contains(&self, glyph: crate::Glyph) -> bool {
        self.characters.contains_key(&glyph.character)
    }

    fn font_metrics(&self, font_size: usize) -> crate::FontMetrics {
        let scale_factor = (font_size as f32 / self.char_size_y as f32).max(1.0);
        FontMetrics {
            ascent: (scale_factor * self.char_size_y as f32),
            descent: 0.0,
            line_gap: 0.0,
        }
    }

    fn draw_sprite(
        &mut self,
        key: SpriteKey,
        color: Color,
        position: Vec2,
        renderer: Renderer2D,
        font_size: usize,
    ) -> Option<()> {
        let sprite = self.texture_atlas().get(key)?;
        let sprite_size = sprite.rect.size();

        let base_scale = vec2(sprite_size.x as f32, sprite_size.y as f32);
        let scale_factor = (font_size as f32 / self.char_size_y as f32).max(1.0);

        let transform = Transform2D::from_scale_translation(base_scale * scale_factor, position);
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
}
