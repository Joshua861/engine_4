use std::collections::HashMap;

use crate::api::debugger_add_drawn_objects;
use crate::prelude::Transform2D;
use crate::programs::{CIRCLE_PROGRAM, FLAT_PROGRAM, TEXTURED_PROGRAM};
use crate::shapes_2d::{QUAD_INDICES, Shape2D, UNIT_QUAD};
use crate::textures::TextureRef;
use crate::{Color, get_state};
use bevy_math::{Mat4, Rect, Vec2};
use glium::{Blend, DrawParameters, IndexBuffer, Surface, VertexBuffer, uniform};
use glium::{Depth, DepthTest, implement_vertex};

pub struct DrawQueue2D {
    shape_vertices: Vec<Vertex3D>,
    shape_indices: Vec<u32>,
    current_max_index: u32,

    circle_instances: Vec<CircleInstance>,
    sprite_draws: HashMap<TextureRef, SpriteDrawBatch>,

    current_z: f32,
    start_z: f32,
    z_increment: f32,
}

struct SpriteDrawBatch {
    vertices: Vec<SpriteVertex>,
    indices: Vec<u32>,
}

implement_vertex!(SpriteVertex, position, tex_coords, color);
#[derive(Copy, Clone, Debug)]
struct SpriteVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub color: [f32; 4],
}

implement_vertex!(
    CircleInstance,
    center,
    radius,
    fill_color,
    outline_thickness,
    outline_color
);
#[derive(Copy, Clone, Debug)]
pub struct CircleInstance {
    pub center: [f32; 3],
    pub radius: [f32; 2],
    pub fill_color: [f32; 4],
    pub outline_thickness: f32,
    pub outline_color: [f32; 4],
}

implement_vertex!(Vertex3D, position, color);
#[derive(Copy, Clone, Debug)]
pub struct Vertex3D {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

implement_vertex!(MaterialVertex3D, position, normal, tex_coords);
#[derive(Copy, Clone, Debug)]
pub struct MaterialVertex3D {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex2D {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

implement_vertex!(Vertex2D, position, color);

impl Vertex2D {
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        Self {
            position: [x, y],
            color: color.for_gpu(),
        }
    }

    pub fn to_3d(self, z: f32) -> Vertex3D {
        Vertex3D {
            position: [self.position[0], self.position[1], z],
            color: self.color,
        }
    }
}

impl CircleInstance {
    pub fn new(center: Vec2, z: f32, radius: Vec2, fill_color: Color) -> Self {
        Self {
            center: [center.x, center.y, z],
            radius: radius.into(),
            fill_color: fill_color.for_gpu(),
            outline_thickness: 0.0,
            outline_color: fill_color.for_gpu(),
        }
    }

    pub fn new_with_outline(
        center: Vec2,
        z: f32,
        radius: Vec2,
        fill_color: Color,
        outline_thickness: f32,
        outline_color: Color,
    ) -> Self {
        Self {
            center: [center.x, center.y, z],
            radius: radius.into(),
            fill_color: fill_color.for_gpu(),
            outline_thickness,
            outline_color: outline_color.for_gpu(),
        }
    }
}

impl DrawQueue2D {
    pub fn empty() -> Self {
        Self {
            shape_vertices: vec![],
            shape_indices: vec![],
            current_max_index: 0,
            circle_instances: vec![],
            sprite_draws: HashMap::new(),
            current_z: 0.0,
            start_z: 0.0,
            z_increment: 0.001,
        }
    }

    pub fn with_z_config(start_z: f32, z_increment: f32) -> Self {
        Self {
            shape_vertices: vec![],
            shape_indices: vec![],
            current_max_index: 0,
            circle_instances: vec![],
            sprite_draws: HashMap::new(),
            current_z: 0.0,
            start_z,
            z_increment,
        }
    }

    pub fn current_z(&self) -> f32 {
        self.current_z
    }

    pub fn set_z(&mut self, z: f32) {
        self.current_z = z;
    }

