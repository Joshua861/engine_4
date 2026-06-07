use std::collections::HashMap;

use fontdue::Metrics;
use log::warn;
use sge_color::u8::ColorU8;
use sge_image::Image;
use sge_texture_atlas::TextureAtlas;
use sge_vectors::ivec2;

use crate::{CharacterInfo, FontSource, Glyph, LoadFontError};

pub struct VectorFont {
    font: fontdue::Font,
    atlas: TextureAtlas,
    characters: HashMap<Glyph, CharacterInfo>,
}

impl VectorFont {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, LoadFontError> {
        Ok(Self {
            font: fontdue::Font::from_bytes(bytes, fontdue::FontSettings::default())?,
            characters: HashMap::new(),
            atlas: TextureAtlas::new()?,
        })
    }

    fn rasterize_glyph(&self, glyph: Glyph) -> (Metrics, Vec<u8>) {
        self.font.rasterize(glyph.character, glyph.size as f32)
    }

    fn cache_glyph(&mut self, glyph: Glyph) -> CharacterInfo {
        let (metrics, bitmap) = self.rasterize_glyph(glyph);
        let sprite = self.atlas.cache_sprite(&Image::new(
            metrics.width,
            metrics.height,
            bitmap
                .iter()
                .map(|&coverage| ColorU8::from_rgba(255, 255, 255, coverage))
                .collect(),
        ));
        let advance = metrics.advance_width;

        let offset = ivec2(metrics.xmin, metrics.ymin + metrics.height as i32);

        let character_info = CharacterInfo {
            advance,
            offset,
            sprite,
        };

        self.characters.insert(glyph, character_info);

        character_info
    }

    fn get_glyph_unchecked(&self, glyph: Glyph) -> CharacterInfo {
        self.characters[&glyph]
    }
}

impl FontSource for VectorFont {
    fn character_info(&mut self, glyph: Glyph) -> CharacterInfo {
        if self.contains(glyph) {
            self.get_glyph_unchecked(glyph)
        } else {
            self.cache_glyph(glyph)
        }
    }

    fn font_metrics(&self, font_size: usize) -> crate::FontMetrics {
        if let Some(metrics) = self.font.horizontal_line_metrics(font_size as f32) {
            crate::FontMetrics {
                ascent: metrics.ascent,
                descent: metrics.descent,
                line_gap: metrics.line_gap,
            }
        } else {
            warn!("line metrics missing from font file header");
            crate::FontMetrics {
                ascent: font_size as f32,
                descent: 0.0,
                line_gap: 0.0,
            }
        }
    }

    fn contains(&self, glyph: Glyph) -> bool {
        self.characters.contains_key(&glyph)
    }

    fn texture_atlas(&self) -> &TextureAtlas {
        &self.atlas
    }

    fn texture_atlas_mut(&mut self) -> &mut TextureAtlas {
        &mut self.atlas
    }
}
