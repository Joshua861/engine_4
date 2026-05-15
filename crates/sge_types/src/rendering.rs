use std::ops::Index;
use std::{fmt::Debug, ops::IndexMut};

use glium::{
    Rect,
    buffer::BufferCreationError,
    implement_uniform_block, implement_vertex,
    texture::{ClientFormat, MipmapsOption, RawImage1d, Texture1d, UncompressedFloatFormat},
};
use sge_color::Color;
use sge_macros::gen_ref_type;
use sge_utils::ConstantArray;
use sge_vectors::{Vec2, Vec3};
use sge_window::get_display;

use crate::{Area, PatternVertex2D};

// ////////////////////////////////////////////////////////////////////////////
//                                  Rounded                                  //
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct RoundedBatch {
    pub instances: Vec<RoundedInstance>,
    pub scissor: Option<glium::Rect>,
}

implement_vertex!(
    RoundedInstance,
    dimensions,
    center,
    corner_radius,
    outline_thickness,
    fill_color,
    outline_color
);
#[derive(Copy, Clone, Debug)]
pub struct RoundedInstance {
    pub dimensions: [f32; 2],
    pub center: [f32; 3],
    pub corner_radius: f32,
    pub outline_thickness: f32,
    pub fill_color: [f32; 4],
    pub outline_color: [f32; 4],
}

impl RoundedInstance {
    pub fn new(
        dimensions: Vec2,
        center: Vec2,
        z: f32,
        corner_radius: f32,
        fill_color: Color,
        outline_thickness: f32,
        outline_color: Color,
    ) -> Self {
        Self {
            dimensions: dimensions.into(),
            center: [center.x, center.y, z],
            corner_radius,
            outline_thickness,
            fill_color: fill_color.for_gpu(),
            outline_color: outline_color.for_gpu(),
        }
    }
}

impl RoundedBatch {
    pub fn new(scissor: Option<glium::Rect>) -> Self {
        Self {
            instances: Vec::new(),
            scissor,
        }
    }
}

// ////////////////////////////////////////////////////////////////////////////
//                                   Circle                                  //
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct CircleBatch {
    pub instances: Vec<CircleInstance>,
    pub scissor: Option<glium::Rect>,
}

impl CircleBatch {
    pub fn new(scissor: Option<glium::Rect>) -> Self {
        Self {
            instances: Vec::new(),
            scissor,
        }
    }
}

implement_vertex!(
    CircleInstance,
    center,
    radius,
    fill_color,
    outline_thickness,
    outline_color,
    start_angle,
    end_angle
);
#[derive(Copy, Clone, Debug)]
pub struct CircleInstance {
    pub center: [f32; 3],
    pub radius: [f32; 2],
    pub fill_color: [f32; 4],
    pub outline_thickness: f32,
    pub outline_color: [f32; 4],
    pub start_angle: f32,
    pub end_angle: f32,
}

impl CircleInstance {
    pub fn new(center: Vec2, z: f32, radius: Vec2, fill_color: Color) -> Self {
        Self {
            center: [center.x, center.y, z],
            radius: radius.into(),
            fill_color: fill_color.for_gpu(),
            outline_thickness: 0.0,
            outline_color: fill_color.for_gpu(),
            start_angle: 0.0,
            end_angle: 0.0,
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
            start_angle: 0.0,
            end_angle: std::f32::consts::TAU,
        }
    }

    pub fn new_sector(
        center: Vec2,
        z: f32,
        radius: Vec2,
        fill_color: Color,
        start_angle: f32,
        end_angle: f32,
    ) -> Self {
        Self {
            center: [center.x, center.y, z],
            radius: radius.into(),
            fill_color: fill_color.for_gpu(),
            outline_thickness: 0.0,
            outline_color: fill_color.for_gpu(),
            start_angle,
            end_angle,
        }
    }

    pub fn new_sector_with_outline(
        center: Vec2,
        z: f32,
        radius: Vec2,
        fill_color: Color,
        outline_thickness: f32,
        outline_color: Color,
        start_angle: f32,
        end_angle: f32,
    ) -> Self {
        Self {
            center: [center.x, center.y, z],
            radius: radius.into(),
            fill_color: fill_color.for_gpu(),
            outline_thickness,
            outline_color: outline_color.for_gpu(),
            start_angle,
            end_angle,
        }
    }
}

// ////////////////////////////////////////////////////////////////////////////
//                                   Shape                                   //
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct MeshBatch {
    pub vertices: Vec<PatternVertex2D>,
    pub indices: Vec<u32>,
    pub max_index: u32,
    pub scissor: Option<glium::Rect>,
}

impl MeshBatch {
    pub fn new(scissor: Option<glium::Rect>) -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            max_index: 0,
            scissor,
        }
    }
}

