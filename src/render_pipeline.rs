use std::ops::{Deref, DerefMut};

use bevy_math::{UVec2, Vec2};
use glium::{Frame, Surface, Texture2d};

use crate::{
    BIG_NUMBER, EngineState, color::Color, draw_queue_2d::DrawQueue2D, draw_queue_3d::DrawQueue3D,
    get_state, post_processing::PostProcessingEffect, textures::TextureRef,
};

pub struct RenderTexture {
    pub dimensions: UVec2,
    pub gl_texture: Texture2d,
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

    pub fn new(output: RenderTarget) -> Self {
        Self {
            steps: vec![RenderStep::Drawing(DrawQueues::empty())],
            output,
            clear_color: None,
        }
    }

    pub fn draw(&mut self) {
        let state = get_state();

        match self.output {
            RenderTarget::Screen => {
                self.draw_on(&mut state.frame.take().unwrap_or_else(|| state.display.draw()));
            }
            RenderTarget::Texture(rt) => self.draw_on(&mut rt.get_mut().gl_texture.as_surface()),
        }
    }

    pub fn draw_on<T: Surface>(&mut self, frame: &mut T) {
        let state = get_state();

        if let Some(c) = self.clear_color {
            frame.clear_color(c.r, c.g, c.b, c.a);
        }

        frame.clear_depth(1.0);

        for step in self.steps.iter_mut() {
            match step {
                RenderStep::PostProcessing(_) => todo!(),
                RenderStep::Drawing(draw_queues) => {
                    let view_proj = state.camera_3d.view_proj();
                    draw_queues.draw_queue_3d.draw(frame, &view_proj);

                    let projection = state.camera_2d.projection_matrix();
                    draw_queues.world_draw_queue_2d.draw(frame, &projection);

                    let projection = state.flat_projection;
                    draw_queues.draw_queue_2d.draw(frame, &projection);
                }
            }
        }
    }
}

impl EngineState {
    pub fn draw_queue_2d(&mut self) -> &mut DrawQueue2D {
        self.render_pipeline.draw_queue_2d()
    }

    pub fn world_draw_queue_2d(&mut self) -> &mut DrawQueue2D {
        self.render_pipeline.world_draw_queue_2d()
    }

    pub fn draw_queue_3d(&mut self) -> &mut DrawQueue3D {
        self.render_pipeline.draw_queue_3d()
    }
}
