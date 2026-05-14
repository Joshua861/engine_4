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
use sge_vectors::Vec2;
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
pub struct ShapeBatch {
    pub vertices: Vec<PatternVertex2D>,
    pub indices: Vec<u32>,
    pub max_index: u32,
    pub scissor: Option<glium::Rect>,
}

impl ShapeBatch {
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
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SdfFill {
    Solid = 0,
    Gradient,
    Checker,
    Lines,
    Dots,
    Grid,
    Waves,
    ConcentricRings,
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
    SdfInstance,
    center,
    dimensions,
    shape_type,
    corner_radius,
    shape_params_a,
    shape_params_b,
    fill_type,
    fill_color_a,
    fill_color_b,
    fill_angle,
    fill_offset,
    fill_scale,
    stroke_width,
    stroke_color,
    stroke_type,
    shadow_offset,
    shadow_radius,
    shadow_color,
);

#[derive(Clone, Copy, Debug)]
pub struct SdfInstance {
    pub center: [f32; 3],
    // half extents
    pub dimensions: [f32; 2],

    pub shape_type: i32, // 0=rect, 1=circle, 2=capsule, 3=triangle, etc.
    pub corner_radius: f32,

    // Sector: [start_angle, end_angle, ...]
    // Ring, Arc: [start_angle, end_angle, thickness, ...]
    // Triangle: [ax, ay, bx, by, cx, cy, ...]
    // Quad: [ax, ay, bx, by, cx, cy, dx, dy]
    // Star: [n_sides, m_ratio, ...]
    // Moon: [center, outer_radius, inner_offset, inner_radius],
    // QuadraticBezier: [ax, ay, bx, by, cx, cy, ...]
    pub shape_params_a: [f32; 4],
    pub shape_params_b: [f32; 4],

    pub fill_type: i32,
    pub fill_color_a: [f32; 4],
    pub fill_color_b: [f32; 4],
    pub fill_angle: f32,
    pub fill_offset: [f32; 2],
    pub fill_scale: f32,

    pub stroke_width: f32,
    pub stroke_color: [f32; 4],
    pub stroke_type: i32,

    pub shadow_offset: [f32; 2],
    pub shadow_radius: f32,
    pub shadow_color: [f32; 4],
}

#[derive(Clone)]
pub struct SdfBatch {
    pub instances: Vec<SdfInstance>,
    pub scissor: Option<glium::Rect>,
}

impl SdfInstance {
    pub fn rect(center: Vec2, size: Vec2) -> SdfInstance {
        SdfInstance {
            center: [center.x, center.y, 0.0],
            dimensions: [size.x * 0.5, size.y * 0.5],
            shape_type: SdfShape::Rect as i32,
            corner_radius: 0.0,
            shape_params_a: [0.0; 4],
            shape_params_b: [0.0; 4],
            fill_type: SdfFill::Solid as i32,
            fill_color_a: [1.0; 4],
            fill_color_b: [1.0; 4],
            fill_angle: 0.0,
            fill_offset: [0.0; 2],
            fill_scale: 1.0,
            stroke_width: 0.0,
            stroke_color: [1.0; 4],
            stroke_type: SdfStroke::None as i32,
            shadow_offset: [0.0; 2],
            shadow_radius: 0.0,
            shadow_color: [1.0; 4],
        }
    }

    pub fn circle(center: Vec2, radius: f32) -> SdfInstance {
        SdfInstance {
            center: [center.x, center.y, 0.0],
            dimensions: [radius, radius],
            shape_type: SdfShape::Ellipse as i32,
            corner_radius: 0.0,
            shape_params_a: [0.0; 4],
            shape_params_b: [0.0; 4],
            fill_type: SdfFill::Solid as i32,
            fill_color_a: [1.0; 4],
            fill_color_b: [1.0; 4],
            fill_angle: 0.0,
            fill_offset: [0.0; 2],
            fill_scale: 1.0,
            stroke_width: 0.0,
            stroke_color: [1.0; 4],
            stroke_type: SdfStroke::None as i32,
            shadow_offset: [0.0; 2],
            shadow_radius: 0.0,
            shadow_color: [1.0; 4],
        }
    }

