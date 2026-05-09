use glium::vertex::Vertex as GliumVertex;
use glium::{Blend, DrawParameters, IndexBuffer, Surface, VertexBuffer, uniform};
use glium::{Depth, DepthTest, implement_vertex};
use sge_color::Color;
use sge_config::{get_dithering, get_polygon_mode};
use sge_debugging::*;
use sge_math::transform::Transform2D;
use sge_programs::{
    CIRCLE_PROGRAM, FLAT_PROGRAM, QBEZIER_PROGRAM, RADIAL_PROGRAM, ROUNDED_PROGRAM,
    TEXTURED_PROGRAM,
};
use sge_shapes::d2::{QUAD_INDICES, Shape2D, UNIT_QUAD};
use sge_textures::TextureRef;
use sge_types::{
    CircleBatch, CircleInstance, ColorVertex2D, CubicBezier, CubicBezierBatch, LineBatch,
    MetaballBatch, Pattern, PointBatch, QuadraticBezier, QuadraticBezierBatch, RadialGradientBatch,
    RadialGradientInstance, RoundedBatch, RoundedInstance, ShapeBatch,
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
    Shapes(ShapeBatch),
    Points(PointBatch),
    Lines(LineBatch),
    Circles(CircleBatch),
    Rounded(RoundedBatch),
    Radial(RadialGradientBatch),
    Sprites(SpriteBatch),
    QuadraticBezier(QuadraticBezierBatch),
    CubicBezier(CubicBezierBatch),
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

#[derive(Clone, Copy)]
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

    pub fn current_shape_batch(&mut self) -> &mut ShapeBatch {
        let scissor = current_scissor();
        let needs_new = match self.draws().last() {
            Some(DrawCommand::Shapes(b)) => b.scissor != scissor,
            _ => true,
        };
        if needs_new {
            self.draws_mut()
                .push(DrawCommand::Shapes(ShapeBatch::new(scissor)));
        }
        match self.draws_mut().last_mut().unwrap() {
            DrawCommand::Shapes(b) => b,
            _ => unreachable!(),
        }
    }

    pub fn current_circle_batch(&mut self) -> &mut CircleBatch {
        let scissor = current_scissor();
        let needs_new = match self.draws().last() {
            Some(DrawCommand::Circles(b)) => b.scissor != scissor,
            _ => true,
        };
        if needs_new {
            self.draws_mut()
                .push(DrawCommand::Circles(CircleBatch::new(scissor)));
        }
        match self.draws_mut().last_mut().unwrap() {
            DrawCommand::Circles(b) => b,
            _ => unreachable!(),
        }
    }

    pub fn current_rounded_batch(&mut self) -> &mut RoundedBatch {
        let scissor = current_scissor();
        let needs_new = match self.draws().last() {
            Some(DrawCommand::Rounded(b)) => b.scissor != scissor,
            _ => true,
        };
        if needs_new {
            self.draws_mut()
                .push(DrawCommand::Rounded(RoundedBatch::new(scissor)));
        }
        match self.draws_mut().last_mut().unwrap() {
            DrawCommand::Rounded(b) => b,
            _ => unreachable!(),
        }
    }

    pub fn current_radial_batch(&mut self) -> &mut RadialGradientBatch {
        let scissor = current_scissor();
        let needs_new = match self.draws().last() {
            Some(DrawCommand::Radial(b)) => b.scissor != scissor,
            _ => true,
        };
        if needs_new {
            self.draws_mut()
                .push(DrawCommand::Radial(RadialGradientBatch::new(scissor)));
        }
        match self.draws_mut().last_mut().unwrap() {
            DrawCommand::Radial(b) => b,
            _ => unreachable!(),
        }
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

    pub fn add_shape(&mut self, shape: &impl Shape2D) {
        debugger_add_drawn_objects(1);
        let batch = self.current_shape_batch();
        let (mut indices, vertices) = shape.gen_mesh(batch.max_index);
        for vertex in &vertices {
            batch.vertices.push(vertex.solid_pattern());
        }
        batch.max_index += vertices.len() as u32;
        batch.indices.append(&mut indices);
    }

    pub fn current_draw_n(&self) -> usize {
        self.draws().len()
    }

    pub fn add_circle(&mut self, center: Vec2, radius: Vec2, color: Color) {
        debugger_add_drawn_objects(1);
        self.current_circle_batch()
            .instances
            .push(CircleInstance::new(center, 0.0, radius, color));
    }

    pub fn add_circle_with_outline(
        &mut self,
        center: Vec2,
        radius: Vec2,
        fill_color: Color,
        outline_thickness: f32,
        outline_color: Color,
    ) {
        debugger_add_drawn_objects(1);
        self.current_circle_batch()
            .instances
            .push(CircleInstance::new_with_outline(
                center,
                0.0,
                radius,
                fill_color,
                outline_thickness,
                outline_color,
            ));
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_radial_gradient(
        &mut self,
        center: Vec2,
        radius: Vec2,
        inner_color: Color,
        outer_color: Color,
        outline_thickness: f32,
        outline_color: Color,
        gradient_offset: Vec2,
    ) {
        debugger_add_drawn_objects(1);
        self.current_radial_batch()
            .instances
            .push(RadialGradientInstance {
                center: [center.x, center.y, 0.0],
                radius: [radius.x, radius.y],
                outline_thickness,
                inner_color: inner_color.for_gpu(),
                outer_color: outer_color.for_gpu(),
                outline_color: outline_color.for_gpu(),
                gradient_offset: [gradient_offset.x, gradient_offset.y],
            });
    }

    pub fn add_rounded_rectangle(
        &mut self,
        center: Vec2,
        dimensions: Vec2,
        corner_radius: f32,
        fill_color: Color,
        outline_thickness: f32,
        outline_color: Color,
    ) {
        debugger_add_drawn_objects(1);
        let shortest_side = dimensions.x.min(dimensions.y);
        self.current_rounded_batch()
            .instances
            .push(RoundedInstance::new(
                dimensions,
                center,
                0.0,
                corner_radius.clamp(0.0, (0.5 * shortest_side).max(0.0)),
                fill_color,
                outline_thickness,
                outline_color,
            ));
    }

    pub fn current_quadratic_bezier_batch(&mut self) -> &mut QuadraticBezierBatch {
        let scissor = current_scissor();
        let needs_new = match self.draws().last() {
            Some(DrawCommand::QuadraticBezier(b)) => b.scissor != scissor,
            _ => true,
        };
        if needs_new {
            self.draws_mut()
                .push(DrawCommand::QuadraticBezier(QuadraticBezierBatch::new(
                    scissor,
                )));
        }
        match self.draws_mut().last_mut().unwrap() {
            DrawCommand::QuadraticBezier(b) => b,
            _ => unreachable!(),
        }
    }

    pub fn add_quadratic_bezier(
        &mut self,
        a: Vec2,
        b: Vec2,
        c: Vec2,
        color: Color,
        thickness: f32,
    ) {
        debugger_add_drawn_objects(1);
        self.current_quadratic_bezier_batch()
            .instances
            .push(QuadraticBezier::new(a, b, c, color, thickness));
    }

    pub fn current_cubic_bezier_batch(&mut self) -> &mut CubicBezierBatch {
        let scissor = current_scissor();
        let needs_new = match self.draws().last() {
            Some(DrawCommand::CubicBezier(b)) => b.scissor != scissor,
            _ => true,
        };
        if needs_new {
            self.draws_mut()
                .push(DrawCommand::CubicBezier(CubicBezierBatch::new(scissor)));
        }
        match self.draws_mut().last_mut().unwrap() {
            DrawCommand::CubicBezier(b) => b,
            _ => unreachable!(),
        }
    }

    pub fn add_cubic_bezier(
        &mut self,
        a: Vec2,
        b: Vec2,
        c: Vec2,
        d: Vec2,
        color: Color,
        thickness: f32,
    ) {
        debugger_add_drawn_objects(1);
        self.current_cubic_bezier_batch()
            .instances
            .push(CubicBezier::new(a, b, c, d, color, thickness));
    }

    pub fn add_metaball_batch(&mut self, batch: &MetaballBatch) {
        debugger_add_drawn_objects(batch.len());
        self.draws_mut()
            .push(DrawCommand::Metaballs(batch as *const MetaballBatch));
    }

    pub fn add_sprite(
        &mut self,
        texture: TextureRef,
        mut transform: Transform2D,
        color: Color,
        region: Option<Rect>,
    ) {
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

        let batch = self.current_sprite_batch(texture);
        let base_index = batch.vertices.len() as u32;

        for i in 0..4 {
            let v = mat.transform_point3(corners[i]);
            batch.vertices.push(SpriteVertex {
                #[cfg(not(feature = "round_coords"))]
                position: [v.x, v.y, 0.0],
                #[cfg(feature = "round_coords")]
                position: [v.x.round(), v.y.round(), 0.0],
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

    pub fn add_mesh(&mut self, vertices: &[ColorVertex2D], indices: &[u32]) {
        let batch = self.current_shape_batch();
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
        let batch = self.current_shape_batch();
        let base_index = batch.max_index;
        for v in vertices {
            batch.vertices.push(v.to_pattern(alt_color, pattern, scale));
        }
        batch.indices.extend(indices.iter().map(|i| i + base_index));
        batch.max_index += vertices.len() as u32;
    }

    pub fn add_shape_with_pattern(
        &mut self,
        shape: &impl Shape2D,
        alt_color: Color,
        pattern: Pattern,
        scale: f32,
    ) {
        debugger_add_drawn_objects(1);
        let batch = self.current_shape_batch();
        let (mut indices, vertices) = shape.gen_mesh(batch.max_index);
        for vertex in &vertices {
            batch
                .vertices
                .push(vertex.to_pattern(alt_color, pattern, scale));
        }
        batch.max_index += vertices.len() as u32;
        batch.indices.append(&mut indices);
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

    pub fn add_pixel(&mut self, shape: &impl Shape2D) {
        debugger_add_drawn_objects(1);
        let batch = self.current_point_batch();
        let (_indices, vertices) = shape.gen_mesh(batch.max_index);
        for vertex in &vertices {
            batch.vertices.push(vertex.solid_pattern());
        }
        batch.max_index += vertices.len() as u32;
    }

    pub fn add_pixel_line(&mut self, shape: &impl Shape2D) {
        debugger_add_drawn_objects(1);
        let batch = self.current_line_batch();
        let (mut indices, vertices) = shape.gen_mesh(batch.max_index);
        for vertex in &vertices {
            batch.vertices.push(vertex.solid_pattern());
        }
        batch.max_index += vertices.len() as u32;
        batch.indices.append(&mut indices);
    }
}