/// ///////////////////////////////////////////////////////////////////////////
//                              Radial gradient                              //
///////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub struct RadialGradientInstance {
    pub center: [f32; 3],
    pub radius: [f32; 2],
    pub outline_thickness: f32,
    pub inner_color: [f32; 4],
    pub outer_color: [f32; 4],
    pub outline_color: [f32; 4],
    pub gradient_offset: [f32; 2],
}

implement_vertex!(
    RadialGradientInstance,
    center,
    radius,
    outline_thickness,
    inner_color,
    outer_color,
    outline_color,
    gradient_offset
);

impl RadialGradientInstance {
    pub fn new(center: Vec2, z: f32, radius: Vec2, inner: Color, outer: Color) -> Self {
        Self {
            center: [center.x, center.y, z],
            radius: [radius.x, radius.y],
            outline_thickness: 0.0,
            inner_color: inner.for_gpu(),
            outer_color: outer.for_gpu(),
            outline_color: [0.0; 4],
            gradient_offset: [0.0; 2],
        }
    }

    pub fn new_with_outline(
        center: Vec2,
        z: f32,
        radius: Vec2,
        inner: Color,
        outer: Color,
        outline_thickness: f32,
        outline_color: Color,
    ) -> Self {
        Self {
            center: [center.x, center.y, z],
            radius: [radius.x, radius.y],
            outline_thickness,
            inner_color: inner.for_gpu(),
            outer_color: outer.for_gpu(),
            outline_color: outline_color.for_gpu(),
            gradient_offset: [0.0; 2],
        }
    }

    pub fn new_offset(
        center: Vec2,
        z: f32,
        radius: Vec2,
        inner: Color,
        outer: Color,
        gradient_offset: Vec2,
    ) -> Self {
        Self {
            center: [center.x, center.y, z],
            radius: [radius.x, radius.y],
            outline_thickness: 0.0,
            inner_color: inner.for_gpu(),
            outer_color: outer.for_gpu(),
            outline_color: [0.0; 4],
            gradient_offset: [gradient_offset.x, gradient_offset.y],
        }
    }
}

#[derive(Clone)]
pub struct RadialGradientBatch {
    pub instances: Vec<RadialGradientInstance>,
    pub scissor: Option<glium::Rect>,
}