    pub fn next_z(&mut self) -> f32 {
        self.current_z += self.z_increment;
        self.current_z
    }

    pub fn add_shape(&mut self, shape: &impl Shape2D) {
        self.add_shape_at_z(shape, self.current_z);
        self.current_z += self.z_increment;
    }

    pub fn add_shape_at_z(&mut self, shape: &impl Shape2D, z: f32) {
        #[cfg(feature = "debugging")]
        {
            use crate::debugging::get_debug_info_mut;

            let debug = get_debug_info_mut();
            let frame = debug.current_frame_mut();
            frame.drawn_objects += 1;
        }

        let (mut indices, vertices) = shape.points(self.current_max_index);

        for vertex in &vertices {
            self.shape_vertices.push(vertex.to_3d(z));
        }

        self.current_max_index += vertices.len() as u32;
        self.shape_indices.append(&mut indices);
    }

    pub fn add_circle(&mut self, center: Vec2, radius: Vec2, color: Color) {
        self.add_circle_at_z(center, radius, color, self.current_z);
        self.current_z += self.z_increment;
    }

    pub fn add_circle_with_outline(
        &mut self,
        center: Vec2,
        radius: Vec2,
        fill_color: Color,
        outline_thickness: f32,
        outline_color: Color,
    ) {
        self.add_circle_with_outline_at_z(
            center,
            radius,
            fill_color,
            outline_thickness,
            outline_color,
            self.current_z,
        );
        self.current_z += self.z_increment;
    }

    pub fn add_circle_at_z(&mut self, center: Vec2, radius: Vec2, color: Color, z: f32) {
        #[cfg(feature = "debugging")]
        {
            use crate::debugging::get_debug_info_mut;

            let debug = get_debug_info_mut();
            let frame = debug.current_frame_mut();
            frame.drawn_objects += 1;
        }

        self.circle_instances
            .push(CircleInstance::new(center, z, radius, color));
    }

    pub fn add_circle_with_outline_at_z(
        &mut self,
        center: Vec2,
        radius: Vec2,
        fill_color: Color,
        outline_thickness: f32,
        outline_color: Color,
        z: f32,
    ) {
        #[cfg(feature = "debugging")]
        {
            use crate::debugging::get_debug_info_mut;

            let debug = get_debug_info_mut();
            let frame = debug.current_frame_mut();
            frame.drawn_objects += 1;
        }

        self.circle_instances.push(CircleInstance::new_with_outline(
            center,
            z,
            radius,
            fill_color,
            outline_thickness,
            outline_color,
        ));
    }

    pub fn add_sprite(
        &mut self,
        texture: TextureRef,
        transform: Transform2D,
        color: Color,
        region: Option<Rect>,
    ) {
        self.add_sprite_at_z(texture, transform, color, region, self.current_z);
        self.current_z += self.z_increment;
    }

    pub fn add_sprite_at_z(
        &mut self,
        texture: TextureRef,
        mut transform: Transform2D,
        color: Color,
        region: Option<Rect>,
        z: f32,
    ) {
        debugger_add_drawn_objects(1);

        let batch = self
            .sprite_draws
            .entry(texture)
            .or_insert_with(|| SpriteDrawBatch {
                vertices: Vec::new(),
                indices: Vec::new(),
            });

        let base_index = batch.vertices.len() as u32;

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
            bevy_math::Vec3::new(0.0, 0.0, 0.0),
            bevy_math::Vec3::new(1.0, 0.0, 0.0),
            bevy_math::Vec3::new(1.0, 1.0, 0.0),
            bevy_math::Vec3::new(0.0, 1.0, 0.0),
        ];

        let tex_coords = [
            [tex_min_x, tex_min_y],
            [tex_max_x, tex_min_y],
            [tex_max_x, tex_max_y],
            [tex_min_x, tex_max_y],
        ];

