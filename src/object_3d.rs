use std::{
    io::Cursor,
    ops::{Index, IndexMut},
    path::Path,
};

use bevy_math::{Mat4, Quat, Vec3};
use glium::{IndexBuffer, VertexBuffer, uniforms::Uniforms};
use obj::{FromRawVertex, load_obj, raw::object::Polygon};

use crate::{
    EngineStorage,
    color::Color,
    draw_queue_2d::MaterialVertex3D,
    draw_queue_3d::ObjectToDraw,
    get_state,
    materials::{DEFAULT_MATERIAL, MaterialRef},
    prelude::create_flat_3d_material,
};

pub struct Object3D {
    pub vertices: VertexBuffer<MaterialVertex3D>,
    pub indices: IndexBuffer<u32>,
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
        Self::from_obj_bytes(src.as_bytes())
    }

    // FIXME: polygon handling
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
            vertices,
            indices,
            material,
            transform: Transform3D::IDENTITY,
        };

        Ok(object.create())
    }

    pub fn create(self) -> Object3DRef {
        create_object(self)
    }
}

pub fn create_object(o: Object3D) -> Object3DRef {
    let state = get_state();
    let id = state.storage.objects.len();
    let id = Object3DRef(id);
    state.storage.objects.push(o);
    id
}

// FIXME: reduce repetition
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
                    // Triangulate position-only polygon
                    if v.len() < 3 {
                        continue;
                    }

                    // For n-gons, create a triangle fan
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
                    // Triangulate position+texture polygon
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
                    // Triangulate position+texture+normal polygon
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
                    // Triangulate position+normal polygon
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Object3DRef(usize);

impl Index<Object3DRef> for EngineStorage {
    type Output = Object3D;
    fn index(&self, index: Object3DRef) -> &Self::Output {
        &self.objects[index.0]
    }
}

impl IndexMut<Object3DRef> for EngineStorage {
    fn index_mut(&mut self, index: Object3DRef) -> &mut Self::Output {
        &mut self.objects[index.0]
    }
}

impl Object3DRef {
    pub fn get(&self) -> &Object3D {
        &get_state().storage.objects[self.0]
    }

    pub fn get_mut(&self) -> &mut Object3D {
        &mut get_state().storage.objects[self.0]
    }

    pub fn draw(&self) {
        get_state()
            .draw_queue_3d
            .objects
            .push(ObjectToDraw::Single(*self));
    }

    pub fn draw_many(&self, transforms: Vec<Transform3D>) {
        get_state().draw_queue_3d.objects.push(ObjectToDraw::Many {
            object: *self,
            transforms,
        });
    }

    pub fn draw_with_transform(&self, transform: Transform3D) {
        get_state()
            .draw_queue_3d
            .objects
            .push(ObjectToDraw::WithTransform(*self, transform));
    }

    pub fn with_transform(mut self, transform: Transform3D) -> Object3DRef {
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

    pub fn material(&self) -> &mut MaterialRef {
        &mut self.get_mut().material
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Transform3D {
    mat: Mat4,
    needs_update: bool,
    scale: Vec3,
    rotation: Quat,
    translation: Vec3,
}

impl Transform3D {
    pub const IDENTITY: Self = Self {
        mat: Mat4::IDENTITY,
        needs_update: false,
        scale: Vec3::ONE,
        rotation: Quat::IDENTITY,
        translation: Vec3::ZERO,
    };

    pub fn update_matrix(&mut self) {
        if !self.needs_update {
            return;
        }

        self.mat =
            Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation);
        self.needs_update = false;
    }

    pub fn matrix(&mut self) -> Mat4 {
        self.update_matrix();
        self.mat
    }

    fn mark_dirty(&mut self) {
        self.needs_update = true;
    }

    pub fn scale(&self) -> Vec3 {
        self.scale
    }

    pub fn rotation(&self) -> Quat {
        self.rotation
    }

    pub fn translation(&self) -> Vec3 {
        self.translation
    }

    pub fn scale_mut(&mut self) -> &mut Vec3 {
        self.mark_dirty();
        &mut self.scale
    }

    pub fn rotation_mut(&mut self) -> &mut Quat {
        self.mark_dirty();
        &mut self.rotation
    }

    pub fn translation_mut(&mut self) -> &mut Vec3 {
        self.mark_dirty();
        &mut self.translation
    }

    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            mat: Mat4::from_translation(translation),
            needs_update: false,
            scale: Vec3::ONE,
            rotation: Quat::IDENTITY,
            translation,
        }
    }

    pub fn from_scale(scale: Vec3) -> Self {
        Self {
            mat: Mat4::from_scale(scale),
            needs_update: false,
            scale,
            rotation: Quat::IDENTITY,
            translation: Vec3::ZERO,
        }
    }

    pub fn from_rotation(rotation: Quat) -> Self {
        Self {
            mat: Mat4::from_quat(rotation),
            needs_update: false,
            scale: Vec3::ONE,
            rotation,
            translation: Vec3::ZERO,
        }
    }

    pub fn from_scale_rotation_translation(scale: Vec3, rotation: Quat, translation: Vec3) -> Self {
        Self {
            mat: Mat4::from_scale_rotation_translation(scale, rotation, translation),
            needs_update: false,
            scale,
            rotation,
            translation,
        }
    }

    pub fn with_translation(mut self, translation: Vec3) -> Self {
        self.translation = translation;
        self.mark_dirty();
        self
    }

    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        self.mark_dirty();
        self
    }

    pub fn with_rotation(mut self, rotation: Quat) -> Self {
        self.rotation = rotation;
        self.mark_dirty();
        self
    }

    pub fn with_scale_rotation_translation(
        mut self,
        scale: Vec3,
        rotation: Quat,
        translation: Vec3,
    ) -> Self {
        self.scale = scale;
        self.rotation = rotation;
        self.translation = translation;
        self.mark_dirty();
        self
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
        vertices,
        indices,
        material: create_flat_3d_material(Color::RED_500),
        transform: Transform3D::IDENTITY,
    };

    Ok(triangle.create())
}
