use easy_ext::ext;
use sge_color::Color;
use sge_rendering::api::draw_texture_scaled;
use sge_textures::TextureRef;
use sge_types::{Area, SdfFill, SdfInstance};

use crate::shapes_2d::{draw_rect, draw_sdf};

#[ext(AreaExt)]
pub impl Area {
    fn fill(&self, color: Color) {
        draw_rect(self.top_left, self.size, color);
    }

    fn fill_pattern(
        &self,
        color_a: Color,
        color_b: Color,
        angle: f32,
        scale: f32,
        fill_type: SdfFill,
    ) {
        draw_sdf(
            SdfInstance::rect(self.center(), self.size)
                .with_fill(color_a, color_b, angle, scale, fill_type),
        );
    }

    fn draw_texture(&self, texture: TextureRef) {
        draw_texture_scaled(texture, self.top_left, self.size);
    }
}
