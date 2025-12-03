use std::collections::HashMap;
use std::io::Cursor;

use super::{EngineTexture, TextureRef};
use crate::api::{draw_texture_ex, draw_texture_world_ex, window_size};
use crate::color::Color;
use crate::get_state;
use crate::image::{Image, ImageRef};
use crate::prelude::Transform2D;
use crate::utils::EngineCreate;
use crate::utils::usize_rect::USizeRect;

use bevy_math::{USizeVec2, Vec2};
use engine_4_macros::gen_ref_type;
use glium::{
    texture::{RawImage2d, TextureCreationError},
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter},
};
use image::ImageFormat;

#[derive(Clone, Copy)]
pub struct Sprite {
    pub rect: USizeRect,
    pub normalized_dimensions: Vec2,
}

pub struct TextureAtlas {
    pub texture: TextureRef,
    pub sprites: HashMap<SpriteKey, Sprite>,
    cursor: USizeVec2,
    max_line_height: usize,
    dirty: bool,
    image: Image,
    next_id: usize,
}

gen_ref_type!(TextureAtlas, TextureAtlasRef, texture_atlasses);

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Debug)]
pub struct SpriteKey(usize);

impl TextureAtlas {
    const DEFAULT_WIDTH: usize = 512;
    const DEFAULT_HEIGHT: usize = 512;
    const GAP: usize = 2;
    const SCALING_FACTOR: usize = 2;

    pub fn new() -> Result<TextureAtlas, TextureCreationError> {
        Self::new_with_size(Self::DEFAULT_WIDTH, Self::DEFAULT_HEIGHT)
    }

    pub fn new_with_size(
        width: usize,
        height: usize,
    ) -> Result<TextureAtlas, TextureCreationError> {
        let texture = EngineTexture::empty(width as u32, height as u32)?.create();
        let image = Image::empty(width, height);

        Ok(TextureAtlas {
            texture,
            sprites: HashMap::new(),
            cursor: USizeVec2::ZERO,
            dirty: false,
            image,
            max_line_height: 0,
            next_id: 0,
        })
    }

    pub fn set_magnify_filter(&mut self, filtering: MagnifySamplerFilter) {
        self.texture.magnify_filter = filtering;
    }

    pub fn set_minify_filter(&mut self, filtering: MinifySamplerFilter) {
        self.texture.minify_filter = filtering;
    }

    pub fn use_linear_filtering(&mut self) {
        self.texture.magnify_filter = MagnifySamplerFilter::Linear;
        self.texture.minify_filter = MinifySamplerFilter::Linear;
    }

    pub fn use_nearest_filtering(&mut self) {
        self.texture.magnify_filter = MagnifySamplerFilter::Nearest;
        self.texture.minify_filter = MinifySamplerFilter::Nearest;
    }

    pub fn get(&self, key: SpriteKey) -> Option<Sprite> {
        self.sprites.get(&key).copied()
    }

    pub fn width(&self) -> u32 {
        self.texture.dimensions.x
    }

    pub fn height(&self) -> u32 {
        self.texture.dimensions.y
    }

    pub(crate) fn texture(&mut self) -> Result<TextureRef, TextureCreationError> {
        let state = get_state();

        if self.dirty {
            self.dirty = false;

            let tex_size = self.texture.dimensions;

            if tex_size != self.image.dimensions_u32() {
                let new_texture = EngineTexture::from_engine_image(self.image.clone())?;
                state.storage[self.texture] = new_texture;
            } else {
                let raw_image = RawImage2d::from_raw_rgba(
                    self.image.clone().into_bytes(),
                    self.image.dimensions_u32().into(),
                );

                let texture = &mut state.storage[self.texture];
                texture.gl_texture.write(
                    glium::Rect {
                        left: 0,
                        bottom: 0,
                        width: raw_image.width,
                        height: raw_image.height,
                    },
                    raw_image,
                );
            }
        }

        Ok(self.texture)
    }

    // TODO: test if this works properly
    pub fn get_uv_rect(&self, key: SpriteKey) -> Option<USizeRect> {
        self.get(key).map(|mut sprite| {
            let dim = self.texture.dimensions.as_usizevec2();

            sprite.rect.min /= dim;
            sprite.rect.max /= dim;
            sprite.rect
        })
    }

