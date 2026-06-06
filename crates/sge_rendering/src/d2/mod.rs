use glium::vertex::Vertex as GliumVertex;
use glium::{Blend, DrawParameters, IndexBuffer, Surface, VertexBuffer, uniform};
use glium::{Depth, DepthTest, implement_vertex};
use sge_color::Color;
use sge_config::{get_dithering, get_polygon_mode};
use sge_debugging::*;
use sge_math::transform::Transform2D;
use sge_shapes::d2::{QUAD_INDICES, Shape2D, UNIT_QUAD};
use sge_textures::TextureRef;
use sge_types::{
    Area, ColorVertex2D, LineBatch, MeshBatch, MetaballBatch, Pattern, PointBatch, Sdf, SdfBatch,
};
use sge_vectors::{Mat4, Rect, Vec2, Vec3};
use sge_window::get_display;

use crate::scissor::current_scissor;

pub use queue::*;
pub use scene::*;

mod queue;
mod scene;

#[derive(Clone)]
pub enum DrawCommand {
    Sdf(SdfBatch),
    Mesh(MeshBatch),
    Points(PointBatch),
    Lines(LineBatch),
    Sprites(SpriteBatch),
    Metaballs(*const MetaballBatch),
}

#[derive(Clone)]
pub struct SpriteBatch {
    texture: TextureRef,
    vertices: Vec<SpriteVertex>,
    indices: Vec<u32>,
    scissor: Option<glium::Rect>,
}

implement_vertex!(SpriteVertex, position, tex_coords, color);
#[derive(Copy, Clone, Debug)]
pub struct SpriteVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub color: [f32; 4],
}

#[derive(Clone, Copy, PartialEq)]
pub enum RendererType {
    Screen,
    World,
    Scene,
    Other,
}

#[derive(Clone, Copy)]
pub struct Renderer2D {
    draws: *mut Vec<DrawCommand>,
    ty: RendererType,
}

impl Renderer2D {
    fn draws_mut(&mut self) -> &mut Vec<DrawCommand> {
        unsafe { &mut *self.draws }
    }

    fn draws(&self) -> &[DrawCommand] {
        unsafe { &*self.draws }
    }

    pub fn is_world(&self) -> bool {
        matches!(self.ty, RendererType::World)
    }

    pub fn current_sprite_batch(&mut self, texture: TextureRef) -> &mut SpriteBatch {
        let scissor = current_scissor();

        let can_merge = match self.draws().last() {
            Some(DrawCommand::Sprites(b)) => b.texture == texture && b.scissor == scissor,
            _ => false,
        };

        if !can_merge {
            self.draws_mut().push(DrawCommand::Sprites(SpriteBatch {
                texture,
                vertices: Vec::new(),
                indices: Vec::new(),
                scissor,
            }));
        }

        match self.draws_mut().last_mut().unwrap() {
            DrawCommand::Sprites(b) => b,
            _ => unreachable!(),
        }
    }

    pub fn current_sdf_batch(&mut self) -> &mut SdfBatch {
        let scissor = current_scissor();
        let needs_new = match self.draws().last() {
            Some(DrawCommand::Sdf(b)) => b.scissor != scissor,
            _ => true,
        };
        if needs_new {
            self.draws_mut()
                .push(DrawCommand::Sdf(SdfBatch::new(scissor)));
        }
        match self.draws_mut().last_mut().unwrap() {
            DrawCommand::Sdf(b) => b,
            _ => unreachable!(),
        }
    }

    #[allow(unused_mut)]
    pub fn add_sdf(&mut self, mut instance: Sdf) {
        debugger_add_drawn_objects(1);
        #[cfg(feature = "round_coords")]
        if self.ty == RendererType::Screen {
            instance.round_coords();
        }
        self.current_sdf_batch().instances.push(instance);
    }

    pub fn add_shape_sdf(&mut self, shape: &impl Shape2D) {
        self.add_sdf(shape.sdf());
    }

    pub fn add_shape_mesh(&mut self, shape: &impl Shape2D) {
        let batch = self.current_mesh_batch();
        let base_index = batch.max_index;
        let (indices, vertices) = shape.gen_mesh(base_index);
        batch.max_index += vertices.len() as u32;
        for v in vertices {
            batch.vertices.push(v.solid_pattern());
        }
        batch.indices.extend(indices);
    }

    pub fn current_draw_n(&self) -> usize {
        self.draws().len()
    }

    pub fn add_metaball_batch(&mut self, batch: &MetaballBatch) {
        debugger_add_drawn_objects(batch.len());
        self.draws_mut()
            .push(DrawCommand::Metaballs(batch as *const MetaballBatch));
    }

