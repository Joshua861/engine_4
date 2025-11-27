use std::ops::{Deref, DerefMut};

use bevy_math::{UVec2, Vec2};

use crate::get_state;

pub struct TextDimensions {
    pub width: f32,
    pub height: f32,
    pub offset_y: f32,
}

pub struct EngineFont {
    font: fontdue::Font,
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct FontRef(pub usize);

impl FontRef {
    pub(crate) fn get(&self) -> &'static EngineFont {
        &get_state().storage.fonts[self.0]
    }

    pub fn get_mut(&self) -> &'static mut EngineFont {
        &mut get_state().storage.fonts[self.0]
    }

    pub fn new() -> Self {
        let id = get_state().storage.fonts.len();
        Self(id)
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
