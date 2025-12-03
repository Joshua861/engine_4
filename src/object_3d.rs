use std::io::Cursor;

use crate::utils::EngineCreate;
use engine_4_macros::gen_ref_type;
use glium::{IndexBuffer, VertexBuffer};
use obj::{FromRawVertex, load_obj, raw::object::Polygon};

use crate::{
    color::Color,
    draw_queue_2d::MaterialVertex3D,
    draw_queue_3d::ObjectToDraw,
    get_state,
    materials::{DEFAULT_MATERIAL, MaterialRef},
    prelude::{Material, Transform3D, create_flat_3d_material},
};

pub struct Object3D {
    pub mesh: MeshRef,
    pub material: MaterialRef,
    pub transform: Transform3D,
}

impl Object3D {
    pub fn from_obj(src: &str) -> anyhow::Result<Object3DRef> {
        Self::from_obj_with_material(src, DEFAULT_MATERIAL)
    }

    pub fn from_obj_bytes(data: &[u8]) -> anyhow::Result<Object3DRef> {
        Self::from_obj_bytes_with_material(data, DEFAULT_MATERIAL)
    }

    pub fn from_obj_with_material(src: &str, material: MaterialRef) -> anyhow::Result<Object3DRef> {
        Self::from_obj_bytes_with_material(src.as_bytes(), material)
    }

    pub fn from_obj_bytes_with_material(
        data: &[u8],
        material: MaterialRef,
    ) -> anyhow::Result<Object3DRef> {
        let state = get_state();
        let buf = Cursor::new(data);
        let obj = load_obj::<MaterialVertex3D, _, _>(buf)?;

        let vertices = obj.vertices;
        let indices = obj.indices;

        // dbg!(&vertices);
        // dbg!(&indices);

        let vertices = VertexBuffer::new(&state.display, &vertices)?;
        let indices = IndexBuffer::new(
            &state.display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        )?;

        let object = Self {
            mesh: Mesh { vertices, indices }.create(),
            material,
            transform: Transform3D::IDENTITY,
        };

        Ok(object.create())
    }

    pub fn compute_smooth_normals(&mut self) {
        use bevy_math::Vec3;
        use std::collections::HashMap;

        let vertices: Vec<MaterialVertex3D> = self.mesh.vertices.read().unwrap();

        let mut position_to_vertices: HashMap<[i32; 3], Vec<usize>> = HashMap::new();

        for (i, vertex) in vertices.iter().enumerate() {
            let key = [
                (vertex.position[0] * 1000.0) as i32,
                (vertex.position[1] * 1000.0) as i32,
                (vertex.position[2] * 1000.0) as i32,
            ];
            position_to_vertices.entry(key).or_default().push(i);
        }

        let mut new_normals = vec![[0.0f32; 3]; vertices.len()];

        for vertex_indices in position_to_vertices.values() {
            let mut avg_normal = Vec3::ZERO;
            for &idx in vertex_indices {
                let n = vertices[idx].normal;
                avg_normal += Vec3::new(n[0], n[1], n[2]);
            }
            avg_normal = avg_normal.normalize_or_zero();

            for &idx in vertex_indices {
                new_normals[idx] = [avg_normal.x, avg_normal.y, avg_normal.z];
            }
        }

        let new_vertices: Vec<MaterialVertex3D> = vertices
            .iter()
            .enumerate()
            .map(|(i, v)| MaterialVertex3D {
                position: v.position,
                normal: new_normals[i],
                tex_coords: v.tex_coords,
            })
            .collect();

        let state = get_state();
        self.mesh.vertices = VertexBuffer::new(&state.display, &new_vertices).unwrap();
    }

    pub fn from_mesh_and_material(mesh: MeshRef, material: MaterialRef) -> Object3DRef {
        Self {
            mesh,
            material,
            transform: Transform3D::IDENTITY,
        }
        .create()
    }
}

