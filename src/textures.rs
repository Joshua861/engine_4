use std::io::Cursor;

use bevy_math::{UVec2, Vec2};
use glium::{Texture2d, implement_vertex, texture::RawImage2d};
use image::ImageFormat;

use crate::get_state;

#[derive(Clone, Copy)]
pub struct TexturedVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(TexturedVertex, position, tex_coords);

pub(crate) const UNIT_SQUARE: [TexturedVertex; 4] = [
    TexturedVertex {
        position: [-1.0, -1.0],
        tex_coords: [0.0, 0.0],
    },
    TexturedVertex {
        position: [1.0, -1.0],
        tex_coords: [1.0, 0.0],
    },
    TexturedVertex {
        position: [-1.0, 1.0],
        tex_coords: [0.0, 1.0],
    },
    TexturedVertex {
        position: [1.0, 1.0],
        tex_coords: [1.0, 1.0],
    },
];

pub fn load_texture(bytes: &[u8], format: ImageFormat) -> anyhow::Result<TextureRef> {
    let state = get_state();

    let image = image::load(Cursor::new(bytes), format)?.to_rgba8();
    let image_dimensions = image.dimensions();
    let image = RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);
    let texture = Texture2d::new(&state.display, image)?;
    let dimensions = image_dimensions.into();
    let texture = EngineTexture {
        dimensions,
        gl_texture: texture,
        normalized_dimensions: {
            let mut v = Vec2::new(dimensions.x as f32, dimensions.y as f32);
            let max = v.x.max(v.y);
            v.x /= max;
            v.y /= max;
            v
        },
    };

    let id = state.storage.textures.len();
    state.storage.textures.push(texture);

    Ok(TextureRef { id })
}

#[derive(Clone, Copy)]
pub struct TextureRef {
    id: usize,
}

pub struct EngineTexture {
    pub dimensions: UVec2,
    pub normalized_dimensions: Vec2,
    pub gl_texture: Texture2d,
}

impl TextureRef {
    pub(crate) fn get(&self) -> &'static EngineTexture {
        &get_state().storage.textures[self.id]
    }

    pub(crate) fn get_mut(&self) -> &'static mut EngineTexture {
        &mut get_state().storage.textures[self.id]
    }

    pub fn dimensions(&self) -> UVec2 {
        self.get().dimensions
    }

    pub fn normalized_dimensions(&self) -> Vec2 {
        self.get().normalized_dimensions
    }
}
