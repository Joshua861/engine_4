use crate::buffers::Buffers;
use crate::post_processing::PostProcessingEffect;
use crate::shapes::{QUAD_INDICES, Shape, UNIT_QUAD};
use crate::utils::extend_vec2;
use crate::{Color, EngineDisplay, Frame, Programs};
use bevy_math::{Mat4, Quat, Vec2};
use glium::{Blend, DrawParameters, IndexBuffer, Surface, VertexBuffer, uniform};

use crate::{Vertex, shapes::CircleInstance, textures::TextureRef};

pub struct DrawQueue {
    batches: Vec<DrawBatch>,
    current_shape_vertices: Vec<Vertex>,
    current_shape_indices: Vec<u32>,
    current_shape_max_index: u32,
    current_circles: Vec<CircleInstance>,
    current_effects: Vec<PostProcessingEffect>,
}

impl DrawQueue {
    pub fn empty() -> Self {
        Self {
            batches: vec![],
            current_shape_vertices: vec![],
            current_shape_indices: vec![],
            current_shape_max_index: 0,
            current_circles: vec![],
            current_effects: vec![],
        }
    }

    fn flush_shapes(&mut self) {
        if !self.current_shape_vertices.is_empty() {
            self.batches.push(DrawBatch::Shapes {
                vertices: std::mem::take(&mut self.current_shape_vertices),
                indices: std::mem::take(&mut self.current_shape_indices),
            });
            self.current_shape_max_index = 0;
        }
    }

    fn flush_circles(&mut self) {
        if !self.current_circles.is_empty() {
            self.batches.push(DrawBatch::Circles {
                instances: std::mem::take(&mut self.current_circles),
            });
        }
    }

    fn flush_effects(&mut self) {
        if !self.current_effects.is_empty() {
            self.batches.push(DrawBatch::PostProcessing(std::mem::take(
                &mut self.current_effects,
            )));
        }
    }

    pub fn add_shape(&mut self, shape: impl Shape) {
        #[cfg(feature = "debugging")]
        {
            use crate::debugging::get_debug_info_mut;

            let debug = get_debug_info_mut();
            let frame = debug.current_frame_mut();
            frame.drawn_objects += 1;
        }

        self.flush_circles();
        self.flush_effects();

        let (mut indices, mut vertices) = shape.points(self.current_shape_max_index);
        self.current_shape_max_index += vertices.len() as u32;
        self.current_shape_vertices.append(&mut vertices);
        self.current_shape_indices.append(&mut indices);
    }

    pub fn add_circle(&mut self, center: Vec2, radius: f32, color: Color) {
        #[cfg(feature = "debugging")]
        {
            use crate::debugging::get_debug_info_mut;

            let debug = get_debug_info_mut();
            let frame = debug.current_frame_mut();
            frame.drawn_objects += 1;
        }

        self.flush_shapes();
        self.flush_effects();

        self.current_circles
            .push(CircleInstance::new(center, radius, color));
    }

    pub fn add_sprite(&mut self, texture: TextureRef, position: Vec2, size: Vec2) {
        #[cfg(feature = "debugging")]
        {
            use crate::debugging::get_debug_info_mut;

            let debug = get_debug_info_mut();
            let frame = debug.current_frame_mut();
            frame.drawn_objects += 1;
        }

        self.flush_shapes();
        self.flush_circles();
        self.flush_effects();

        self.batches.push(DrawBatch::Sprite {
            texture,
            position,
            size,
        });
    }

    pub fn add_effect(&mut self, effect: PostProcessingEffect) {
        self.flush_shapes();
        self.flush_circles();

        self.current_effects.push(effect);
    }

    pub fn draw(
        &mut self,
        frame: &mut Frame,
        display: &EngineDisplay,
        programs: &Programs,
        projection: &Mat4,
        buffers: &Buffers,
    ) {
        self.flush_shapes();
        self.flush_circles();

        for batch in &self.batches {
            match batch {
                DrawBatch::Shapes { vertices, indices } => {
                    let vertex_buffer = VertexBuffer::new(display, vertices).unwrap();
                    let index_buffer = IndexBuffer::new(
                        display,
                        glium::index::PrimitiveType::TrianglesList,
                        indices,
                    )
                    .unwrap();

                    let uniforms = uniform! {
                        transform: projection.to_cols_array_2d(),
                    };

                    let params = DrawParameters {
                        blend: Blend::alpha_blending(),
                        ..Default::default()
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
                            &programs.flat,
                            &uniforms,
                            &params,
                        )
                        .unwrap();
                }
                DrawBatch::Circles { instances } => {
                    let quad_buffer = VertexBuffer::new(display, &UNIT_QUAD).unwrap();
                    let instance_buffer = VertexBuffer::dynamic(display, instances).unwrap();
                    let index_buffer = IndexBuffer::new(
                        display,
                        glium::index::PrimitiveType::TrianglesList,
                        &QUAD_INDICES,
                    )
                    .unwrap();

                    let uniforms = uniform! {
                        transform: projection.to_cols_array_2d(),
                    };

                    let params = DrawParameters {
                        blend: Blend::alpha_blending(),
                        ..Default::default()
                    };

                    #[cfg(feature = "debugging")]
                    {
                        use crate::debugging::get_debug_info_mut;

                        let debug = get_debug_info_mut();
                        let frame = debug.current_frame_mut();
                        frame.draw_calls += 1;
                        frame.vertex_count += index_buffer.len();
                        frame.index_count += index_buffer.len();
                    }

                    frame
                        .draw(
                            (&quad_buffer, instance_buffer.per_instance().unwrap()),
                            &index_buffer,
                            &programs.circle,
                            &uniforms,
                            &params,
                        )
                        .unwrap();
                }
                DrawBatch::Sprite {
                    texture,
                    position,
                    size,
                } => {
                    let texture = texture.get();

                    #[cfg(feature = "debugging")]
                    {
                        use crate::debugging::get_debug_info_mut;

                        let debug = get_debug_info_mut();
                        let frame = debug.current_frame_mut();
                        frame.draw_calls += 1;
                        frame.vertex_count += buffers.unit_square_tex.len();
                        frame.index_count += buffers.unit_indices_tex.len();
                    }

                    let transform = projection
                        * Mat4::from_scale_rotation_translation(
                            extend_vec2(size),
                            Quat::IDENTITY,
                            extend_vec2(position),
                        );

                    let uniforms = uniform! {
                        tex: &texture.gl_texture,
                        matrix: transform.to_cols_array_2d()
                    };

                    frame
                        .draw(
                            &buffers.unit_square_tex,
                            &buffers.unit_indices_tex,
                            &programs.textured,
                            &uniforms,
                            &Default::default(),
                        )
                        .unwrap();
                }
                DrawBatch::PostProcessing(effects) => {

                }
            }
        }
    }

    pub fn clear(&mut self) {
        *self = Self::empty()
    }
}

enum DrawBatch {
    Shapes {
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    },
    Circles {
        instances: Vec<CircleInstance>,
    },
    Sprite {
        texture: TextureRef,
        position: Vec2,
        size: Vec2,
    },
    PostProcessing(Vec<PostProcessingEffect>),
}
