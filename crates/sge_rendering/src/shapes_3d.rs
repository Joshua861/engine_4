use sge_color::Color;
use sge_types::MaterialVertex3D;
use sge_vectors::{Mat3, Vec3};

use crate::{
    api::create_flat_material,
    object_3d::{Mesh, MeshRef, Object3D, Object3DRef},
};

pub trait HasBounds3D {
    fn bounds(&self) -> AABB3D;
}

pub trait Shape3D: HasBounds3D {
    fn create_mesh(&self) -> Mesh;
}

#[derive(Debug, Clone, Copy)]
pub struct AABB3D {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB3D {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn from_center_size(center: Vec3, size: Vec3) -> Self {
        let half_size = size * 0.5;
        Self {
            min: center - half_size,
            max: center + half_size,
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    pub fn expand(self, amount: f32) -> Self {
        Self {
            min: self.min - Vec3::splat(amount),
            max: self.max + Vec3::splat(amount),
        }
    }
}

pub struct Line3D {
    pub start: Vec3,
    pub end: Vec3,
    pub thickness: f32,
}

impl Line3D {
    pub fn new(start: Vec3, end: Vec3, thickness: f32) -> Self {
        Self {
            start,
            end,
            thickness,
        }
    }

    pub fn as_cuboid(&self) -> Cuboid {
        Cuboid::from_segment(self.start, self.end, self.thickness)
    }
}

impl Shape3D for Line3D {
    fn create_mesh(&self) -> Mesh {
        self.as_cuboid().create_mesh()
    }
}

impl HasBounds3D for Line3D {
    fn bounds(&self) -> AABB3D {
        self.as_cuboid().bounds()
    }
}

pub fn line_3d(start: Vec3, end: Vec3, thickness: f32) -> MeshRef {
    let line = Line3D::new(start, end, thickness);
    line.create_mesh().create()
}

pub fn line_3d_flat(start: Vec3, end: Vec3, thickness: f32, color: Color) -> Object3DRef {
    let mesh = line_3d(start, end, thickness);
    let material = create_flat_material(color);
    Object3D::from_mesh_and_material(mesh, material)
}

pub fn cube_wireframe(center: Vec3, size: f32, thickness: f32) -> MeshRef {
    let half = size / 2.0;

    let v0 = center + Vec3::new(-half, -half, -half);
    let v1 = center + Vec3::new(half, -half, -half);
    let v2 = center + Vec3::new(half, -half, half);
    let v3 = center + Vec3::new(-half, -half, half);

    let v4 = center + Vec3::new(-half, half, -half);
    let v5 = center + Vec3::new(half, half, -half);
    let v6 = center + Vec3::new(half, half, half);
    let v7 = center + Vec3::new(-half, half, half);

    let edges = [
        (v0, v1),
        (v1, v2),
        (v2, v3),
        (v3, v0),
        (v4, v5),
        (v5, v6),
        (v6, v7),
        (v7, v4),
        (v0, v4),
        (v1, v5),
        (v2, v6),
        (v3, v7),
    ];

    let mut vertices = Vec::<MaterialVertex3D>::new();
    let mut indices = Vec::<u32>::new();

    const CUBOID_INDICES: &[u32] = &[
        0, 1, 2, 2, 1, 3, 4, 6, 5, 5, 6, 7, 0, 2, 4, 4, 2, 6, 1, 5, 3, 3, 5, 7, 0, 4, 1, 1, 4, 5,
        2, 3, 6, 6, 3, 7,
    ];

    for (p1_raw, p2_raw) in edges {
        let dir = (p2_raw - p1_raw).normalize();

        let p1 = p1_raw - dir * (thickness * 0.5);
        let p2 = p2_raw + dir * (thickness * 0.5);

        let cuboid = Cuboid::from_segment(p1, p2, thickness);

        let verts = cuboid.vertices();
        let base = vertices.len() as u32;

        for v in verts {
            vertices.push(MaterialVertex3D::from_pos(v));
        }

        for &i in CUBOID_INDICES {
            indices.push(base + i);
        }
    }

    Mesh::from_points(vertices, indices).unwrap()
}

pub fn cube_wireframe_flat(center: Vec3, size: f32, thickness: f32, color: Color) -> Object3DRef {
    let mesh = cube_wireframe(center, size, thickness);
    let material = create_flat_material(color);
    Object3D::from_mesh_and_material(mesh, material)
}

pub fn cuboid(center: Vec3, size: Vec3) -> MeshRef {
    Cuboid::new(center, size).create_mesh().create()
}

pub fn cube(center: Vec3, size: f32) -> MeshRef {
    cuboid(center, Vec3::splat(size))
}

pub fn cuboid_with_orientation(center: Vec3, size: Vec3, orientation: Mat3) -> MeshRef {
    Cuboid::new_with_orientation(center, size, orientation)
        .create_mesh()
        .create()
}

pub fn cube_with_orientation(center: Vec3, size: f32, orientation: Mat3) -> MeshRef {
    cuboid_with_orientation(center, Vec3::splat(size), orientation)
}

pub fn cuboid_from_extents(center: Vec3, extents: Vec3) -> MeshRef {
    Cuboid {
        center,
        extents,
        orientation: Mat3::IDENTITY,
    }
    .create_mesh()
    .create()
}

pub fn cuboid_from_extents_with_orientation(
    center: Vec3,
    extents: Vec3,
    orientation: Mat3,
) -> MeshRef {
    Cuboid {
        center,
        extents,
        orientation,
    }
    .create_mesh()
    .create()
}

#[derive(Debug, Clone, Copy)]
pub struct Cuboid {
    pub center: Vec3,
    pub extents: Vec3,
    pub orientation: Mat3,
}

impl Cuboid {
    pub fn new(center: Vec3, size: Vec3) -> Self {
        Self {
            center,
            extents: size * 0.5,
            orientation: Mat3::IDENTITY,
        }
    }

