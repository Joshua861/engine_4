use crate::d3::DrawQueue3D;
use crate::get_render_state;
use crate::post_processing::PostProcessingEffect;
use crate::{DrawQueues, d2::DrawQueue2D};
use glium::Texture2d;
use glium::framebuffer::{DepthRenderBuffer, RenderBuffer};
use glium::{
    Surface,
    framebuffer::SimpleFrameBuffer,
    texture::{DepthTexture2d, TextureCreationError},
    uniform,
};
use sge_camera::{Cameras, get_cameras};
use sge_color::Color;
use sge_macros::gen_ref_type;
use sge_programs::COPY_PROGRAM;
use sge_textures::{SgeTexture, TextureRef};
use sge_vectors::{Mat4, UVec2, Vec2, Vec3};
use sge_window::{get_display, get_display_mut, get_window_state};

const SAMPLES: u32 = 4;

pub struct RenderTexture {
    pub dimensions: UVec2,
    pub color_texture: TextureRef,
    pub depth_texture: DepthTexture2d,
    pub msaa_color: RenderBuffer,
    pub msaa_depth: DepthRenderBuffer,
}

impl RenderTexture {
    pub fn framebuffer(&mut self) -> SimpleFrameBuffer<'_> {
        let window = get_window_state();
        SimpleFrameBuffer::with_depth_buffer(&window.display, &self.msaa_color, &self.msaa_depth)
            .unwrap()
    }

    pub fn resolve(&self) {
        let display = get_display();
        let msaa_fbo = SimpleFrameBuffer::new(display, &self.msaa_color).unwrap();
        let resolved_fbo =
            SimpleFrameBuffer::new(display, &self.color_texture.get().gl_texture).unwrap();
        msaa_fbo.blit_whole_color_to(
            &resolved_fbo,
            &glium::BlitTarget {
                left: 0,
                bottom: 0,
                width: self.dimensions.x as i32,
                height: self.dimensions.y as i32,
            },
            glium::uniforms::MagnifySamplerFilter::Linear,
        );
    }
}

gen_ref_type!(RenderTexture, RenderTextureRef, render_textures);

impl RenderTextureRef {
    pub fn dimensions(&self) -> UVec2 {
        self.get().dimensions
    }

    pub fn framebuffer(&self) -> SimpleFrameBuffer<'_> {
        self.get_mut().framebuffer()
    }
}

#[derive(Clone, Copy)]
pub enum RenderTarget {
    Screen,
    Texture(RenderTextureRef),
}

pub struct RenderPipeline {
    pub steps: Vec<RenderStep>,
    pub output: RenderTarget,
    pub clear_color: ClearColor,
    pub camera_override: Option<Cameras>,
}

pub enum ClearColor {
    Clear(Color),
    DontClear,
    Default,
}

impl From<Color> for ClearColor {
    fn from(value: Color) -> Self {
        Self::Clear(value)
    }
}

// i dont care 😝😝😝😝😝
#[allow(clippy::large_enum_variant)]
pub enum RenderStep {
    Drawing(DrawQueues),
    PostProcessing(PostProcessingStep),
}

impl DrawQueues {
    pub fn empty() -> Self {
        let draw_queue_2d = DrawQueue2D::screen();
        let world_draw_queue_2d = DrawQueue2D::world();
        let draw_queue_3d = DrawQueue3D::empty();

        Self {
            draw_queue_2d,
            draw_queue_3d,
            world_draw_queue_2d,
        }
    }
}

pub struct PostProcessingStep(pub Vec<PostProcessingEffect>);

impl RenderPipeline {
    pub fn draw_queues(&mut self) -> &mut DrawQueues {
        if !matches!(self.most_recent_step(), Some(RenderStep::Drawing(_))) {
            self.steps.push(RenderStep::Drawing(DrawQueues::empty()));
        }

        let len = self.steps.len() - 1;
        match &mut self.steps[len] {
            RenderStep::Drawing(draw) => draw,
            RenderStep::PostProcessing(_) => unreachable!(),
        }
    }

    pub fn post_processing_effects(&mut self) -> &mut PostProcessingStep {
        if !matches!(self.most_recent_step(), Some(RenderStep::PostProcessing(_))) {
            self.steps
                .push(RenderStep::PostProcessing(PostProcessingStep(Vec::new())));
        }

        let len = self.steps.len() - 1;
        match &mut self.steps[len] {
            RenderStep::PostProcessing(p) => p,
            RenderStep::Drawing(_) => unreachable!(),
        }
    }

    pub fn draw_queue_2d(&mut self) -> &mut DrawQueue2D {
        &mut self.draw_queues().draw_queue_2d
    }

    pub fn world_draw_queue_2d(&mut self) -> &mut DrawQueue2D {
        &mut self.draw_queues().world_draw_queue_2d
    }

    pub fn draw_queue_3d(&mut self) -> &mut DrawQueue3D {
        &mut self.draw_queues().draw_queue_3d
    }

    pub fn most_recent_step(&self) -> Option<&RenderStep> {
        self.steps.last()
    }

    pub fn cameras(&self) -> Cameras {
        match self.camera_override {
            Some(c) => c,
            None => *get_cameras(),
        }
    }

    pub fn add_effect(&mut self, effect: PostProcessingEffect) {
        self.post_processing_effects().0.push(effect);
    }

    pub fn new(output: RenderTarget, camera_override: Option<Cameras>) -> Self {
        Self {
            steps: vec![],
            output,
            clear_color: ClearColor::Default,
            camera_override,
        }
    }

