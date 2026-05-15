use glium::{BlendingFunction, LinearBlendingFactor};
use sge_programs::{FLAT_PROGRAM, METABALL_PROGRAM, SDF_PROGRAM, TEXTURED_PROGRAM};
use sge_types::Vertex2D;

use super::*;

pub struct DrawQueue2D {
    world: bool,
    draws: Vec<DrawCommand>,
}

impl DrawQueue2D {
    pub fn screen() -> Self {
        Self {
            world: false,
            draws: vec![],
        }
    }

    pub fn world() -> Self {
        Self {
            world: true,
            draws: vec![],
        }
    }

    pub fn renderer(&mut self) -> Renderer2D {
        Renderer2D {
            draws: &mut self.draws as *mut Vec<DrawCommand>,
            ty: if self.world {
                RendererType::World
            } else {
                RendererType::Screen
            },
        }
    }
}

impl DrawQueue2D {
    pub fn draw<T: Surface>(&mut self, frame: &mut T, projection: &Mat4) {
        unsafe { MetaballBatch::update_all() };

        for command in &self.draws {
            match command {
                DrawCommand::Sdf(batch) => {
                    if !batch.instances.is_empty() {
                        self.draw_quad_instanced(
                            frame,
                            projection,
                            &batch
                                .instances
                                .iter()
                                .map(|i| i.to_gpu())
                                .collect::<Vec<_>>(),
                            SDF_PROGRAM.get(),
                            batch.scissor,
                        );
                    }
                }
                DrawCommand::Mesh(batch) => {
                    if !batch.vertices.is_empty() && !batch.indices.is_empty() {
                        self.draw_mesh_batch(frame, projection, batch, FLAT_PROGRAM.get());
                    }
                }
                DrawCommand::Points(batch) => {
                    if !batch.vertices.is_empty() {
                        self.draw_point_batch(frame, projection, batch);
                    }
                }
                DrawCommand::Lines(batch) => {
                    if !batch.vertices.is_empty() {
                        self.draw_line_batch(frame, projection, batch);
                    }
                }
                DrawCommand::Sprites(batch) => {
                    if !batch.vertices.is_empty() {
                        self.draw_sprite_batch(frame, projection, batch);
                    }
                }
                DrawCommand::Metaballs(batch_ptr) => {
                    let batch = unsafe { &**batch_ptr };
                    let bb = batch.bounding_box();
                    let min = bb.top_left;
                    let max = bb.bottom_right();
                    let quad = [
                        Vertex2D::new(min.x, min.y),
                        Vertex2D::new(max.x, min.y),
                        Vertex2D::new(max.x, max.y),
                        Vertex2D::new(min.x, max.y),
                    ];

                    let display = get_display();
                    let params = Self::common_draw_params(None, false);
                    let vertex_buffer = VertexBuffer::new(display, &quad).unwrap();
                    let index_buffer = IndexBuffer::new(
                        display,
                        glium::index::PrimitiveType::TrianglesList,
                        &[0u32, 1, 2, 0, 2, 3],
                    )
                    .unwrap();

                    let uniforms = uniform! {
                        transform: projection.to_cols_array_2d(),
                        metaball_data: batch.texture().sampled()
                            .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                        num_metaballs: batch.len() as i32,
                        color: batch.color().for_gpu(),
                    };

                    debugger_add_draw_calls(1);
                    frame
                        .draw(
                            &vertex_buffer,
                            &index_buffer,
                            METABALL_PROGRAM.get(),
                            &uniforms,
                            &params,
                        )
                        .unwrap();
                }
            }
        }
    }

    fn draw_mesh_batch<T: Surface>(
        &self,
        frame: &mut T,
        projection: &Mat4,
        batch: &MeshBatch,
        program: &glium::Program,
    ) {
        let display = get_display();
        let params = Self::common_draw_params(batch.scissor, false);
        let vertex_buffer = VertexBuffer::new(display, &batch.vertices).unwrap();
        let index_buffer = IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &batch.indices,
        )
        .unwrap();

        let uniforms = uniform! {
            transform: projection.to_cols_array_2d(),
        };

        debugger_add_draw_calls(1);
        debugger_add_vertices(vertex_buffer.len());
        debugger_add_indices(index_buffer.len());

