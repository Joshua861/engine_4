use bevy_math::Vec2;

use crate::color::Color;

pub enum PostProcessingEffect {
    Bloom { threshold: f32, radius: Vec2 },
    Blur { radius: Vec2 },
    Pixelate { pixel_width: f32 },
    Saturate(f32),
    HueRotate(f32),
    Brighten(f32),
    Vignette(Color),
}

// pub struct EngineFramebuffer {
//     a: Option<SimpleFrameBuffer>,
//     b: Option<SimpleFrameBuffer>,
// }