    pub fn draw(&mut self) {
        match self.output {
            RenderTarget::Screen => {
                self.draw_on(&mut get_display_mut().draw());
            }
            RenderTarget::Texture(rt) => {
                let rt_mut = rt.get_mut();
                let texture = rt_mut.color_texture;
                let mut framebuffer = SimpleFrameBuffer::with_depth_buffer(
                    get_display(),
                    &texture.gl_texture,
                    &rt_mut.depth_texture,
                )
                .unwrap();
                self.draw_on(&mut framebuffer);
            }
        }
    }

    pub fn draw_on<T: Surface>(&mut self, frame: &mut T) {
        let mut cameras = self.cameras();
        let is_texture_target = matches!(self.output, RenderTarget::Texture(_));

        let dimensions = frame.get_dimensions();

        get_render_state().update_rts(frame);

        let a = &mut get_render_state().a;
        let b = &mut get_render_state().b;

        match self.clear_color {
            ClearColor::DontClear => (),
            ClearColor::Default => {
                a.framebuffer().clear_color(0.0, 0.0, 0.0, 1.0);
                b.framebuffer().clear_color(0.0, 0.0, 0.0, 1.0);
            }
            ClearColor::Clear(c) => {
                a.framebuffer().clear_color(c.r, c.g, c.b, c.a);
                b.framebuffer().clear_color(c.r, c.g, c.b, c.a);
            }
        }

        for step in std::mem::take(&mut self.steps) {
            a.framebuffer().clear_depth(1.0);
            b.framebuffer().clear_depth(1.0);

            match step {
                RenderStep::Drawing(draw_queues) => {
                    RenderPipeline::draw_queues_to(
                        &mut a.framebuffer(),
                        draw_queues,
                        &mut cameras,
                        is_texture_target,
                    );
                    a.resolve();
                }
                RenderStep::PostProcessing(effects) => {
                    for effect in effects.0 {
                        effect
                            .apply(
                                a.color_texture,
                                &mut b.framebuffer(),
                                Vec2::new(dimensions.0 as f32, dimensions.1 as f32),
                            )
                            .unwrap();

                        std::mem::swap(a, b);
                    }
                }
            }
        }

        self.draw_texture_to_target(frame, a.color_texture);
    }

    fn draw_queues_to<T: Surface>(
        target: &mut T,
        mut draw_queues: DrawQueues,
        cameras: &mut Cameras,
        is_texture_target: bool,
    ) {
        let view_proj = cameras.d3.view_proj();
        draw_queues.draw_queue_3d.draw(target, &view_proj);
        target.clear_depth(1.0);

        let mut projection = cameras.d2.projection_matrix();
        if is_texture_target {
            projection = Mat4::from_scale(Vec3::new(1.0, -1.0, 1.0)) * projection;
        }
        draw_queues.world_draw_queue_2d.draw(target, &projection);
        target.clear_depth(1.0);

        let mut flat_projection = cameras.flat;
        if is_texture_target {
            flat_projection = Mat4::from_scale(Vec3::new(1.0, -1.0, 1.0)) * flat_projection;
        }
        draw_queues.draw_queue_2d.draw(target, &flat_projection);
        target.clear_depth(1.0);
    }

    fn draw_texture_to_target<T: Surface>(&self, target: &mut T, texture: TextureRef) {
        use crate::post_processing::render_fullscreen_quad;

        let uniforms = uniform! {
            tex: texture.get().gl_texture.sampled()
        };

        render_fullscreen_quad(target, &COPY_PROGRAM, &uniforms).unwrap();
    }

    pub fn screen() -> Self {
        Self::new(RenderTarget::Screen, None)
    }

    pub(crate) fn new_draw_queues(&mut self) {
        self.steps.push(RenderStep::Drawing(DrawQueues::empty()));
    }
}

pub fn current_render_pipeline() -> &'static mut RenderPipeline {
    let state = get_render_state();

    match &mut state.texture_pipeline {
        Some(pipeline) => pipeline,
        None => &mut state.render_pipeline,
    }
}

pub fn new_draw_queues() {
    current_render_pipeline().new_draw_queues();
}

pub fn draw_queue_2d() -> &'static mut DrawQueue2D {
    current_render_pipeline().draw_queue_2d()
}

pub fn draw_queue_3d() -> &'static mut DrawQueue3D {
    current_render_pipeline().draw_queue_3d()
}

pub fn world_draw_queue_2d() -> &'static mut DrawQueue2D {
    current_render_pipeline().world_draw_queue_2d()
}

pub(crate) fn empty_render_texture(
    width: u32,
    height: u32,
) -> Result<RenderTexture, TextureCreationError> {
    let window = get_window_state();
    let facade = &window.display;
    let samples = SAMPLES;
    let texture = Texture2d::empty(facade, width, height)?;
    let texture = SgeTexture::new(texture).create();
    Ok(RenderTexture {
        dimensions: UVec2::new(width, height),
        depth_texture: DepthTexture2d::empty(facade, width, height)?,
        color_texture: texture,
        msaa_color: RenderBuffer::new_multisample(
            facade,
            glium::texture::UncompressedFloatFormat::U8U8U8U8,
            width,
            height,
            samples,
        )
        .unwrap(),
        msaa_depth: DepthRenderBuffer::new_multisample(
            facade,
            glium::texture::DepthFormat::I24,
            width,
            height,
            samples,
        )
        .unwrap(),
    })
}

pub fn init() {
    init_render_textures_storage();
    log::info!("Initialized render texture storage");
}
