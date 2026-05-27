use d2::{DrawQueue2D, Renderer2D};
use d3::DrawQueue3D;
use glium::{
    Rect, Surface, Texture2d,
    framebuffer::{DepthRenderBuffer, RenderBuffer, SimpleFrameBuffer},
    texture::{DepthTexture2d, TextureCreationError},
    uniform,
};
use materials::init_materials;
use pipeline::{RenderPipeline, RenderTarget, RenderTexture, empty_render_texture};
use post_processing::render_fullscreen_quad;
use sge_image::Image;
use sge_programs::COPY_PROGRAM;
use sge_textures::{SgeTexture, TextureRef};
use sge_types::MetaballBatch;
use sge_vectors::UVec2;
use sge_window::{get_display, window_size};

pub mod api;
pub mod d2;
pub mod d3;
pub mod materials;
pub mod object_3d;
pub mod pipeline;
pub mod post_processing;
pub mod scissor;
pub mod shapes_3d;

pub struct DrawQueues {
    pub draw_queue_2d: DrawQueue2D,
    pub world_draw_queue_2d: DrawQueue2D,
    pub draw_queue_3d: DrawQueue3D,
}

pub struct RenderState {
    pub render_pipeline: RenderPipeline,
    pub texture_pipeline: Option<RenderPipeline>,
    pub scissor_stack: Vec<Rect>,
    pub a: RenderTexture,
    pub b: RenderTexture,
}

impl RenderState {
    fn rts(output: RenderTarget) -> Result<(RenderTexture, RenderTexture), TextureCreationError> {
        let (w, h) = match output {
            RenderTarget::Screen => {
                let size = window_size().round();
                (size.x as u32, size.y as u32)
            }
            RenderTarget::Texture(rt) => {
                let dimensions = rt.color_texture.get().dimensions;
                (dimensions.x, dimensions.y)
            }
        };
        Ok((empty_render_texture(w, h)?, empty_render_texture(w, h)?))
    }

    fn update_rts<T: Surface>(&mut self, frame: &mut T) {
        let dimensions = frame.get_dimensions();
        let (w, h) = (dimensions.0, dimensions.1);
        let facade = get_display();
        let samples = 4;

        if self.a.dimensions.x != w || self.a.dimensions.y != h {
            for tex in [&mut self.a, &mut self.b] {
                tex.color_texture.gl_texture = Texture2d::empty(facade, w, h).unwrap();
                tex.depth_texture = DepthTexture2d::empty(facade, w, h).unwrap();
                tex.msaa_color = RenderBuffer::new_multisample(
                    facade,
                    glium::texture::UncompressedFloatFormat::U8U8U8U8,
                    w,
                    h,
                    samples,
                )
                .unwrap();
                tex.msaa_depth = DepthRenderBuffer::new_multisample(
                    facade,
                    glium::texture::DepthFormat::I24,
                    w,
                    h,
                    samples,
                )
                .unwrap();
                tex.dimensions = UVec2::new(w, h);
                tex.color_texture.normalized_dimensions =
                    SgeTexture::create_normalized_dimensions(w, h);
                tex.color_texture.dimensions = UVec2::new(w, h);
            }
        }
    }
}

sge_global::global!(RenderState, render_state);

pub fn init() {
    object_3d::init();
    pipeline::init();

    let (a, b) = RenderState::rts(RenderTarget::Screen).unwrap();
    set_render_state(RenderState {
        render_pipeline: RenderPipeline::screen(),
        texture_pipeline: None,
        scissor_stack: vec![],
        a,
        b,
    });
    log::info!("Initialized render state");
    init_materials();
    log::info!("Initialized materials");
    unsafe { MetaballBatch::init_storage() };
}

pub fn dq2d() -> Renderer2D {
    get_render_state()
        .render_pipeline
        .draw_queue_2d()
        .renderer()
}

pub fn wdq2d() -> Renderer2D {
    get_render_state()
        .render_pipeline
        .world_draw_queue_2d()
        .renderer()
}

pub fn take_screenshot() -> TextureRef {
    texture_create_clone(get_render_state().a.color_texture)
}

pub fn take_screenshot_image() -> Image {
    get_render_state().a.color_texture.download_to_image()
}

pub fn window_texture() -> TextureRef {
    get_render_state().a.color_texture
}

pub fn texture_create_clone(texture: TextureRef) -> TextureRef {
    let facade = get_display();
    let w = texture.gl_texture.width();
    let h = texture.gl_texture.height();

    let new_texture = Texture2d::empty(facade, w, h).unwrap();

    {
        let mut fb = SimpleFrameBuffer::new(facade, &new_texture).unwrap();
        let uniforms = uniform! { tex: texture.gl_texture.sampled() };
        render_fullscreen_quad(&mut fb, COPY_PROGRAM.get(), &uniforms).unwrap();
    }

    SgeTexture {
        gl_texture: new_texture,
        dimensions: texture.dimensions,
        normalized_dimensions: texture.normalized_dimensions,
        magnify_filter: texture.magnify_filter,
        minify_filter: texture.minify_filter,
    }
    .create()
}
