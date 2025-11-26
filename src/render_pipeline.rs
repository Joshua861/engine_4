use std::ops::{Deref, DerefMut};

use bevy_math::{Mat4, UVec2, Vec2, Vec3};
use glium::{Frame, Surface, Texture2d, framebuffer::SimpleFrameBuffer, texture::DepthTexture2d};
use log::warn;

use crate::{
    BIG_NUMBER, EngineState, camera::Cameras, color::Color, draw_queue_2d::DrawQueue2D,
    draw_queue_3d::DrawQueue3D, get_state, post_processing::PostProcessingEffect,
    textures::TextureRef,
};

pub struct RenderTexture {
    pub dimensions: UVec2,
    pub color_texture: TextureRef,
    pub depth_texture: DepthTexture2d,
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct RenderTextureRef(pub usize);

impl RenderTextureRef {
    pub(crate) fn get(&self) -> &'static RenderTexture {
        &get_state().storage.render_textures[self.0]
    }

    pub fn get_mut(&self) -> &'static mut RenderTexture {
        &mut get_state().storage.render_textures[self.0]
    }

    pub fn dimensions(&self) -> UVec2 {
        self.get().dimensions
    }
}

impl Deref for RenderTextureRef {
    type Target = RenderTexture;
    fn deref(&self) -> &Self::Target {
        &get_state().storage.render_textures[self.0]
    }
}

impl DerefMut for RenderTextureRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut get_state().storage.render_textures[self.0]
    }
}

impl RenderTexture {
    pub fn create(self) -> RenderTextureRef {
        let state = get_state();
        let id = state.storage.render_textures.len();
        state.storage.render_textures.push(self);
        RenderTextureRef(id)
    }
}

pub enum RenderTarget {
    Screen,
    Texture(RenderTextureRef),
}

pub trait RenderPipelinePass {
    fn render(&mut self, state: &mut EngineState, target: RenderTarget);
}

pub struct RenderPipeline {
    pub steps: Vec<RenderStep>,
    pub output: RenderTarget,
    pub clear_color: Option<Color>,
    pub camera_override: Option<Cameras>,
}

pub enum RenderStep {
    Drawing(DrawQueues),
    PostProcessing(PostProcessingStep),
}

impl RenderStep {
    fn assert_drawing(&mut self) -> &mut DrawQueues {
        match self {
            Self::Drawing(d) => d,
            _ => panic!("called `assert_drawing` on non-drawing render step."),
        }
    }
}

pub struct DrawQueues {
    pub draw_queue_2d: DrawQueue2D,
    pub world_draw_queue_2d: DrawQueue2D,
    pub draw_queue_3d: DrawQueue3D,
}

impl DrawQueues {
    pub fn empty() -> Self {
        let draw_queue_2d = DrawQueue2D::empty();
        let world_draw_queue_2d = DrawQueue2D::with_z_config(-BIG_NUMBER, 0.01);
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
        if matches!(self.most_recent_step(), RenderStep::PostProcessing(_)) {
            self.steps.push(RenderStep::Drawing(DrawQueues::empty()));
        }

        let len = self.steps.len() - 1;
        match &mut self.steps[len] {
            RenderStep::Drawing(draw) => draw,
            RenderStep::PostProcessing(_) => unreachable!(),
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

    pub fn most_recent_step(&self) -> &RenderStep {
        let len = self.steps.len() - 1;
        &self.steps[len]
    }

    pub fn most_recent_step_mut(&mut self) -> &mut RenderStep {
        let len = self.steps.len() - 1;
        &mut self.steps[len]
    }

    pub fn cameras(&self) -> Cameras {
        match self.camera_override {
            Some(c) => c,
            None => get_state().cameras(),
        }
    }

    pub fn new(output: RenderTarget, camera_override: Option<Cameras>) -> Self {
        Self {
            steps: vec![RenderStep::Drawing(DrawQueues::empty())],
            output,
            clear_color: None,
            camera_override,
        }
    }

    pub fn draw(&mut self) {
        let state = get_state();

        match self.output {
            RenderTarget::Screen => {
                self.draw_on(&mut state.frame.take().unwrap_or_else(|| state.display.draw()));
            }
            RenderTarget::Texture(rt) => {
                let rt_mut = rt.get_mut();
                let texture = rt_mut.color_texture.get();
                let mut framebuffer = SimpleFrameBuffer::with_depth_buffer(
                    &state.display,
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

        if let Some(c) = self.clear_color {
            frame.clear_color(c.r, c.g, c.b, c.a);
        } else {
            frame.clear_color(0.0, 0.0, 0.0, 1.0);
        }

        frame.clear_depth(1.0);

        let is_texture_target = matches!(self.output, RenderTarget::Texture(_));

        for step in self.steps.iter_mut() {
            match step {
                RenderStep::PostProcessing(_) => todo!(),
                RenderStep::Drawing(draw_queues) => {
                    let view_proj = cameras.d3.view_proj();
                    draw_queues.draw_queue_3d.draw(frame, &view_proj);

                    let mut projection = cameras.d2.projection_matrix();
                    if is_texture_target {
                        projection = Mat4::from_scale(Vec3::new(1.0, -1.0, 1.0)) * projection;
                    }
                    draw_queues.world_draw_queue_2d.draw(frame, &projection);

                    let mut flat_projection = cameras.flat;
                    if is_texture_target {
                        flat_projection =
                            Mat4::from_scale(Vec3::new(1.0, -1.0, 1.0)) * flat_projection;
                    }
                    draw_queues.draw_queue_2d.draw(frame, &flat_projection);
                }
            }
        }
    }

    pub fn screen() -> Self {
        Self::new(RenderTarget::Screen, None)
    }
}

impl EngineState {
    pub fn draw_queue_2d(&mut self) -> &mut DrawQueue2D {
        self.current_render_pipeline().draw_queue_2d()
    }

    pub fn world_draw_queue_2d(&mut self) -> &mut DrawQueue2D {
        self.current_render_pipeline().world_draw_queue_2d()
    }

    pub fn draw_queue_3d(&mut self) -> &mut DrawQueue3D {
        self.current_render_pipeline().draw_queue_3d()
    }

    pub fn current_render_pipeline(&mut self) -> &mut RenderPipeline {
        match &mut self.texture_pipeline {
            Some(pipeline) => pipeline,
            None => &mut self.render_pipeline,
        }
    }

    pub fn start_rendering_to_texture(&mut self, texture: RenderTextureRef) {
        let size = texture.dimensions;
        self.texture_pipeline = Some(RenderPipeline::new(
            RenderTarget::Texture(texture),
            Some(self.cameras_for_resolution(size.x as u32, size.y as u32)),
        ));
    }

    pub fn end_rendering_to_texture(&mut self) {
        match &mut self.texture_pipeline {
            Some(pipeline) => pipeline.draw(),
            None => warn!(
                "Called `end_rendering_to_texture` without any texture pipeline loaded. Create one with `start_rendering_to_texture`."
            ),
        }

        self.texture_pipeline = None;
    }
}