impl FromRawVertex<u32> for MaterialVertex3D {
    fn process(
        vertices: Vec<(f32, f32, f32, f32)>,
        normals: Vec<(f32, f32, f32)>,
        tex_coords: Vec<(f32, f32, f32)>,
        polygons: Vec<obj::raw::object::Polygon>,
    ) -> obj::ObjResult<(Vec<Self>, Vec<u32>)> {
        use std::collections::HashMap;

        let mut output_vertices = Vec::new();
        let mut indices = Vec::new();
        let mut vertex_map: HashMap<(usize, Option<usize>, Option<usize>), u32> = HashMap::new();

        let tex_coords: Vec<(f32, f32)> = tex_coords
            .into_iter()
            .map(|(u, v, _)| (u, 1.0 - v))
            .collect();

        for polygon in polygons {
            match polygon {
                Polygon::P(v) => {
                    if v.len() < 3 {
                        continue;
                    }

                    for i in 1..v.len() - 1 {
                        for &idx in &[v[0], v[i], v[i + 1]] {
                            let key = (idx, None, None);
                            let index = *vertex_map.entry(key).or_insert_with(|| {
                                let idx = output_vertices.len() as u32;
                                let pos = vertices[idx as usize];
                                output_vertices.push(MaterialVertex3D {
                                    position: [pos.0, pos.1, pos.2],
                                    normal: [0.0, 0.0, 0.0],
                                    tex_coords: [0.0, 0.0],
                                });
                                idx
                            });
                            indices.push(index);
                        }
                    }
                }
                Polygon::PT(v) => {
                    if v.len() < 3 {
                        continue;
                    }

                    for i in 1..v.len() - 1 {
                        for &(pos_idx, tex_idx) in &[v[0], v[i], v[i + 1]] {
                            let key = (pos_idx, Some(tex_idx), None);
                            let index = *vertex_map.entry(key).or_insert_with(|| {
                                let idx = output_vertices.len() as u32;
                                let pos = vertices[pos_idx];
                                let tex = tex_coords[tex_idx];
                                output_vertices.push(MaterialVertex3D {
                                    position: [pos.0, pos.1, pos.2],
                                    normal: [0.0, 0.0, 0.0],
                                    tex_coords: [tex.0, tex.1],
                                });
                                idx
                            });
                            indices.push(index);
                        }
                    }
                }
                Polygon::PTN(v) => {
                    if v.len() < 3 {
                        continue;
                    }

                    for i in 1..v.len() - 1 {
                        for &(pos_idx, tex_idx, norm_idx) in &[v[0], v[i], v[i + 1]] {
                            let key = (pos_idx, Some(tex_idx), Some(norm_idx));
                            let index = *vertex_map.entry(key).or_insert_with(|| {
                                let idx = output_vertices.len() as u32;
                                let pos = vertices[pos_idx];
                                let tex = tex_coords[tex_idx];
                                let normal = normals[norm_idx];
                                output_vertices.push(MaterialVertex3D {
                                    position: [pos.0, pos.1, pos.2],
                                    normal: [normal.0, normal.1, normal.2],
                                    tex_coords: [tex.0, tex.1],
                                });
                                idx
                            });
                            indices.push(index);
                        }
                    }
                }
                Polygon::PN(v) => {
                    if v.len() < 3 {
                        continue;
                    }

                    for i in 1..v.len() - 1 {
                        for &(pos_idx, norm_idx) in &[v[0], v[i], v[i + 1]] {
                            let key = (pos_idx, None, Some(norm_idx));
                            let index = *vertex_map.entry(key).or_insert_with(|| {
                                let idx = output_vertices.len() as u32;
                                let pos = vertices[pos_idx];
                                let normal = normals[norm_idx];
                                output_vertices.push(MaterialVertex3D {
                                    position: [pos.0, pos.1, pos.2],
                                    normal: [normal.0, normal.1, normal.2],
                                    tex_coords: [0.0, 0.0],
                                });
                                idx
                            });
                            indices.push(index);
                        }
                    }
                }
            }
        }

        Ok((output_vertices, indices))
    }
}

gen_ref_type!(Object3D, Object3DRef, objects);

impl Object3DRef {
    pub fn draw(&self) {
        get_state()
            .draw_queue_3d()
            .objects
            .push(ObjectToDraw::Single(*self));
    }

    pub fn draw_many(&self, transforms: Vec<Transform3D>) {
        get_state()
            .draw_queue_3d()
            .objects
            .push(ObjectToDraw::Many {
                object: *self,
                transforms,
            });
    }

    pub fn draw_with_transform(&self, transform: Transform3D) {
        get_state()
            .draw_queue_3d()
            .objects
            .push(ObjectToDraw::WithTransform(*self, transform));
    }

    pub fn with_transform(self, transform: Transform3D) -> Object3DRef {
        let object = self.get_mut();
        object.transform = transform;
        self
    }

    pub fn transform(&self) -> &mut Transform3D {
        &mut self.get_mut().transform
    }

    // pub fn vertices(&self) -> &mut Vec<MaterialVertex3D> {
    //     &mut self.get_mut().vertices
    // }

    // pub fn indices(&self) -> &mut Vec<u32> {
    //     &mut self.get_mut().indices
    // }

    pub fn material(&self) -> &mut Material {
        self.get_mut().material.get_mut()
    }
}

pub fn test_triangle() -> anyhow::Result<Object3DRef> {
    let vertices = vec![
        MaterialVertex3D {
            position: [0.0, 1.0, 0.0],
            normal: [0.0, 0.0, 1.0],
            tex_coords: [0.5, 0.0],
        },
        MaterialVertex3D {
            position: [-1.0, -1.0, 0.0],
            normal: [0.0, 0.0, 1.0],
            tex_coords: [0.0, 1.0],
        },
        MaterialVertex3D {
            position: [1.0, -1.0, 0.0],
            normal: [0.0, 0.0, 1.0],
            tex_coords: [1.0, 1.0],
        },
    ];

    let indices = vec![0, 1, 2];

    let state = get_state();
    let vertices = VertexBuffer::new(&state.display, &vertices)?;
    let indices = IndexBuffer::new(
        &state.display,
        glium::index::PrimitiveType::TrianglesList,
        &indices,
    )?;

    let triangle = Object3D {
        mesh: Mesh { vertices, indices }.create(),
        material: create_flat_3d_material(Color::RED_500),
        transform: Transform3D::IDENTITY,
    };

    Ok(triangle.create())
}

pub struct Mesh {
    pub vertices: VertexBuffer<MaterialVertex3D>,
    pub indices: IndexBuffer<u32>,
}

gen_ref_type!(Mesh, MeshRef, meshes);
