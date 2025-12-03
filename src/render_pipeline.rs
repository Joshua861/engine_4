use bevy_math::{Mat4, UVec2, Vec2, Vec3};
use engine_4_macros::gen_ref_type;
use glium::{Surface, framebuffer::SimpleFrameBuffer, texture::DepthTexture2d, uniform};
use log::warn;

use crate::{
    EngineState, api::empty_render_texture, camera::Cameras, color::Color,
    draw_queue_2d::DrawQueue2D, draw_queue_3d::DrawQueue3D, get_state,
    post_processing::PostProcessingEffect, programs::ProgramRef, textures::TextureRef,
};

pub struct RenderTexture {
    pub dimensions: UVec2,
    pub color_texture: TextureRef,
    pub depth_texture: DepthTexture2d,
}

impl RenderTexture {
    pub fn framebuffer(&mut self) -> SimpleFrameBuffer<'_> {
        let state = get_state();
        let texture = self.color_texture.get();
        SimpleFrameBuffer::with_depth_buffer(
            &state.display,
            &texture.gl_texture,
            &self.depth_texture,
        )
        .unwrap()
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

pub enum RenderTarget {
    Screen,
    Texture(RenderTextureRef),
}

pub struct RenderPipeline {
    pub steps: Vec<RenderStep>,
    pub output: RenderTarget,
    pub clear_color: Option<Color>,
    pub camera_override: Option<Cameras>,
}

// i dont care
#[allow(clippy::large_enum_variant)]
pub enum RenderStep {
    Drawing(DrawQueues),
    PostProcessing(PostProcessingStep),
}

pub struct DrawQueues {
    pub draw_queue_2d: DrawQueue2D,
    pub world_draw_queue_2d: DrawQueue2D,
    pub draw_queue_3d: DrawQueue3D,
}

impl DrawQueues {
    pub fn empty() -> Self {
        let draw_queue_2d = DrawQueue2D::empty();
        let world_draw_queue_2d = DrawQueue2D::empty();
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

    pub fn post_processing_effects(&mut self) -> &mut PostProcessingStep {
        if matches!(self.most_recent_step(), RenderStep::Drawing(_)) {
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

    pub fn most_recent_step(&self) -> &RenderStep {
        let len = self.steps.len() - 1;
        &self.steps[len]
    }

    pub fn cameras(&self) -> Cameras {
        match self.camera_override {
            Some(c) => c,
            None => get_state().cameras(),
        }
    }

    pub fn add_effect(&mut self, effect: PostProcessingEffect) {
        self.post_processing_effects().0.push(effect);
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
        let state = get_state();
        let mut cameras = self.cameras();
        let is_texture_target = matches!(self.output, RenderTarget::Texture(_));

        let has_post_processing = self
            .steps
            .iter()
            .any(|step| matches!(step, RenderStep::PostProcessing(_)));

        if has_post_processing {
            let dimensions = frame.get_dimensions();

            let mut a = empty_render_texture(dimensions.0, dimensions.1).unwrap();
            let mut b = empty_render_texture(dimensions.0, dimensions.1).unwrap();

            if let Some(c) = self.clear_color {
                a.framebuffer().clear_color(c.r, c.g, c.b, c.a);
                b.framebuffer().clear_color(c.r, c.g, c.b, c.a);
            } else {
                a.framebuffer().clear_color(0.0, 0.0, 0.0, 1.0);
                b.framebuffer().clear_color(0.0, 0.0, 0.0, 1.0);
            }
            a.framebuffer().clear_depth(1.0);
            b.framebuffer().clear_depth(1.0);

            for step in std::mem::take(&mut self.steps) {
                match step {
                    RenderStep::Drawing(draw_queues) => {
                        self.draw_queues_to(
                            &mut a.framebuffer(),
                            draw_queues,
                            &mut cameras,
                            is_texture_target,
                        );
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

                            std::mem::swap(&mut a, &mut b);
                        }
                    }
                }
            }

            self.draw_texture_to_target(frame, a.color_texture);

            state.storage.textures.pop();
            state.storage.textures.pop();
        } else {
            if let Some(c) = self.clear_color {
                frame.clear_color(c.r, c.g, c.b, c.a);
            } else {
                frame.clear_color(0.0, 0.0, 0.0, 1.0);
            }
            frame.clear_depth(1.0);

            for step in std::mem::take(&mut self.steps) {
                match step {
                    RenderStep::Drawing(draw_queues) => {
                        self.draw_queues_to(frame, draw_queues, &mut cameras, is_texture_target);
                    }
                    RenderStep::PostProcessing(_) => {
                        unreachable!();
                    }
                }
            }
        }
    }

    fn draw_queues_to<T: Surface>(
        &self,
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
        use crate::programs::load_program;

        static COPY_PROGRAM: std::sync::OnceLock<ProgramRef> = std::sync::OnceLock::new();
        let program = COPY_PROGRAM.get_or_init(|| {
            let vertex_shader = include_str!("../assets/shaders/copy/vertex.glsl");
            let fragment_shader = include_str!("../assets/shaders/copy/fragment.glsl");
            load_program(vertex_shader, fragment_shader).unwrap()
        });

        let uniforms = uniform! {
            tex: texture.get().gl_texture.sampled()
        };

        render_fullscreen_quad(target, program.get(), &uniforms).unwrap();
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
            Some(self.cameras_for_resolution(size.x, size.y)),
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