impl RadialGradientBatch {
    pub fn new(scissor: Option<glium::Rect>) -> Self {
        Self {
            instances: Vec::new(),
            scissor,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
//                              Quadratic Bezier                             //
///////////////////////////////////////////////////////////////////////////////

implement_vertex!(QuadraticBezier, a, b, c, color, thickness);

#[derive(Clone, Copy)]
pub struct QuadraticBezier {
    pub a: [f32; 2],
    pub b: [f32; 2],
    pub c: [f32; 2],
    pub color: [f32; 4],
    pub thickness: f32,
}

impl QuadraticBezier {
    pub fn new(a: Vec2, b: Vec2, c: Vec2, color: Color, thickness: f32) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
            c: c.into(),
            color: color.for_gpu(),
            thickness,
        }
    }
}

#[derive(Clone)]
pub struct QuadraticBezierBatch {
    pub instances: Vec<QuadraticBezier>,
    pub scissor: Option<glium::Rect>,
}

impl QuadraticBezierBatch {
    pub fn new(scissor: Option<glium::Rect>) -> Self {
        Self {
            instances: Vec::new(),
            scissor,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
//                                Cubic Bezier                               //
///////////////////////////////////////////////////////////////////////////////

implement_vertex!(CubicBezier, a, b, c, d, color, thickness);

#[derive(Clone, Copy)]
pub struct CubicBezier {
    pub a: [f32; 2],
    pub b: [f32; 2],
    pub c: [f32; 2],
    pub d: [f32; 2],
    pub color: [f32; 4],
    pub thickness: f32,
}

impl CubicBezier {
    pub fn new(a: Vec2, b: Vec2, c: Vec2, d: Vec2, color: Color, thickness: f32) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
            c: c.into(),
            d: d.into(),
            color: color.for_gpu(),
            thickness,
        }
    }
}

#[derive(Clone)]
pub struct CubicBezierBatch {
    pub instances: Vec<CubicBezier>,
    pub scissor: Option<glium::Rect>,
}

impl CubicBezierBatch {
    pub fn new(scissor: Option<glium::Rect>) -> Self {
        Self {
            instances: Vec::new(),
            scissor,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
//                                 Metaballs                                 //
///////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Metaball {
    pub center: [f32; 2],
    pub radius: f32,
    pub _pad: f32, // 16-byte alignment
}

impl Debug for Metaball {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Metaball(x: {}, y: {}, r: {})",
            self.center[0], self.center[1], self.radius
        ))
    }
}

impl Metaball {
    pub fn new(center: Vec2, radius: f32) -> Self {
        Self {
            center: center.into(),
            radius,
            _pad: 0.0,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MetaballBlock {
    pub centers: [[f32; 2]; METABALLS_N],
    pub radii: [f32; METABALLS_N],
    pub pads: [f32; METABALLS_N],
}

implement_uniform_block!(MetaballBlock, centers, radii, pads);

impl Metaballs {
    pub fn new() -> Result<Self, BufferCreationError> {
        MetaballBatch::new(None)
    }
}

gen_ref_type!(MetaballBatch, Metaballs, metaballs);

const METABALLS_N: usize = 64;

#[derive(Debug)]
pub struct MetaballBatch {
    color: Color,
    bounding_box: Area,
    data: ConstantArray<Metaball, METABALLS_N>,
    texture_dirty: bool,
    bounding_box_dirty: bool,
    texture: Texture1d,
    pub scissor: Option<Rect>,
}

impl MetaballBatch {
    pub fn new(scissor: Option<Rect>) -> Result<Metaballs, BufferCreationError> {
        let texture = Texture1d::empty_with_format(
            get_display(),
            UncompressedFloatFormat::F32F32F32F32,
            MipmapsOption::NoMipmap,
            METABALLS_N as u32,
        )
        .expect("failed to create metaball texture");

        Ok(Self {
            bounding_box: Area::new(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0)),
            data: ConstantArray::new(),
            texture_dirty: false,
            bounding_box_dirty: true,
            color: Color::WHITE,
            texture,
            scissor,
        }
        .create())
    }

    pub fn get_ball(&self, n: usize) -> Option<&Metaball> {
        self.data.as_slice().get(n)
    }

    pub fn get_ball_mut(&mut self, n: usize) -> Option<&mut Metaball> {
        if let Some(ball) = self.data.as_mut_slice().get_mut(n) {
            self.texture_dirty = true;
            self.bounding_box_dirty = true;
            Some(ball)
        } else {
            None
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn texture(&self) -> &Texture1d {
        &self.texture
    }

    fn update(&mut self) {
        if self.texture_dirty {
            let mut raw_data = vec![0.0f32; METABALLS_N * 4];
            for (i, ball) in self.data.as_slice().iter().enumerate() {
                raw_data[i * 4] = ball.center[0];
                raw_data[i * 4 + 1] = ball.center[1];
                raw_data[i * 4 + 2] = ball.radius;
                raw_data[i * 4 + 3] = 0.0;
            }
            let raw = RawImage1d {
                data: raw_data.into(),
                width: METABALLS_N as u32,
                format: ClientFormat::F32F32F32F32,
            };
            self.texture = Texture1d::with_format(
                get_display(),
                raw,
                UncompressedFloatFormat::F32F32F32F32,
                MipmapsOption::NoMipmap,
            )
            .expect("failed to update metaball texture");
            self.texture_dirty = false;
        }

        if self.bounding_box_dirty {
            self.recalculate_bounding_box();
            self.bounding_box_dirty = false;
        }
    }

    pub fn add_metaball(&mut self, ball: Metaball) -> Result<(), sge_utils::CapacityReached> {
        self.texture_dirty = true;
        self.grow_bounding_box(ball);
        self.data.push(ball)
    }

    pub fn remove_metaball(&mut self) -> Option<()> {
        if self.data.pop().is_some() {
            self.texture_dirty = true;
            self.bounding_box_dirty = true;
            Some(())
        } else {
            None
        }
    }

    /// this function is run by the engine internally
    pub unsafe fn init_storage() {
        init_metaballs_storage();
    }

    /// this function is run by the engine internally
    pub unsafe fn update_all() {
        for ball in get_metaballs_state() {
            ball.update();
        }
    }

    pub fn bounding_box(&self) -> Area {
        self.bounding_box
    }

    fn grow_bounding_box(&mut self, ball: Metaball) {
        let center = ball.center;
        let radius = ball.radius;

        let ball_min = Vec2::new(center[0] - radius, center[1] - radius);
        let ball_max = Vec2::new(center[0] + radius, center[1] + radius);

        let bb_min = self.bounding_box.top_left;
        let bb_max = self.bounding_box.bottom_right();

        let new_bb_min = Vec2::new(bb_min.x.min(ball_min.x), bb_min.y.min(ball_min.y));
        let new_bb_max = Vec2::new(bb_max.x.max(ball_max.x), bb_max.y.max(ball_max.y));

        self.bounding_box.top_left = new_bb_min;
        self.bounding_box.size = new_bb_max - new_bb_min;
    }

    fn recalculate_bounding_box(&mut self) {
        let slice = self.data.as_slice();
        if slice.is_empty() {
            self.bounding_box = Area::new(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0));
            return;
        }

        let mut min = Vec2::new(f32::INFINITY, f32::INFINITY);
        let mut max = Vec2::new(f32::NEG_INFINITY, f32::NEG_INFINITY);

        for ball in slice.iter() {
            let c = Vec2::new(ball.center[0], ball.center[1]);
            let r = ball.radius;
            min.x = min.x.min(c.x - r);
            min.y = min.y.min(c.y - r);
            max.x = max.x.max(c.x + r);
            max.y = max.y.max(c.y + r);
        }

        self.bounding_box = Area::new(min, max - min);
    }
}

impl Index<usize> for MetaballBatch {
    type Output = Metaball;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for MetaballBatch {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.texture_dirty = true;
        self.bounding_box_dirty = true;
        &mut self.data[index]
    }
}

///////////////////////////////////////////////////////////////////////////////
//                                   Points                                  //
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct PointBatch {
    pub vertices: Vec<PatternVertex2D>,
    pub max_index: u32,
    pub scissor: Option<glium::Rect>,
}

impl PointBatch {
    pub fn new(scissor: Option<glium::Rect>) -> Self {
        Self {
            vertices: Vec::new(),
            max_index: 0,
            scissor,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
//                                   Lines                                   //
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct LineBatch {
    pub vertices: Vec<PatternVertex2D>,
    pub indices: Vec<u32>,
    pub max_index: u32,
    pub scissor: Option<glium::Rect>,
}

impl LineBatch {
    pub fn new(scissor: Option<glium::Rect>) -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            max_index: 0,
            scissor,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
//                                    SDF                                    //
///////////////////////////////////////////////////////////////////////////////

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SdfShape {
    Rect = 0,
    Ellipse,
    Triangle,
    Quad,
    Sector,
    Ring,
    Arc,
    Pentagon,
    Hexagon,
    Octogon,
    Hexagram,
    Pentagram,
    Star,
    Moon,
    Heart,
    Cross,
    X,
    QuadraticBezier,
    QuadraticCircle,
    Segment,
    OrientedBox,
    CubicBezier,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SdfFill {
    Solid = 0,
    Gradient = 1,
    Checker = 2,
    Lines = 3,
    Dots = 4,
    Grid = 5,
    Waves = 6,
    ConcentricRings = 7,
    RadialGradient = 8,
    CrossHatch = 9,
    SparseDots = 10,
    Bricks = 11,
    Herringbone = 12,
    Triangles = 13,
    ConcentricSquares = 14,
    Textured = 15,
    Truchet = 16,
    RandomTiles = 17,
    DiagonalWaves = 18,
    Topology = 19,
    Zebra = 20,
    FishScales = 21,
    Maze = 22,
    Moire = 23,
    LeopardSpots = 24,
    Rings = 25,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SdfStroke {
    None = 0,
    Inside = 1,
    Outside = 2,
    Centered = 3,
}

implement_vertex!(
    SdfInstanceGpu,
    center,
    bounding_box,
    shape_params_a,
    shape_params_b,
    fill_color_a,
    fill_color_b,
    stroke_color,
    shadow_color,
    misc_a,
    misc_b,
    misc_c,
    misc_d,
);

#[derive(Clone, Copy, Debug)]
pub struct Sdf {
    pub center: Vec3,
    // half extents
    pub bounding_box: Vec2,
    pub rotation: f32,

    pub shape_type: SdfShape,
    pub corner_radius: f32,

    // Sector: [start_angle, end_angle, ...]
    // Ring, Arc: [start_angle, end_angle, thickness, ...]
    // Triangle: [ax, ay, bx, by, cx, cy, ...]
    // Quad: [ax, ay, bx, by, cx, cy, dx, dy]
    // Star: [n_sides, m_ratio, ...]
    // Moon: [center, outer_radius, inner_offset, inner_radius]
    // QuadraticBezier: [ax, ay, bx, by, cx, cy, ...]
    // Segment: [ax, ay, bx, by, ...]
    // Oriented box: [ax ,ay, bx, by, thickness, ...]
    // CubicBezier: [ax, ay, bx, by, cx, cy, dx, dy]
    pub shape_params_a: [f32; 4],
    pub shape_params_b: [f32; 4],

    pub fill_type: SdfFill,
    pub fill_color_a: Color,
    pub fill_color_b: Color,
    pub fill_angle: f32,
    pub fill_offset: Vec2,
    pub fill_scale: f32,

    pub stroke_width: f32,
    pub stroke_color: Color,
    pub stroke_type: SdfStroke,

    pub shadow_offset: Vec2,
    pub shadow_radius: f32,
    pub shadow_color: Color,
}

#[derive(Clone, Copy, Debug)]
pub struct SdfInstanceGpu {
    pub center: [f32; 3],
    pub bounding_box: [f32; 2],
    pub shape_params_a: [f32; 4],
    pub shape_params_b: [f32; 4],
    pub fill_color_a: [f32; 4],
    pub fill_color_b: [f32; 4],
    pub stroke_color: [f32; 4],
    pub shadow_color: [f32; 4],
    /// rotation, corner_radius, fill_angle, fill_scale
    pub misc_a: [f32; 4],
    /// stroke_width, shadow_radius, fill_offset.xy
    pub misc_b: [f32; 4],
    /// shadow_offset.xy, fill_type as f32, stroke_type as f32
    pub misc_c: [f32; 4],
    /// shape_type as f32, 0, 0, 0
    pub misc_d: [f32; 4],
}

fn default_instance(center: Vec3, bounding_box: Vec2, shape_type: SdfShape) -> Sdf {
    Sdf {
        center,
        bounding_box,
        rotation: 0.0,
        shape_type,
        corner_radius: 0.0,
        shape_params_a: [0.0; 4],
        shape_params_b: [0.0; 4],
        fill_type: SdfFill::Solid,
        fill_color_a: Color::WHITE,
        fill_color_b: Color::WHITE,
        fill_angle: 0.0,
        fill_offset: Vec2::ZERO,
        fill_scale: 1.0,
        stroke_width: 0.0,
        stroke_color: Color::WHITE,
        stroke_type: SdfStroke::Inside,
        shadow_offset: Vec2::ZERO,
        shadow_radius: 0.0,
        shadow_color: Color::WHITE,
    }
}

#[derive(Clone)]
pub struct SdfBatch {
    pub instances: Vec<Sdf>,
    pub scissor: Option<glium::Rect>,
}

impl SdfBatch {
    pub fn new(scissor: Option<glium::Rect>) -> Self {
        Self {
            instances: Vec::new(),
            scissor,
        }
    }
}

impl Sdf {
    pub fn to_gpu(&self) -> SdfInstanceGpu {
        SdfInstanceGpu {
            center: self.center.to_array(),
            bounding_box: self.bounding_box.to_array(),
            shape_params_a: self.shape_params_a,
            shape_params_b: self.shape_params_b,
            fill_color_a: self.fill_color_a.for_gpu(),
            fill_color_b: self.fill_color_b.for_gpu(),
            stroke_color: self.stroke_color.for_gpu(),
            shadow_color: self.shadow_color.for_gpu(),
            misc_a: [
                self.rotation,
                self.corner_radius,
                self.fill_angle,
                self.fill_scale,
            ],
            misc_b: [
                self.stroke_width,
                self.shadow_radius,
                self.fill_offset.x,
                self.fill_offset.y,
            ],
            misc_c: [
                self.shadow_offset.x,
                self.shadow_offset.y,
                self.fill_type as i32 as f32,
                self.stroke_type as i32 as f32,
            ],
            misc_d: [self.shape_type as i32 as f32, 0.0, 0.0, 0.0],
        }
    }

    pub fn rect(center: Vec2, size: Vec2) -> Self {
        default_instance(center.extend(0.0), size * 0.5, SdfShape::Rect)
    }

    pub fn rect_tl(top_left: Vec2, size: Vec2) -> Self {
        let center = top_left + size * 0.5;
        default_instance(center.extend(0.0), size * 0.5, SdfShape::Rect)
    }

    pub fn square(center: Vec2, size: f32) -> Self {
        default_instance(center.extend(0.0), Vec2::splat(size * 0.5), SdfShape::Rect)
    }

    pub fn square_tl(top_left: Vec2, size: f32) -> Self {
        let center = top_left + Vec2::splat(size * 0.5);
        default_instance(center.extend(0.0), Vec2::splat(size * 0.5), SdfShape::Rect)
    }

    pub fn circle(center: Vec2, radius: f32) -> Self {
        default_instance(center.extend(0.0), Vec2::splat(radius), SdfShape::Ellipse)
    }

    pub fn ellipse(center: Vec2, radii: Vec2) -> Self {
        default_instance(center.extend(0.0), radii, SdfShape::Ellipse)
    }

    pub fn quad(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> Self {
        let min_coord = a.min(b).min(c).min(d);
        let max_coord = a.max(b).max(c).max(d);
        let dimensions = (max_coord - min_coord) * 0.5;
        let center = (min_coord + max_coord) * 0.5;
        let la = a - center;
        let lb = b - center;
        let lc = c - center;
        let ld = d - center;
        let mut inst = default_instance(center.extend(0.0), dimensions, SdfShape::Quad);
        inst.shape_params_a = [la.x, la.y, lb.x, lb.y];
        inst.shape_params_b = [lc.x, lc.y, ld.x, ld.y];
        inst
    }

    pub fn sector(center: Vec2, radius: Vec2, start_angle: f32, end_angle: f32) -> Self {
        let mut inst = default_instance(center.extend(0.0), radius, SdfShape::Sector);
        inst.shape_params_a = [start_angle, end_angle, 0.0, 0.0];
        inst
    }

    pub fn ring(
        center: Vec2,
        radius: Vec2,
        thickness: f32,
        start_angle: f32,
        end_angle: f32,
    ) -> Self {
        let mut inst = default_instance(center.extend(0.0), radius, SdfShape::Ring);
        inst.shape_params_a = [start_angle, end_angle, thickness, 0.0];
        inst
    }

    pub fn arc(
        center: Vec2,
        radius: Vec2,
        thickness: f32,
        start_angle: f32,
        end_angle: f32,
    ) -> Self {
        let mut inst = default_instance(center.extend(0.0), radius, SdfShape::Arc);
        inst.shape_params_a = [start_angle, end_angle, thickness, 0.0];
        inst
    }

    pub fn triangle(a: Vec2, b: Vec2, c: Vec2) -> Self {
        let min = a.min(b).min(c);
        let max = a.max(b).max(c);
        let center = (min + max) * 0.5;
        let dimensions = (max - min) * 0.5;
        let la = a - center;
        let lb = b - center;
        let lc = c - center;
        let mut inst = default_instance(center.extend(0.0), dimensions, SdfShape::Triangle);
        inst.shape_params_a = [la.x, la.y, lb.x, lb.y];
        inst.shape_params_b = [lc.x, lc.y, 0.0, 0.0];
        inst
    }

    pub fn pentagon(center: Vec2, radius: f32) -> Self {
        default_instance(center.extend(0.0), Vec2::splat(radius), SdfShape::Pentagon)
    }

    pub fn hexagon(center: Vec2, radius: f32) -> Self {
        default_instance(center.extend(0.0), Vec2::splat(radius), SdfShape::Hexagon)
    }

    pub fn octogon(center: Vec2, radius: f32) -> Self {
        default_instance(center.extend(0.0), Vec2::splat(radius), SdfShape::Octogon)
    }

    pub fn hexagram(center: Vec2, radius: f32) -> Self {
        default_instance(center.extend(0.0), Vec2::splat(radius), SdfShape::Hexagram)
    }

    pub fn pentagram(center: Vec2, radius: f32) -> Self {
        default_instance(center.extend(0.0), Vec2::splat(radius), SdfShape::Pentagram)
    }

    pub fn star(center: Vec2, radius: f32, n_sides: f32, m_ratio: f32) -> Self {
        let mut inst = default_instance(center.extend(0.0), Vec2::splat(radius), SdfShape::Star);
        inst.shape_params_a = [n_sides, m_ratio, 0.0, 0.0];
        inst
    }

    pub fn moon(center: Vec2, outer_radius: f32, inner_offset: f32, inner_radius: f32) -> Self {
        let mut inst = default_instance(
            center.extend(0.0),
            Vec2::splat(outer_radius),
            SdfShape::Moon,
        );
        inst.shape_params_a = [inner_offset, inner_radius, 0.0, 0.0];
        inst
    }

    pub fn heart(center: Vec2, radius: f32) -> Self {
        default_instance(center.extend(0.0), Vec2::splat(radius), SdfShape::Heart)
    }

    // pub fn cross(center: Vec2, size: Vec2) -> Self {
    //     default_instance(center.extend(0.0), size * 0.5, SdfShape::Cross)
    // }

    // pub fn x_shape(center: Vec2, size: Vec2) -> Self {
    //     default_instance(center.extend(0.0), size * 0.5, SdfShape::X)
    // }

    pub fn quadratic_bezier(a: Vec2, b: Vec2, c: Vec2) -> Self {
        let (min, max) = quadratic_bezier_bounds(a, b, c);

        let center = (min + max) * 0.5;
        let dimensions = (max - min) * 0.5;

        let la = a - center;
        let lb = b - center;
        let lc = c - center;

        let mut inst = default_instance(center.extend(0.0), dimensions, SdfShape::QuadraticBezier);

        inst.shape_params_a = [la.x, la.y, lb.x, lb.y];
        inst.shape_params_b = [lc.x, lc.y, 0.0, 0.0];

        inst
    }

    pub fn cubic_bezier(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> Self {
        let (min, max) = cubic_bezier_bounds(a, b, c, d);

        let center = (min + max) * 0.5;
        let dimensions = (max - min) * 0.5;

        let la = a - center;
        let lb = b - center;
        let lc = c - center;
        let ld = d - center;

        let mut inst = default_instance(center.extend(0.0), dimensions, SdfShape::CubicBezier);

        inst.shape_params_a = [la.x, la.y, lb.x, lb.y];
        inst.shape_params_b = [lc.x, lc.y, ld.x, ld.y];

        inst
    }

    pub fn quadratic_circle(center: Vec2, radius: f32) -> Self {
        default_instance(
            center.extend(0.0),
            Vec2::splat(radius),
            SdfShape::QuadraticCircle,
        )
    }

    pub fn segment(a: Vec2, b: Vec2) -> Self {
        let min = a.min(b);
        let max = a.max(b);
        let center = (min + max) * 0.5;
        let bb = (max - min) * 0.5;
        let la = a - center;
        let lb = b - center;
        let mut inst = default_instance(center.extend(0.0), bb, SdfShape::Segment);
        inst.shape_params_a = [la.x, la.y, lb.x, lb.y];
        inst
    }

    pub fn oriented_box(a: Vec2, b: Vec2, thickness: f32) -> Self {
        let min = a.min(b);
        let max = a.max(b);
        let center = (min + max) * 0.5;
        let bb = ((max - min) * 0.5) + thickness;
        let la = a - center;
        let lb = b - center;
        let mut inst = default_instance(center.extend(0.0), bb, SdfShape::OrientedBox);
        inst.shape_params_a = [la.x, la.y, lb.x, lb.y];
        inst.shape_params_b = [thickness, 0.0, 0.0, 0.0];
        inst
    }

    pub fn get_position(&self) -> Vec2 {
        self.center.truncate()
    }
    pub fn set_position(&mut self, pos: Vec2) {
        self.center = pos.extend(self.center.z);
    }

    pub fn get_depth(&self) -> f32 {
        self.center.z
    }
    pub fn set_depth(&mut self, z: f32) {
        self.center.z = z;
    }

    pub fn get_dimensions(&self) -> Vec2 {
        self.bounding_box
    }
    pub fn set_dimensions(&mut self, d: Vec2) {
        self.bounding_box = d;
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }
    pub fn set_rotation(&mut self, r: f32) {
        self.rotation = r;
    }

    pub fn get_corner_radius(&self) -> f32 {
        self.corner_radius
    }
    pub fn set_corner_radius(&mut self, r: f32) {
        self.corner_radius = r;
    }

    pub fn get_shape_type(&self) -> SdfShape {
        self.shape_type
    }
    pub fn set_shape_type(&mut self, t: SdfShape) {
        self.shape_type = t;
    }

    pub fn get_shape_param(&self, i: usize) -> f32 {
        match i {
            0..=3 => self.shape_params_a[i],
            4..=7 => self.shape_params_b[i - 4],
            _ => panic!("shape param index {i} out of range 0..=7"),
        }
    }
    pub fn set_shape_param(&mut self, i: usize, v: f32) {
        match i {
            0..=3 => self.shape_params_a[i] = v,
            4..=7 => self.shape_params_b[i - 4] = v,
            _ => panic!("shape param index {i} out of range 0..=7"),
        }
    }

    pub fn get_fill_type(&self) -> SdfFill {
        self.fill_type
    }
    pub fn set_fill_type(&mut self, t: SdfFill) {
        self.fill_type = t;
    }

    pub fn get_fill_color_a(&self) -> Color {
        self.fill_color_a
    }
    pub fn set_fill_color_a(&mut self, c: Color) {
        self.fill_color_a = c;
    }

    pub fn get_fill_color_b(&self) -> Color {
        self.fill_color_b
    }
    pub fn set_fill_color_b(&mut self, c: Color) {
        self.fill_color_b = c;
    }

    pub fn get_fill_angle(&self) -> f32 {
        self.fill_angle
    }
    pub fn set_fill_angle(&mut self, a: f32) {
        self.fill_angle = a;
    }

    pub fn get_fill_scale(&self) -> f32 {
        self.fill_scale
    }
    pub fn set_fill_scale(&mut self, s: f32) {
        self.fill_scale = s;
    }

    pub fn get_fill_offset(&self) -> Vec2 {
        self.fill_offset
    }
    pub fn set_fill_offset(&mut self, o: Vec2) {
        self.fill_offset = o;
    }

    pub fn get_stroke_width(&self) -> f32 {
        self.stroke_width
    }
    pub fn set_stroke_width(&mut self, w: f32) {
        self.stroke_width = w;
    }

    pub fn get_stroke_color(&self) -> Color {
        self.stroke_color
    }
    pub fn set_stroke_color(&mut self, c: Color) {
        self.stroke_color = c;
    }

    pub fn get_stroke_type(&self) -> SdfStroke {
        self.stroke_type
    }
    pub fn set_stroke_type(&mut self, t: SdfStroke) {
        self.stroke_type = t;
    }

    pub fn get_shadow_offset(&self) -> Vec2 {
        self.shadow_offset
    }
    pub fn set_shadow_offset(&mut self, o: Vec2) {
        self.shadow_offset = o;
    }

    pub fn get_shadow_radius(&self) -> f32 {
        self.shadow_radius
    }
    pub fn set_shadow_radius(&mut self, r: f32) {
        self.shadow_radius = r;
    }

    pub fn get_shadow_color(&self) -> Color {
        self.shadow_color
    }
    pub fn set_shadow_color(&mut self, c: Color) {
        self.shadow_color = c;
    }

    pub fn get_color(&self) -> Color {
        self.fill_color_a
    }
    pub fn set_color(&mut self, color: Color) {
        self.fill_color_a = color;
    }

    pub fn with_fill_solid(mut self, color: Color) -> Self {
        self.fill_type = SdfFill::Solid;
        self.fill_color_a = color;
        self
    }

    pub fn with_fill_gradient(mut self, color_a: Color, color_b: Color, angle: f32) -> Self {
        self.fill_type = SdfFill::Gradient;
        self.fill_color_a = color_a;
        self.fill_color_b = color_b;
        self.fill_angle = angle;
        self.fill_scale = 1.0;
        self
    }

    pub fn with_fill_radial_gradient(mut self, color_a: Color, color_b: Color) -> Self {
        self.fill_type = SdfFill::RadialGradient;
        self.fill_color_a = color_a;
        self.fill_color_b = color_b;
        self.fill_scale = 1.0;
        self
    }

    pub fn with_fill_colors(mut self, color_a: Color, color_b: Color) -> Self {
        self.fill_color_a = color_a;
        self.fill_color_b = color_b;
        self
    }

    pub fn with_fill_scale(mut self, scale: f32) -> Self {
        self.fill_scale = scale;
        self
    }
    pub fn with_fill_angle(mut self, angle: f32) -> Self {
        self.fill_angle = angle;
        self
    }
    pub fn with_fill_type(mut self, fill_type: SdfFill) -> Self {
        self.fill_type = fill_type;
        self
    }

    pub fn with_fill(
        mut self,
        color_a: Color,
        color_b: Color,
        angle: f32,
        scale: f32,
        fill_type: SdfFill,
    ) -> Self {
        self.fill_type = fill_type;
        self.fill_color_a = color_a;
        self.fill_color_b = color_b;
        self.fill_angle = angle;
        self.fill_scale = scale;
        self
    }

    pub fn with_shadow(mut self, offset: Vec2, radius: f32, color: Color) -> Self {
        self.shadow_offset = offset;
        self.shadow_radius = radius;
        self.shadow_color = color;
        self
    }

    pub fn with_stroke(mut self, width: f32, color: Color, stroke_type: SdfStroke) -> Self {
        self.stroke_width = width;
        self.stroke_color = color;
        self.stroke_type = stroke_type;
        self
    }

    pub fn with_corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }
    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }
    pub fn with_depth(mut self, z: f32) -> Self {
        self.center.z = z;
        self
    }
    pub fn with_color(mut self, color: Color) -> Self {
        self.fill_color_a = color;
        self
    }

    pub fn with_fill_offset(mut self, offset: Vec2) -> Self {
        self.fill_offset = offset;
        self
    }
}

fn quadratic_bezier_bounds(p0: Vec2, p1: Vec2, p2: Vec2) -> (Vec2, Vec2) {
    let mut min = p0.min(p2);
    let mut max = p0.max(p2);

    for axis in 0..2 {
        let a = if axis == 0 { p0.x } else { p0.y };
        let b = if axis == 0 { p1.x } else { p1.y };
        let c = if axis == 0 { p2.x } else { p2.y };

        let denom = a - 2.0 * b + c;

        if denom.abs() > f32::EPSILON {
            let t = (a - b) / denom;

            if (0.0..=1.0).contains(&t) {
                let s = 1.0 - t;

                let q = s * s * a + 2.0 * s * t * b + t * t * c;

                if axis == 0 {
                    min.x = min.x.min(q);
                    max.x = max.x.max(q);
                } else {
                    min.y = min.y.min(q);
                    max.y = max.y.max(q);
                }
            }
        }
    }

    (min, max)
}

fn cubic_bezier_bounds(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2) -> (Vec2, Vec2) {
    let mut min = p0.min(p3);
    let mut max = p0.max(p3);

    for axis in 0..2 {
        let p0v = if axis == 0 { p0.x } else { p0.y };
        let p1v = if axis == 0 { p1.x } else { p1.y };
        let p2v = if axis == 0 { p2.x } else { p2.y };
        let p3v = if axis == 0 { p3.x } else { p3.y };

        // IQ formulation
        let c = -p0v + p1v;
        let b = p0v - 2.0 * p1v + p2v;
        let a = -p0v + 3.0 * p1v - 3.0 * p2v + p3v;

        let h = b * b - a * c;

        if h >= 0.0 && a.abs() > f32::EPSILON {
            let g = h.sqrt();

            let t1 = ((-b - g) / a).clamp(0.0, 1.0);
            let t2 = ((-b + g) / a).clamp(0.0, 1.0);

            for t in [t1, t2] {
                let s = 1.0 - t;

                let q = s * s * s * p0v
                    + 3.0 * s * s * t * p1v
                    + 3.0 * s * t * t * p2v
                    + t * t * t * p3v;

                if axis == 0 {
                    min.x = min.x.min(q);
                    max.x = max.x.max(q);
                } else {
                    min.y = min.y.min(q);
                    max.y = max.y.max(q);
                }
            }
        }
    }

    (min, max)
}