        for i in 0..4 {
            let v = mat.transform_point3(corners[i]);
            batch.vertices.push(SpriteVertex {
                position: [v.x, v.y, z],
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

    pub fn draw<T: Surface>(&mut self, frame: &mut T, projection: &Mat4) {
        let state = get_state();
        let display = &state.display;

        let params = DrawParameters {
            blend: Blend {
                color: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::SourceAlpha,
                    destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                },
                alpha: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                },
                constant_value: (1.0, 1.0, 1.0, 1.0),
            },
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        if !self.shape_vertices.is_empty() {
            let vertex_buffer = VertexBuffer::new(display, &self.shape_vertices).unwrap();
            let index_buffer = IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &self.shape_indices,
            )
            .unwrap();

            let uniforms = uniform! {
                transform: projection.to_cols_array_2d(),
            };

            #[cfg(feature = "debugging")]
            {
                use crate::debugging::get_debug_info_mut;

                let debug = get_debug_info_mut();
                let frame = debug.current_frame_mut();
                frame.draw_calls += 1;
                frame.vertex_count += vertex_buffer.len();
                frame.index_count += index_buffer.len();
            }

            frame
                .draw(
                    &vertex_buffer,
                    &index_buffer,
                    FLAT_PROGRAM.get(),
                    &uniforms,
                    &params,
                )
                .unwrap();
        }

        if !self.circle_instances.is_empty() {
            let quad_buffer = VertexBuffer::new(display, &UNIT_QUAD).unwrap();
            let instance_buffer = VertexBuffer::dynamic(display, &self.circle_instances).unwrap();
            let index_buffer = IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &QUAD_INDICES,
            )
            .unwrap();

            let uniforms = uniform! {
                transform: projection.to_cols_array_2d(),
            };

            #[cfg(feature = "debugging")]
            {
                use crate::debugging::get_debug_info_mut;

                let debug = get_debug_info_mut();
                let frame = debug.current_frame_mut();
                frame.draw_calls += 1;
                frame.vertex_count += quad_buffer.len() * self.circle_instances.len();
                frame.index_count += index_buffer.len() * self.circle_instances.len();
            }

            frame
                .draw(
                    (&quad_buffer, instance_buffer.per_instance().unwrap()),
                    &index_buffer,
                    CIRCLE_PROGRAM.get(),
                    &uniforms,
                    &params,
                )
                .unwrap();
        }

        for (texture_ref, batch) in &self.sprite_draws {
            if !batch.vertices.is_empty() {
                self.draw_sprite_batch(frame, projection, *texture_ref, batch);
            }
        }
    }

    fn draw_sprite_batch<T: Surface>(
        &self,
        frame: &mut T,
        projection: &Mat4,
        texture: TextureRef,
        batch: &SpriteDrawBatch,
    ) {
        let state = get_state();
        let display = &state.display;

        let texture = texture.get();
        let vertex_buffer = VertexBuffer::new(display, &batch.vertices).unwrap();
        let index_buffer = IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &batch.indices,
        )
        .unwrap();

        let uniforms = uniform! {
            tex: texture.gl_texture.sampled().minify_filter(texture.minify_filter).magnify_filter(texture.magnify_filter),
            projection: projection.to_cols_array_2d()
        };

        let params = DrawParameters {
            blend: Blend::alpha_blending(),
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        #[cfg(feature = "debugging")]
        {
            use crate::debugging::get_debug_info_mut;
            let debug = get_debug_info_mut();
            let frame_info = debug.current_frame_mut();
            frame_info.draw_calls += 1;
            frame_info.vertex_count += vertex_buffer.len();
            frame_info.index_count += index_buffer.len();
        }

        frame
            .draw(
                &vertex_buffer,
                &index_buffer,
                TEXTURED_PROGRAM.get(),
                &uniforms,
                &params,
            )
            .unwrap();
    }

    pub fn clear(&mut self) {
        self.shape_vertices.clear();
        self.shape_indices.clear();
        self.current_max_index = 0;
        self.circle_instances.clear();
        self.sprite_draws.clear();
        self.current_z = self.start_z;
    }
}
