use glium::{DrawParameters, Surface};
use sge_camera::get_camera_3d;
use sge_config::get_dithering;
use sge_debugging::*;
use sge_math::transform::Transform3D;
use sge_rng::rand;
use sge_time::delta_time;
use sge_time::time;
use sge_vectors::Mat4;
use sge_window::window_size;

use crate::materials::Material;
use crate::object_3d::Object3D;
use crate::object_3d::ObjectToDraw;

pub struct DrawQueue3D {
    pub(crate) objects: Vec<ObjectToDraw>,
}

impl DrawQueue3D {
    pub fn empty() -> Self {
        Self { objects: vec![] }
    }

    pub fn draw<T: Surface>(&mut self, frame: &mut T, view_proj: &Mat4) {
        let params = DrawParameters {
            blend: glium::Blend::alpha_blending(),
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            dithering: get_dithering(),
            ..Default::default()
        };

        let delta_time = delta_time();
        let time = time();
        let random_number: f32 = rand();
        let screen_size = window_size();

        let set_common_uniforms = |material: &mut Material, mut transform: Transform3D| {
            material.set_mat4("view_proj_matrix", *view_proj);
            material.set_mat4("model_matrix", transform.matrix());
            material.set_mat3("normal_matrix", transform.into_normal_matrix());
            material.set_float("time", time);
            material.set_float("delta_time", delta_time);
            material.set_float("random", random_number);
            material.set_vec2("screen_size", screen_size);
            material.set_vec3("camera_pos", get_camera_3d().eye());
        };

        let draw_object = |frame: &mut T, object: &mut Object3D, transform: Transform3D| {
            let material = object.material.get_mut();
            set_common_uniforms(material, transform);
            let program = material.program.get();

            let default_params = DrawParameters {
                backface_culling: object.transform.desired_culling_mode(object.flip_normals),
                ..params.clone()
            };
            let params = object
                .material
                .draw_param_overrides
                .as_ref()
                .unwrap_or(&default_params);

            debugger_add_vertices(object.mesh.vertices.len());
            debugger_add_indices(object.mesh.indices.len());
            debugger_add_drawn_objects(1);
            debugger_add_draw_calls(1);

            frame
                .draw(
                    &object.mesh.vertices,
                    &object.mesh.indices,
                    program,
                    material,
                    params,
                )
                .unwrap();
        };

        for object in std::mem::take(&mut self.objects).iter_mut() {
            match object {
                ObjectToDraw::Many { object, transforms } => {
                    let object = object.get_mut();
                    for transform in transforms.iter_mut() {
                        draw_object(frame, object, *transform);
                    }
                }
                ObjectToDraw::Single(object) => {
                    let object = object.get_mut();
                    let transform = object.transform;
                    draw_object(frame, object, transform);
                }
                ObjectToDraw::WithTransform(object, transform) => {
                    let object = object.get_mut();
                    draw_object(frame, object, *transform);
                }
            }
        }
    }
}
