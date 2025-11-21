use bevy_math::Mat4;
use glium::{DrawParameters, Surface};
use rand::Rng;

use crate::object_3d::Object3DRef;
use crate::prelude::Transform3D;
use crate::{Frame, get_state};

pub struct DrawQueue3D {
    pub(crate) objects: Vec<ObjectToDraw>,
}

pub enum ObjectToDraw {
    Single(Object3DRef),
    Many {
        object: Object3DRef,
        transforms: Vec<Transform3D>,
    },
    WithTransform(Object3DRef, Transform3D),
}

impl DrawQueue3D {
    pub fn empty() -> Self {
        Self { objects: vec![] }
    }

    // FIXME: reduce repetition if possible
    pub fn draw(&mut self, frame: &mut Frame, view_proj: &Mat4) {
        let state = get_state();
        let display = &state.display;

        let params = DrawParameters {
            blend: glium::Blend::alpha_blending(),
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let delta_time = state.delta_time;
        let time = state.time;
        let random_number: f32 = state.rng.random();
        let screen_size = state.window_size();

        for object in std::mem::take(&mut self.objects).iter_mut() {
            match object {
                ObjectToDraw::Many { object, transforms } => {
                    let object = object.get_mut();
                    let material = object.material.get_mut();

                    material.set_mat4("view_proj_matrix", *view_proj);
                    material.set_float("time", time);
                    material.set_float("delta_time", delta_time);
                    material.set_float("random", random_number);
                    material.set_vec2("screen_size", screen_size);
                    for transform in transforms.iter_mut() {
                        material.set_mat4("model_matrix", transform.matrix());
                        let program = material.program.get();

                        // FIXME: can we reduce repetition with a macro or function something
                        #[cfg(feature = "debugging")]
                        {
                            use crate::debugging::get_debug_info_mut;
                            let debug = get_debug_info_mut();
                            let frame_info = debug.current_frame_mut();
                            frame_info.vertex_count += object.vertices.len();
                            frame_info.index_count += object.indices.len();
                            frame_info.draw_calls += 1;
                            frame_info.drawn_objects += 1;
                        }

                        frame
                            .draw(
                                &object.vertices,
                                &object.indices,
                                program,
                                material,
                                &params,
                            )
                            .unwrap();
                    }
                }
                ObjectToDraw::Single(object) => {
                    let object = object.get_mut();
                    let transform = object.transform.matrix();
                    let material = object.material.get_mut();

                    material.set_mat4("view_proj_matrix", *view_proj);
                    material.set_mat4("model_matrix", transform);
                    material.set_float("time", time);
                    material.set_float("delta_time", delta_time);
                    material.set_float("random", random_number);
                    material.set_vec2("screen_size", screen_size);
                    let program = material.program.get();

                    #[cfg(feature = "debugging")]
                    {
                        use crate::debugging::get_debug_info_mut;
                        let debug = get_debug_info_mut();
                        let frame_info = debug.current_frame_mut();
                        frame_info.vertex_count += object.vertices.len();
                        frame_info.index_count += object.indices.len();
                        frame_info.draw_calls += 1;
                        frame_info.drawn_objects += 1;
                    }

                    frame
                        .draw(
                            &object.vertices,
                            &object.indices,
                            program,
                            material,
                            &params,
                        )
                        .unwrap();
                }
                ObjectToDraw::WithTransform(object, transform) => {
                    let object = object.get_mut();
                    let transform = object.transform.matrix();
                    let material = object.material.get_mut();

                    material.set_mat4("view_proj_matrix", *view_proj);
                    material.set_mat4("model_matrix", transform);
                    material.set_float("time", time);
                    material.set_vec2("screen_size", screen_size);
                    material.set_float("delta_time", delta_time);
                    material.set_float("random", random_number);
                    let program = material.program.get();

                    #[cfg(feature = "debugging")]
                    {
                        use crate::debugging::get_debug_info_mut;
                        let debug = get_debug_info_mut();
                        let frame_info = debug.current_frame_mut();
                        frame_info.vertex_count += object.vertices.len();
                        frame_info.index_count += object.indices.len();
                        frame_info.draw_calls += 1;
                        frame_info.drawn_objects += 1;
                    }

                    frame
                        .draw(
                            &object.vertices,
                            &object.indices,
                            program,
                            material,
                            &params,
                        )
                        .unwrap();
                }
            }
        }
    }
}