        frame
            .draw(&vertex_buffer, &index_buffer, program, &uniforms, &params)
            .unwrap();
    }

    fn common_draw_params(
        scissor: Option<glium::Rect>,
        alpha_blend: bool,
    ) -> DrawParameters<'static> {
        DrawParameters {
            point_size: Some(1.0),
            line_width: Some(1.0),
            blend: if alpha_blend {
                Blend {
                    color: glium::BlendingFunction::Addition {
                        source: glium::LinearBlendingFactor::One,
                        destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                    },
                    alpha: glium::BlendingFunction::Addition {
                        source: glium::LinearBlendingFactor::One,
                        destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                    },
                    constant_value: (1.0, 1.0, 1.0, 1.0),
                }
            } else {
                Blend {
                    color: glium::BlendingFunction::Addition {
                        source: glium::LinearBlendingFactor::SourceAlpha,
                        destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                    },
                    alpha: glium::BlendingFunction::Addition {
                        source: glium::LinearBlendingFactor::One,
                        destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                    },
                    constant_value: (1.0, 1.0, 1.0, 1.0),
                }
            },
            depth: Depth {
                test: DepthTest::Overwrite,
                write: false,
                ..Default::default()
            },
            dithering: get_dithering(),
            polygon_mode: get_polygon_mode(),
            scissor,
            ..Default::default()
        }
    }

    fn draw_point_batch<T: Surface>(&self, frame: &mut T, projection: &Mat4, batch: &PointBatch) {
        let display = get_display();
        let params = Self::common_draw_params(batch.scissor, false);
        let vertex_buffer = VertexBuffer::new(display, &batch.vertices).unwrap();
        let index_buffer = IndexBuffer::new(
            display,
            glium::index::PrimitiveType::Points,
            &(0u32..batch.vertices.len() as u32).collect::<Vec<_>>(),
        )
        .unwrap();

        let uniforms = uniform! {
            transform: projection.to_cols_array_2d(),
        };

        debugger_add_draw_calls(1);
        debugger_add_vertices(vertex_buffer.len());

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

    fn draw_line_batch<T: Surface>(&self, frame: &mut T, projection: &Mat4, batch: &LineBatch) {
        let display = get_display();
        let mut params = Self::common_draw_params(batch.scissor, false);
        params.multisampling = false;
        let vertex_buffer = VertexBuffer::new(display, &batch.vertices).unwrap();
        let index_buffer = IndexBuffer::new(
            display,
            glium::index::PrimitiveType::LinesList,
            &batch.indices,
        )
        .unwrap();

        let uniforms = uniform! {
            transform: projection.to_cols_array_2d(),
        };

        debugger_add_draw_calls(1);
        debugger_add_vertices(vertex_buffer.len());
        debugger_add_indices(index_buffer.len());

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

    fn draw_quad_instanced<T, S>(
        &self,
        frame: &mut S,
        projection: &Mat4,
        instances: &[T],
        program: &glium::Program,
        scissor: Option<glium::Rect>,
    ) where
        T: Copy + GliumVertex,
        S: Surface,
    {
        let display = get_display();
        let params = Self::common_draw_params(scissor, true);
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

        debugger_add_vertices(quad_buffer.len() * instances.len());
        debugger_add_indices(index_buffer.len() * instances.len());

        frame
            .draw(
                (&quad_buffer, instance_buffer.per_instance().unwrap()),
                &index_buffer,
                program,
                &uniforms,
                &params,
            )
            .unwrap();
    }

    fn draw_sprite_batch<T: Surface>(&self, frame: &mut T, projection: &Mat4, batch: &SpriteBatch) {
        let display = get_display();
        let texture = batch.texture.get();
        let vertex_buffer = VertexBuffer::new(display, &batch.vertices).unwrap();
        let index_buffer = IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &batch.indices,
        )
        .unwrap();

        let uniforms = uniform! {
            tex: texture.gl_texture.sampled()
                .minify_filter(texture.minify_filter)
                .magnify_filter(texture.magnify_filter),
            projection: projection.to_cols_array_2d()
        };

        let params = Self::common_draw_params(batch.scissor, false);

        debugger_add_draw_calls(1);
        debugger_add_indices(index_buffer.len());
        debugger_add_vertices(vertex_buffer.len());

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
        self.draws.clear();
    }
}
