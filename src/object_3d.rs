use std::{
    io::Cursor,
    ops::{Deref, DerefMut, Index, IndexMut},
};

use bevy_math::{BVec3, Mat3, Mat4, Quat, Vec3};
use glium::{BackfaceCullingMode, IndexBuffer, VertexBuffer};
use obj::{FromRawVertex, load_obj, raw::object::Polygon};

use crate::{
    EngineStorage,
    color::Color,
    draw_queue_2d::MaterialVertex3D,
    draw_queue_3d::ObjectToDraw,
    get_state,
    materials::{DEFAULT_MATERIAL, MaterialRef},
    prelude::{Material, create_flat_3d_material},
};

pub struct Object3D {
    pub mesh: MeshRef,
    pub material: MaterialRef,
    pub transform: Transform3D,
    pub draw_params_override: Option<glium::DrawParameters<'static>>,
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
            draw_params_override: None,
        };

        Ok(object.create())
    }

    pub fn compute_smooth_normals(&mut self) {
        use bevy_math::Vec3;
        use std::collections::HashMap;

        let vertices: Vec<MaterialVertex3D> = self.mesh.vertices.read().unwrap();
        let indices: Vec<u32> = self.mesh.indices.read().unwrap();

        let mut position_to_vertices: HashMap<[i32; 3], Vec<usize>> = HashMap::new();

        for (i, vertex) in vertices.iter().enumerate() {
            let key = [
                (vertex.position[0] * 1000.0) as i32,
                (vertex.position[1] * 1000.0) as i32,
                (vertex.position[2] * 1000.0) as i32,
            ];
            position_to_vertices
                .entry(key)
                .or_insert_with(Vec::new)
                .push(i);
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

    pub fn create(self) -> Object3DRef {
        create_object(self)
    }

    pub fn from_mesh_and_material(mesh: MeshRef, material: MaterialRef) -> Object3DRef {
        Self {
            mesh,
            material,
            transform: Transform3D::IDENTITY,
            draw_params_override: None,
        }
        .create()
    }
}

pub fn create_object(o: Object3D) -> Object3DRef {
    let state = get_state();
    let id = state.storage.objects.len();
    let id = Object3DRef(id);
    state.storage.objects.push(o);
    id
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

impl Deref for Object3DRef {
    type Target = Object3D;
    fn deref(&self) -> &Self::Target {
        &get_state().storage[*self]
    }
}

impl DerefMut for Object3DRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut get_state().storage[*self]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Transform3D {
    mat: Mat4,
    needs_update: bool,
    scale: Vec3,
    rotation: Quat,
    translation: Vec3,
    mirror: BVec3,
}

impl Transform3D {
    pub const IDENTITY: Self = Self {
        mat: Mat4::IDENTITY,
        needs_update: false,
        scale: Vec3::ONE,
        rotation: Quat::IDENTITY,
        translation: Vec3::ZERO,
        mirror: BVec3::FALSE,
    };

    pub fn update_matrix(&mut self) {
        if !self.needs_update {
            return;
        }

        let effective_scale = Vec3::new(
            if self.mirror.x {
                -self.scale.x
            } else {
                self.scale.x
            },
            if self.mirror.y {
                -self.scale.y
            } else {
                self.scale.y
            },
            if self.mirror.z {
                -self.scale.z
            } else {
                self.scale.z
            },
        );

        self.mat =
            Mat4::from_scale_rotation_translation(effective_scale, self.rotation, self.translation);
        self.needs_update = false;
    }

    pub fn should_flip_culling(&self) -> bool {
        let mirror_count = [self.mirror.x, self.mirror.y, self.mirror.z]
            .iter()
            .filter(|&&m| m)
            .count();
        mirror_count % 2 == 1
    }

    pub fn desired_culling_mode(&self) -> BackfaceCullingMode {
        if self.should_flip_culling() {
            glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise
        } else {
            glium::draw_parameters::BackfaceCullingMode::CullClockwise
        }
    }

    pub fn matrix(&mut self) -> Mat4 {
        self.update_matrix();
        self.mat
    }

    pub fn into_normal_matrix(&mut self) -> Mat3 {
        let mat = self.matrix();
        let mat = Mat3::from_mat4(mat);
        mat.inverse().transpose()
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

    pub fn mirror(&self) -> BVec3 {
        self.mirror
    }

    pub fn mirror_mut(&mut self) -> &mut BVec3 {
        self.mark_dirty();
        &mut self.mirror
    }

    pub fn mirror_x(&mut self) {
        self.mirror.x = !self.mirror.x;
        self.mark_dirty();
    }

    pub fn mirror_y(&mut self) {
        self.mirror.y = !self.mirror.y;
        self.mark_dirty();
    }

    pub fn mirror_z(&mut self) {
        self.mirror.z = !self.mirror.z;
        self.mark_dirty();
    }

    pub fn translate_by(&mut self, translation: Vec3) {
        self.translation += translation;
        self.mark_dirty();
    }

    pub fn rotate_by(&mut self, rotation: Quat) {
        self.rotation = self.rotation * rotation;
        self.mark_dirty();
    }

    pub fn scale_by(&mut self, scale: Vec3) {
        self.scale *= scale;
        self.mark_dirty();
    }

    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            mat: Mat4::from_translation(translation),
            needs_update: false,
            scale: Vec3::ONE,
            rotation: Quat::IDENTITY,
            translation,
            mirror: BVec3::new(false, false, false),
        }
    }

    pub fn from_scale(scale: Vec3) -> Self {
        Self {
            mat: Mat4::from_scale(scale),
            needs_update: false,
            scale,
            rotation: Quat::IDENTITY,
            translation: Vec3::ZERO,
            mirror: BVec3::new(false, false, false),
        }
    }

    pub fn from_rotation(rotation: Quat) -> Self {
        Self {
            mat: Mat4::from_quat(rotation),
            needs_update: false,
            scale: Vec3::ONE,
            rotation,
            translation: Vec3::ZERO,
            mirror: BVec3::new(false, false, false),
        }
    }

    pub fn from_scale_rotation_translation(scale: Vec3, rotation: Quat, translation: Vec3) -> Self {
        Self {
            mat: Mat4::from_scale_rotation_translation(scale, rotation, translation),
            needs_update: false,
            scale,
            rotation,
            translation,
            mirror: BVec3::new(false, false, false),
        }
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
        draw_params_override: None,
    };

    Ok(triangle.create())
}

pub struct Mesh {
    pub vertices: VertexBuffer<MaterialVertex3D>,
    pub indices: IndexBuffer<u32>,
}

impl Mesh {
    pub fn create(self) -> MeshRef {
        let state = get_state();

        let id = MeshRef(state.storage.meshes.len());
        state.storage.meshes.push(self);
        id
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MeshRef(usize);

impl Index<MeshRef> for EngineStorage {
    type Output = Mesh;
    fn index(&self, index: MeshRef) -> &Self::Output {
        &self.meshes[index.0]
    }
}

impl IndexMut<MeshRef> for EngineStorage {
    fn index_mut(&mut self, index: MeshRef) -> &mut Self::Output {
        &mut self.meshes[index.0]
    }
}

impl MeshRef {
    pub fn get(&self) -> &Mesh {
        &get_state().storage.meshes[self.0]
    }

    pub fn get_mut(&self) -> &mut Mesh {
        &mut get_state().storage.meshes[self.0]
    }
}

impl Deref for MeshRef {
    type Target = Mesh;
    fn deref(&self) -> &Self::Target {
        &get_state().storage[*self]
    }
}

impl DerefMut for MeshRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut get_state().storage[*self]
    }
}
