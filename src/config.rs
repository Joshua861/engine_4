use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};

pub struct EngineConfig {
    // applies when loading a texture, not drawing
    //
    // setting this to true will make textures look better (less horrible and pixelated) from afer
    //
    // setting this to false will sometimes make images look crisper
    pub use_mipmaps: bool,
    pub default_magnify_filter: MagnifySamplerFilter,
    pub default_minify_filter: MinifySamplerFilter,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            use_mipmaps: true,
            default_magnify_filter: MagnifySamplerFilter::Nearest,
            default_minify_filter: MinifySamplerFilter::LinearMipmapLinear,
        }
    }
}