    fn cache_sprite_with_key(&mut self, key: SpriteKey, sprite: &Image) {
        let dim = sprite.dimensions();

        let x = if self.cursor.x + dim.x < self.image.width() {
            if dim.y > self.max_line_height {
                self.max_line_height = dim.y;
            }
            let res = self.cursor.x + Self::GAP;
            self.cursor.x += dim.x + Self::GAP * 2;
            res
        } else {
            self.cursor.y += self.max_line_height + Self::GAP * 2;
            self.cursor.x = dim.x + Self::GAP;
            self.max_line_height = dim.y;
            Self::GAP
        };
        let y = self.cursor.y;

        if y + sprite.height() > self.image.height() || x + sprite.width() > self.image.width() {
            let sprites = std::mem::take(&mut self.sprites);
            self.cursor = USizeVec2::ZERO;
            self.max_line_height = 0;

            let old_image = self.image.clone();

            let new_width = self.image.width() * Self::SCALING_FACTOR;
            let new_height = self.image.height() * Self::SCALING_FACTOR;

            self.image = Image::empty(new_width, new_height);

            for (key, sprite) in sprites {
                let image = old_image.sub_image(sprite.rect);
                self.cache_sprite_with_key(key, &image);
            }

            self.cache_sprite_with_key(key, sprite);
        } else {
            self.dirty = true;

            for j in 0..dim.y {
                for i in 0..dim.x {
                    self.image
                        .set(x + i, y + j, *sprite.get_pixel(i, j).unwrap());
                }
            }

            self.sprites.insert(
                key,
                Sprite {
                    rect: USizeRect::new(x, y, x + dim.x, y + dim.y),
                    normalized_dimensions: EngineTexture::create_normalized_dimensions(
                        dim.x as u32,
                        dim.y as u32,
                    ),
                },
            );
        }
    }

    fn gen_key(&mut self) -> SpriteKey {
        let key = SpriteKey(self.next_id);
        self.next_id += 1;
        key
    }

    pub fn cache_sprite(&mut self, image: &Image) -> SpriteKey {
        let key = self.gen_key();
        self.cache_sprite_with_key(key, image);
        key
    }

    pub fn unregister_sprite(&mut self, key: SpriteKey) {
        self.sprites.remove(&key);
    }

    pub fn draw(&mut self, sprite: SpriteKey, position: Vec2, scale: f32) -> Option<()> {
        let sprite = self.get(sprite)?;

        draw_texture_ex(
            self.texture().ok()?,
            Transform2D::from_scale_translation(sprite.normalized_dimensions * scale, position),
            Color::WHITE,
            Some(sprite.rect.as_rect()),
        );

        Some(())
    }

    pub fn draw_fullscreen(&mut self, sprite: SpriteKey) -> Option<()> {
        self.draw_scaled(sprite, Vec2::ZERO, window_size())
    }

    pub fn draw_world(&mut self, sprite: SpriteKey, position: Vec2, scale: f32) -> Option<()> {
        let sprite = self.get(sprite)?;

        draw_texture_world_ex(
            self.texture().ok()?,
            Transform2D::from_scale_translation(sprite.normalized_dimensions * scale, position),
            Color::WHITE,
            Some(sprite.rect.as_rect()),
        );

        Some(())
    }

    pub fn draw_scaled(&mut self, sprite: SpriteKey, position: Vec2, scale: Vec2) -> Option<()> {
        let sprite = self.get(sprite)?;

        draw_texture_ex(
            self.texture().ok()?,
            Transform2D::from_scale_translation(scale, position),
            Color::WHITE,
            Some(sprite.rect.as_rect()),
        );

        Some(())
    }

    pub fn draw_scaled_world(
        &mut self,
        sprite: SpriteKey,
        position: Vec2,
        scale: Vec2,
    ) -> Option<()> {
        let sprite = self.get(sprite)?;

        draw_texture_ex(
            self.texture().ok()?,
            Transform2D::from_scale_translation(scale, position),
            Color::WHITE,
            Some(sprite.rect.as_rect()),
        );

        Some(())
    }

    pub fn draw_ex(
        &mut self,
        sprite: SpriteKey,
        transform: Transform2D,
        color: Color,
    ) -> Option<()> {
        let sprite = self.get(sprite)?;

        draw_texture_ex(
            self.texture().ok()?,
            transform,
            color,
            Some(sprite.rect.as_rect()),
        );

        Some(())
    }

    pub fn draw_world_ex(
        &mut self,
        sprite: SpriteKey,
        transform: Transform2D,
        color: Color,
    ) -> Option<()> {
        let sprite = self.get(sprite)?;

        draw_texture_ex(
            self.texture().ok()?,
            transform,
            color,
            Some(sprite.rect.as_rect()),
        );

        Some(())
    }
}

pub fn create_spritesheet() -> Result<TextureAtlasRef, TextureCreationError> {
    TextureAtlas::new().map(|a| a.create())
}

pub fn load_image(bytes: &[u8], format: ImageFormat) -> anyhow::Result<ImageRef> {
    let image = image::load(Cursor::new(bytes), format)?.to_rgba8();
    let dim = image.dimensions();
    Ok(Image::from_bytes(dim.0 as usize, dim.1 as usize, image.into_raw())?.create())
}