    pub fn ellipse(center: Vec2, radii: Vec2) -> SdfInstance {
        SdfInstance {
            center: [center.x, center.y, 0.0],
            dimensions: [radii.x, radii.y],
            shape_type: SdfShape::Ellipse as i32,
            corner_radius: 0.0,
            shape_params_a: [0.0; 4],
            shape_params_b: [0.0; 4],
            fill_type: SdfFill::Solid as i32,
            fill_color_a: [1.0; 4],
            fill_color_b: [1.0; 4],
            fill_angle: 0.0,
            fill_offset: [0.0; 2],
            fill_scale: 1.0,
            stroke_width: 0.0,
            stroke_color: [1.0; 4],
            stroke_type: SdfStroke::None as i32,
            shadow_offset: [0.0; 2],
            shadow_radius: 0.0,
            shadow_color: [1.0; 4],
        }
    }

    pub fn quad(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> SdfInstance {
        SdfInstance {
            center: [
                (a.x + b.x + c.x + d.x) * 0.25,
                (a.y + b.y + c.y + d.y) * 0.25,
                0.0,
            ],
            dimensions: [(a - c).length() * 0.5, (b - d).length() * 0.5],
            shape_type: SdfShape::Quad as i32,
            corner_radius: 0.0,
            shape_params_a: [a.x, a.y, b.x, b.y],
            shape_params_b: [c.x, c.y, d.x, d.y],
            fill_type: SdfFill::Solid as i32,
            fill_color_a: [1.0; 4],
            fill_color_b: [1.0; 4],
            fill_angle: 0.0,
            fill_offset: [0.0; 2],
            fill_scale: 1.0,
            stroke_width: 0.0,
            stroke_color: [1.0; 4],
            stroke_type: SdfStroke::None as i32,
            shadow_offset: [0.0; 2],
            shadow_radius: 0.0,
            shadow_color: [1.0; 4],
        }
    }

    pub fn sector(center: Vec2, radius: Vec2, start_angle: f32, end_angle: f32) -> SdfInstance {
        SdfInstance {
            center: [center.x, center.y, 0.0],
            dimensions: [radius.x, radius.y],
            shape_type: SdfShape::Sector as i32,
            corner_radius: 0.0,
            shape_params_a: [start_angle, end_angle, 0.0, 0.0],
            shape_params_b: [0.0; 4],
            fill_type: SdfFill::Solid as i32,
            fill_color_a: [1.0; 4],
            fill_color_b: [1.0; 4],
            fill_angle: 0.0,
            fill_offset: [0.0; 2],
            fill_scale: 1.0,
            stroke_width: 0.0,
            stroke_color: [1.0; 4],
            stroke_type: SdfStroke::None as i32,
            shadow_offset: [0.0; 2],
            shadow_radius: 0.0,
            shadow_color: [1.0; 4],
        }
    }

    pub fn ring(
        center: Vec2,
        radius: Vec2,
        thickness: f32,
        start_angle: f32,
        end_angle: f32,
    ) -> SdfInstance {
        SdfInstance {
            center: [center.x, center.y, 0.0],
            dimensions: [radius.x, radius.y],
            shape_type: SdfShape::Ring as i32,
            corner_radius: 0.0,
            shape_params_a: [start_angle, end_angle, thickness, 0.0],
            shape_params_b: [0.0; 4],
            fill_type: SdfFill::Solid as i32,
            fill_color_a: [1.0; 4],
            fill_color_b: [1.0; 4],
            fill_angle: 0.0,
            fill_offset: [0.0; 2],
            fill_scale: 1.0,
            stroke_width: 0.0,
            stroke_color: [1.0; 4],
            stroke_type: SdfStroke::None as i32,
            shadow_offset: [0.0; 2],
            shadow_radius: 0.0,
            shadow_color: [1.0; 4],
        }
    }

    pub fn arc(
        center: Vec2,
        radius: Vec2,
        thickness: f32,
        start_angle: f32,
        end_angle: f32,
    ) -> SdfInstance {
        SdfInstance {
            center: [center.x, center.y, 0.0],
            dimensions: [radius.x, radius.y],
            shape_type: SdfShape::Arc as i32,
            corner_radius: 0.0,
            shape_params_a: [start_angle, end_angle, thickness, 0.0],
            shape_params_b: [0.0; 4],
            fill_type: SdfFill::Solid as i32,
            fill_color_a: [1.0; 4],
            fill_color_b: [1.0; 4],
            fill_angle: 0.0,
            fill_offset: [0.0; 2],
            fill_scale: 1.0,
            stroke_width: 0.0,
            stroke_color: [1.0; 4],
            stroke_type: SdfStroke::None as i32,
            shadow_offset: [0.0; 2],
            shadow_radius: 0.0,
            shadow_color: [1.0; 4],
        }
    }

    pub fn triangle(a: Vec2, b: Vec2, c: Vec2) -> SdfInstance {
        SdfInstance {
            center: [(a.x + b.x + c.x) / 3.0, (a.y + b.y + c.y) / 3.0, 0.0],
            dimensions: [(a - b).length() * 0.5, (b - c).length() * 0.5],
            shape_type: SdfShape::Triangle as i32,
            corner_radius: 0.0,
            shape_params_a: [a.x, a.y, b.x, b.y],
            shape_params_b: [c.x, c.y, 0.0, 0.0],
            fill_type: SdfFill::Solid as i32,
            fill_color_a: [1.0; 4],
            fill_color_b: [1.0; 4],
            fill_angle: 0.0,
            fill_offset: [0.0; 2],
            fill_scale: 1.0,
            stroke_width: 0.0,
            stroke_color: [1.0; 4],
            stroke_type: SdfStroke::None as i32,
            shadow_offset: [0.0; 2],
            shadow_radius: 0.0,
            shadow_color: [1.0; 4],
        }
    }

    pub fn pentagon(center: Vec2, radius: f32) -> SdfInstance {
        SdfInstance {
            center: [center.x, center.y, 0.0],
            dimensions: [radius, radius],
            shape_type: SdfShape::Pentagon as i32,
            corner_radius: 0.0,
            shape_params_a: [0.0; 4],
            shape_params_b: [0.0; 4],
            fill_type: SdfFill::Solid as i32,
            fill_color_a: [1.0; 4],
            fill_color_b: [1.0; 4],
            fill_angle: 0.0,
            fill_offset: [0.0; 2],
            fill_scale: 1.0,
            stroke_width: 0.0,
            stroke_color: [1.0; 4],
            stroke_type: SdfStroke::None as i32,
            shadow_offset: [0.0; 2],
            shadow_radius: 0.0,
            shadow_color: [1.0; 4],
        }
    }

    pub fn hexagon(center: Vec2, radius: f32) -> SdfInstance {
        SdfInstance {
            center: [center.x, center.y, 0.0],
            dimensions: [radius, radius],
            shape_type: SdfShape::Hexagon as i32,
            corner_radius: 0.0,
            shape_params_a: [0.0; 4],
            shape_params_b: [0.0; 4],
            fill_type: SdfFill::Solid as i32,
            fill_color_a: [1.0; 4],
            fill_color_b: [1.0; 4],
            fill_angle: 0.0,
            fill_offset: [0.0; 2],
            fill_scale: 1.0,
            stroke_width: 0.0,
            stroke_color: [1.0; 4],
            stroke_type: SdfStroke::None as i32,
            shadow_offset: [0.0; 2],
            shadow_radius: 0.0,
            shadow_color: [1.0; 4],
        }
    }

    pub fn octogon(center: Vec2, radius: f32) -> SdfInstance {
        SdfInstance {
            center: [center.x, center.y, 0.0],
            dimensions: [radius, radius],
            shape_type: SdfShape::Octogon as i32,
            corner_radius: 0.0,
            shape_params_a: [0.0; 4],
            shape_params_b: [0.0; 4],
            fill_type: SdfFill::Solid as i32,
            fill_color_a: [1.0; 4],
            fill_color_b: [1.0; 4],
            fill_angle: 0.0,
            fill_offset: [0.0; 2],
            fill_scale: 1.0,
            stroke_width: 0.0,
            stroke_color: [1.0; 4],
            stroke_type: SdfStroke::None as i32,
            shadow_offset: [0.0; 2],
            shadow_radius: 0.0,
            shadow_color: [1.0; 4],
        }
    }

    pub fn hexagram(center: Vec2, radius: f32) -> SdfInstance {
        SdfInstance {
            center: [center.x, center.y, 0.0],
            dimensions: [radius, radius],
            shape_type: SdfShape::Hexagram as i32,
            corner_radius: 0.0,
            shape_params_a: [0.0; 4],
            shape_params_b: [0.0; 4],
            fill_type: SdfFill::Solid as i32,
            fill_color_a: [1.0; 4],
            fill_color_b: [1.0; 4],
            fill_angle: 0.0,
            fill_offset: [0.0; 2],
            fill_scale: 1.0,
            stroke_width: 0.0,
            stroke_color: [1.0; 4],
            stroke_type: SdfStroke::None as i32,
            shadow_offset: [0.0; 2],
            shadow_radius: 0.0,
            shadow_color: [1.0; 4],
        }
    }

    pub fn pentagram(center: Vec2, radius: f32) -> SdfInstance {
        SdfInstance {
            center: [center.x, center.y, 0.0],
            dimensions: [radius, radius],
            shape_type: SdfShape::Pentagram as i32,
            corner_radius: 0.0,
            shape_params_a: [0.0; 4],
            shape_params_b: [0.0; 4],
            fill_type: SdfFill::Solid as i32,
            fill_color_a: [1.0; 4],
            fill_color_b: [1.0; 4],
            fill_angle: 0.0,
            fill_offset: [0.0; 2],
            fill_scale: 1.0,
            stroke_width: 0.0,
            stroke_color: [1.0; 4],
            stroke_type: SdfStroke::None as i32,
            shadow_offset: [0.0; 2],
            shadow_radius: 0.0,
            shadow_color: [1.0; 4],
        }
    }

    pub fn star(center: Vec2, radius: f32, n_sides: f32, m_ratio: f32) -> SdfInstance {
        SdfInstance {
            center: [center.x, center.y, 0.0],
            dimensions: [radius, radius],
            shape_type: SdfShape::Star as i32,
            corner_radius: 0.0,
            shape_params_a: [n_sides, m_ratio, 0.0, 0.0],
            shape_params_b: [0.0; 4],
            fill_type: SdfFill::Solid as i32,
            fill_color_a: [1.0; 4],
            fill_color_b: [1.0; 4],
            fill_angle: 0.0,
            fill_offset: [0.0; 2],
            fill_scale: 1.0,
            stroke_width: 0.0,
            stroke_color: [1.0; 4],
            stroke_type: SdfStroke::None as i32,
            shadow_offset: [0.0; 2],
            shadow_radius: 0.0,
            shadow_color: [1.0; 4],
        }
    }

    pub fn moon(
        center: Vec2,
        outer_radius: f32,
        inner_offset: f32,
        inner_radius: f32,
    ) -> SdfInstance {
        SdfInstance {
            center: [center.x, center.y, 0.0],
            dimensions: [outer_radius, outer_radius],
            shape_type: SdfShape::Moon as i32,
            corner_radius: 0.0,
            shape_params_a: [inner_offset, inner_radius, 0.0, 0.0],
            shape_params_b: [0.0; 4],
            fill_type: SdfFill::Solid as i32,
            fill_color_a: [1.0; 4],
            fill_color_b: [1.0; 4],
            fill_angle: 0.0,
            fill_offset: [0.0; 2],
            fill_scale: 1.0,
            stroke_width: 0.0,
            stroke_color: [1.0; 4],
            stroke_type: SdfStroke::None as i32,
            shadow_offset: [0.0; 2],
            shadow_radius: 0.0,
            shadow_color: [1.0; 4],
        }
    }

    pub fn heart(center: Vec2, radius: f32) -> SdfInstance {
        SdfInstance {
            center: [center.x, center.y, 0.0],
            dimensions: [radius, radius],
            shape_type: SdfShape::Heart as i32,
            corner_radius: 0.0,
            shape_params_a: [0.0; 4],
            shape_params_b: [0.0; 4],
            fill_type: SdfFill::Solid as i32,
            fill_color_a: [1.0; 4],
            fill_color_b: [1.0; 4],
            fill_angle: 0.0,
            fill_offset: [0.0; 2],
            fill_scale: 1.0,
            stroke_width: 0.0,
            stroke_color: [1.0; 4],
            stroke_type: SdfStroke::None as i32,
            shadow_offset: [0.0; 2],
            shadow_radius: 0.0,
            shadow_color: [1.0; 4],
        }
    }

    pub fn cross(center: Vec2, size: Vec2) -> SdfInstance {
        SdfInstance {
            center: [center.x, center.y, 0.0],
            dimensions: [size.x * 0.5, size.y * 0.5],
            shape_type: SdfShape::Cross as i32,
            corner_radius: 0.0,
            shape_params_a: [0.0; 4],
            shape_params_b: [0.0; 4],
            fill_type: SdfFill::Solid as i32,
            fill_color_a: [1.0; 4],
            fill_color_b: [1.0; 4],
            fill_angle: 0.0,
            fill_offset: [0.0; 2],
            fill_scale: 1.0,
            stroke_width: 0.0,
            stroke_color: [1.0; 4],
            stroke_type: SdfStroke::None as i32,
            shadow_offset: [0.0; 2],
            shadow_radius: 0.0,
            shadow_color: [1.0; 4],
        }
    }

    pub fn x(center: Vec2, size: Vec2) -> SdfInstance {
        SdfInstance {
            center: [center.x, center.y, 0.0],
            dimensions: [size.x * 0.5, size.y * 0.5],
            shape_type: SdfShape::X as i32,
            corner_radius: 0.0,
            shape_params_a: [0.0; 4],
            shape_params_b: [0.0; 4],
            fill_type: SdfFill::Solid as i32,
            fill_color_a: [1.0; 4],
            fill_color_b: [1.0; 4],
            fill_angle: 0.0,
            fill_offset: [0.0; 2],
            fill_scale: 1.0,
            stroke_width: 0.0,
            stroke_color: [1.0; 4],
            stroke_type: SdfStroke::None as i32,
            shadow_offset: [0.0; 2],
            shadow_radius: 0.0,
            shadow_color: [1.0; 4],
        }
    }

    pub fn quadratic_bezier(a: Vec2, b: Vec2, c: Vec2) -> SdfInstance {
        SdfInstance {
            center: [(a.x + b.x + c.x) / 3.0, (a.y + b.y + c.y) / 3.0, 0.0],
            dimensions: [(a - c).length() * 0.5, (b - c).length() * 0.5],
            shape_type: SdfShape::QuadraticBezier as i32,
            corner_radius: 0.0,
            shape_params_a: [a.x, a.y, b.x, b.y],
            shape_params_b: [c.x, c.y, 0.0, 0.0],
            fill_type: SdfFill::Solid as i32,
            fill_color_a: [1.0; 4],
            fill_color_b: [1.0; 4],
            fill_angle: 0.0,
            fill_offset: [0.0; 2],
            fill_scale: 1.0,
            stroke_width: 0.0,
            stroke_color: [1.0; 4],
            stroke_type: SdfStroke::None as i32,
            shadow_offset: [0.0; 2],
            shadow_radius: 0.0,
            shadow_color: [1.0; 4],
        }
    }

    pub fn quadratic_circle(center: Vec2, control: Vec2, radius: f32) -> SdfInstance {
        SdfInstance {
            center: [center.x, center.y, 0.0],
            dimensions: [radius, radius],
            shape_type: SdfShape::QuadraticCircle as i32,
            corner_radius: 0.0,
            shape_params_a: [control.x, control.y, 0.0, 0.0],
            shape_params_b: [0.0; 4],
            fill_type: SdfFill::Solid as i32,
            fill_color_a: [1.0; 4],
            fill_color_b: [1.0; 4],
            fill_angle: 0.0,
            fill_offset: [0.0; 2],
            fill_scale: 1.0,
            stroke_width: 0.0,
            stroke_color: [1.0; 4],
            stroke_type: SdfStroke::None as i32,
            shadow_offset: [0.0; 2],
            shadow_radius: 0.0,
            shadow_color: [1.0; 4],
        }
    }

    pub fn with_fill_solid(mut self, color: Color) -> Self {
        self.fill_type = SdfFill::Solid as i32;
        self.fill_color_a = color.for_gpu();
        self
    }

    pub fn with_fill_gradient(
        mut self,
        color_a: Color,
        color_b: Color,
        angle: f32,
        scale: f32,
    ) -> Self {
        self.fill_type = SdfFill::Gradient as i32;
        self.fill_color_a = color_a.for_gpu();
        self.fill_color_b = color_b.for_gpu();
        self.fill_angle = angle;
        self.fill_scale = scale;
        self
    }

    pub fn with_fill_colors(mut self, color_a: Color, color_b: Color) -> Self {
        self.fill_color_a = color_a.for_gpu();
        self.fill_color_b = color_b.for_gpu();
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
        self.fill_type = fill_type as i32;
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
        self.fill_type = fill_type as i32;
        self.fill_color_a = color_a.for_gpu();
        self.fill_color_b = color_b.for_gpu();
        self.fill_angle = angle;
        self.fill_scale = scale;
        self
    }

    pub fn with_shadow(mut self, offset: Vec2, radius: f32, color: Color) -> Self {
        self.shadow_offset = offset.into();
        self.shadow_radius = radius;
        self.shadow_color = color.for_gpu();
        self
    }

    pub fn with_stroke(mut self, width: f32, color: Color, stroke_type: SdfStroke) -> Self {
        self.stroke_width = width;
        self.stroke_color = color.for_gpu();
        self.stroke_type = stroke_type as i32;
        self
    }

    pub fn with_corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }
}