    pub fn new_with_orientation(center: Vec3, size: Vec3, orientation: Mat3) -> Self {
        Self {
            center,
            extents: size * 0.5,
            orientation,
        }
    }

    pub fn from_negative_corner(negative_corner: Vec3, size: Vec3) -> Self {
        Self {
            center: negative_corner + size * 0.5,
            extents: size * 0.5,
            orientation: Mat3::IDENTITY,
        }
    }

    pub fn from_negative_corner_with_orientation(
        negative_corner: Vec3,
        size: Vec3,
        orientation: Mat3,
    ) -> Self {
        Self {
            center: negative_corner + size * 0.5,
            extents: size * 0.5,
            orientation,
        }
    }

    pub fn from_segment(p1: Vec3, p2: Vec3, thickness: f32) -> Self {
        let dir = p2 - p1;
        let len = dir.length();
        let x_axis = dir.normalize();

        let ref_vec = if x_axis.y.abs() < 0.9 {
            Vec3::Y
        } else {
            Vec3::X
        };

        let z_axis = x_axis.cross(ref_vec).normalize();
        let y_axis = z_axis.cross(x_axis).normalize();
        let orientation = Mat3::from_cols(x_axis, y_axis, z_axis);

        Self {
            center: (p1 + p2) * 0.5,
            extents: Vec3::new(len * 0.5, thickness * 0.5, thickness * 0.5),
            orientation,
        }
    }

    fn vertices(&self) -> [Vec3; 8] {
        let ex = self.orientation * Vec3::new(self.extents.x, 0.0, 0.0);
        let ey = self.orientation * Vec3::new(0.0, self.extents.y, 0.0);
        let ez = self.orientation * Vec3::new(0.0, 0.0, self.extents.z);

        [
            self.center + ex + ey + ez,
            self.center + ex + ey - ez,
            self.center + ex - ey + ez,
            self.center + ex - ey - ez,
            self.center - ex + ey + ez,
            self.center - ex + ey - ez,
            self.center - ex - ey + ez,
            self.center - ex - ey - ez,
        ]
    }
}

impl Shape3D for Cuboid {
    fn create_mesh(&self) -> Mesh {
        let v = self.vertices();

        let vertices: Vec<MaterialVertex3D> =
            v.iter().map(|&p| MaterialVertex3D::from_pos(p)).collect();

        const INDICES: &[u32] = &[
            0, 1, 2, 2, 1, 3, 4, 6, 5, 5, 6, 7, 0, 2, 4, 4, 2, 6, 1, 5, 3, 3, 5, 7, 0, 4, 1, 1, 4,
            5, 2, 3, 6, 6, 3, 7,
        ];

        Mesh::from_points_internal(vertices, INDICES.to_vec()).unwrap()
    }
}

impl HasBounds3D for Cuboid {
    fn bounds(&self) -> AABB3D {
        let verts = self.vertices();
        let mut min = verts[0];
        let mut max = verts[0];

        for &v in &verts {
            min = min.min(v);
            max = max.max(v);
        }

        AABB3D::new(min, max)
    }
}
