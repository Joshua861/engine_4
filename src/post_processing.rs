use bevy_math::Vec2;

pub enum PostProcessingEffect {
    Bloom { threshold: f32, radius: Vec2 },
    Blur { radius: Vec2 },
}

// pub struct EngineFramebuffer {
//     a: Option<SimpleFrameBuffer>,
//     b: Option<SimpleFrameBuffer>,
// }