    pub fn add_texture(
        &mut self,
        texture: TextureRef,
        mut transform: Transform2D,
        color: Color,
        region: Option<Area>,
    ) {
        let region = region.map(|a| a.to_rect());
        debugger_add_drawn_objects(1);

        let (tex_min_x, tex_min_y, tex_max_x, tex_max_y) = if let Some(region) = region {
            let tex = texture.get();
            let tex_width = tex.gl_texture.width() as f32;
            let tex_height = tex.gl_texture.height() as f32;
            (
                region.min.x / tex_width,
                region.min.y / tex_height,
                region.max.x / tex_width,
                region.max.y / tex_height,
            )
        } else {
            (0.0, 0.0, 1.0, 1.0)
        };

        let color_gpu = color.for_gpu();
        let mat = transform.matrix();

        let corners = [
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ];

        let tex_coords = [
            [tex_min_x, tex_min_y],
            [tex_max_x, tex_min_y],
            [tex_max_x, tex_max_y],
            [tex_min_x, tex_max_y],
        ];

        let round = cfg!(feature = "round_coords") && self.ty == RendererType::Screen;

        let batch = self.current_sprite_batch(texture);
        let base_index = batch.vertices.len() as u32;

        for i in 0..4 {
            let v = mat.transform_point3(corners[i]);
            let position = if round {
                [v.x.round(), v.y.round(), 0.0]
            } else {
                [v.x, v.y, 0.0]
            };
            batch.vertices.push(SpriteVertex {
                position,
                tex_coords: tex_coords[i],
                color: color_gpu,
            });
        }

        batch.indices.extend_from_slice(&[
            base_index,
            base_index + 1,
            base_index + 2,
            base_index,
            base_index + 2,
            base_index + 3,
        ]);
    }

    pub fn current_mesh_batch(&mut self) -> &mut MeshBatch {
        let scissor = current_scissor();
        let needs_new = match self.draws().last() {
            Some(DrawCommand::Mesh(b)) => b.scissor != scissor,
            _ => true,
        };
        if needs_new {
            self.draws_mut()
                .push(DrawCommand::Mesh(MeshBatch::new(scissor)));
        }
        match self.draws_mut().last_mut().unwrap() {
            DrawCommand::Mesh(b) => b,
            _ => unreachable!(),
        }
    }

    pub fn add_mesh(&mut self, vertices: &[ColorVertex2D], indices: &[u32]) {
        let batch = self.current_mesh_batch();
        let base_index = batch.max_index;
        for v in vertices {
            batch.vertices.push(v.solid_pattern());
        }
        batch.indices.extend(indices.iter().map(|i| i + base_index));
        batch.max_index += vertices.len() as u32;
    }

    pub fn add_mesh_with_pattern(
        &mut self,
        vertices: &[ColorVertex2D],
        indices: &[u32],
        alt_color: Color,
        pattern: Pattern,
        scale: f32,
    ) {
        let batch = self.current_mesh_batch();
        let base_index = batch.max_index;
        for v in vertices {
            batch.vertices.push(v.to_pattern(alt_color, pattern, scale));
        }
        batch.indices.extend(indices.iter().map(|i| i + base_index));
        batch.max_index += vertices.len() as u32;
    }

    pub fn add_scene(&mut self, scene: &Scene2D) {
        self.draws_mut().append(&mut scene.clone().draws)
    }
    pub fn current_point_batch(&mut self) -> &mut PointBatch {
        let scissor = current_scissor();
        let needs_new = match self.draws().last() {
            Some(DrawCommand::Points(b)) => b.scissor != scissor,
            _ => true,
        };
        if needs_new {
            self.draws_mut()
                .push(DrawCommand::Points(PointBatch::new(scissor)));
        }
        match self.draws_mut().last_mut().unwrap() {
            DrawCommand::Points(b) => b,
            _ => unreachable!(),
        }
    }

    pub fn current_line_batch(&mut self) -> &mut LineBatch {
        let scissor = current_scissor();
        let needs_new = match self.draws().last() {
            Some(DrawCommand::Lines(b)) => b.scissor != scissor,
            _ => true,
        };
        if needs_new {
            self.draws_mut()
                .push(DrawCommand::Lines(LineBatch::new(scissor)));
        }
        match self.draws_mut().last_mut().unwrap() {
            DrawCommand::Lines(b) => b,
            _ => unreachable!(),
        }
    }

    pub fn add_pixel(&mut self, pos: Vec2, color: Color) {
        debugger_add_drawn_objects(1);
        let batch = self.current_point_batch();
        batch
            .vertices
            .push(ColorVertex2D::new(pos.x, pos.y, color).solid_pattern());
        batch.max_index += 1;
    }

    pub fn add_pixel_line(&mut self, a: Vec2, b: Vec2, color: Color) {
        debugger_add_drawn_objects(1);
        let batch = self.current_line_batch();
        let base_index = batch.max_index;
        batch
            .vertices
            .push(ColorVertex2D::new(a.x, a.y, color).solid_pattern());
        batch
            .vertices
            .push(ColorVertex2D::new(b.x, b.y, color).solid_pattern());
        batch
            .indices
            .extend_from_slice(&[base_index, base_index + 1]);
        batch.max_index += 2;
    }
}
