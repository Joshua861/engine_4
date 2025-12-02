use std::f32::consts::PI;

use bevy_math::{FloatExt, Quat, Vec2, Vec3, Vec4};

use crate::{
    collisions::AABB2D,
    color::Color,
    get_state,
    prelude::{AABB3D, Circle, Transform2D, Transform3D},
    shapes_2d::Rect,
};

pub trait Animatable {
    /// progress ranges from 0 (animation start) to 1 (animation end)
    fn lerp(a: Self, b: Self, progress: f32) -> Self;
}

pub trait EasingFunction {
    /// t ranges from 0 (animation start) to 1 (animation end)
    fn progress(&self, t: f32) -> f32;
}

pub struct AnimationController<T: Animatable, E: EasingFunction> {
    start: T,
    end: T,
    easing_function: E,
    start_time: f32,
    length: f32,
}

impl<T: Animatable + Copy, E: EasingFunction> AnimationController<T, E> {
    /// Length is in seconds
    pub fn new(start: T, end: T, length: f32, easing_function: E) -> Self {
        let state = get_state();
        Self {
            start,
            end,
            easing_function,
            start_time: state.time,
            length,
        }
    }

    pub fn time_elapsed(&self) -> f32 {
        let state = get_state();
        state.time - self.start_time
    }

    pub fn value(&self) -> T {
        let progress = self
            .easing_function
            .progress(self.time_elapsed() / self.length);
        T::lerp(self.start, self.end, progress)
    }

    pub fn is_complete(&self) -> bool {
        self.time_elapsed() >= self.length
    }

    pub fn now_animate_towards(&mut self, new_end: T) {
        let state = get_state();
        self.start = self.end;
        self.end = new_end;
        self.start_time = state.time;
    }
}

pub fn lerp(a: f32, b: f32, progress: f32) -> f32 {
    a + progress * (b - a)
}

impl Animatable for f32 {
    fn lerp(a: Self, b: Self, progress: f32) -> Self {
        lerp(a, b, progress)
    }
}

impl Animatable for f64 {
    fn lerp(a: Self, b: Self, progress: f32) -> Self {
        a.lerp(b, progress as f64)
    }
}

impl Animatable for u8 {
    fn lerp(a: Self, b: Self, progress: f32) -> Self {
        (a as f32).lerp(b as f32, progress) as u8
    }
}

impl Animatable for u16 {
    fn lerp(a: Self, b: Self, progress: f32) -> Self {
        (a as f32).lerp(b as f32, progress) as u16
    }
}

impl Animatable for u32 {
    fn lerp(a: Self, b: Self, progress: f32) -> Self {
        (a as f32).lerp(b as f32, progress) as u32
    }
}

impl Animatable for u64 {
    fn lerp(a: Self, b: Self, progress: f32) -> Self {
        (a as f32).lerp(b as f32, progress) as u64
    }
}

impl Animatable for usize {
    fn lerp(a: Self, b: Self, progress: f32) -> Self {
        (a as f32).lerp(b as f32, progress) as usize
    }
}

impl Animatable for Vec2 {
    fn lerp(a: Self, b: Self, progress: f32) -> Self {
        a.lerp(b, progress)
    }
}

impl Animatable for Vec3 {
    fn lerp(a: Self, b: Self, progress: f32) -> Self {
        a.lerp(b, progress)
    }
}

impl Animatable for Vec4 {
    fn lerp(a: Self, b: Self, progress: f32) -> Self {
        a.lerp(b, progress)
    }
}

impl Animatable for Color {
    fn lerp(a: Self, b: Self, progress: f32) -> Self {
        Color::from_vec4(a.to_vec4().lerp(b.to_vec4(), progress))
    }
}

impl Animatable for Transform2D {
    fn lerp(a: Self, b: Self, progress: f32) -> Self {
        let translation = a.translation().lerp(b.translation(), progress);
        let rotation = a.rotation().lerp(b.rotation(), progress);
        let scale = a.scale().lerp(b.scale(), progress);
        Transform2D::from_scale_rotation_translation(scale, rotation, translation)
    }
}

impl Animatable for Transform3D {
    fn lerp(a: Self, b: Self, progress: f32) -> Self {
        let translation = a.translation().lerp(b.translation(), progress);
        let rotation = a.rotation().slerp(b.rotation(), progress);
        let scale = a.scale().lerp(b.scale(), progress);
        Transform3D::from_scale_rotation_translation(scale, rotation, translation)
    }
}

/// # Panics
///
/// Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.
impl Animatable for Quat {
    fn lerp(a: Self, b: Self, progress: f32) -> Self {
        a.slerp(b, progress)
    }
}

impl Animatable for Circle {
    fn lerp(a: Self, b: Self, progress: f32) -> Self {
        Self {
            center: Vec2::lerp(a.center, b.center, progress),
            radius: Vec2::lerp(a.radius, b.radius, progress),
            color: Color::lerp(a.color, b.color, progress),
        }
    }
}

impl Animatable for crate::collisions::Circle {
    fn lerp(a: Self, b: Self, progress: f32) -> Self {
        Self {
            center: Vec2::lerp(a.center, b.center, progress),
            radius: a.radius.lerp(b.radius, progress),
        }
    }
}

impl Animatable for Rect {
    fn lerp(a: Self, b: Self, progress: f32) -> Self {
        Self {
            top_left: Vec2::lerp(a.top_left, b.top_left, progress),
            size: Vec2::lerp(a.size, b.size, progress),
            color: Color::lerp(a.color, b.color, progress),
        }
    }
}

impl Animatable for crate::collisions::Square {
    fn lerp(a: Self, b: Self, progress: f32) -> Self {
        Self {
            center: a.center.lerp(b.center, progress),
            half_size: a.half_size.lerp(b.half_size, progress),
        }
    }
}

impl Animatable for bevy_math::Rect {
    fn lerp(a: Self, b: Self, progress: f32) -> Self {
        Self::from_corners(a.min.lerp(b.min, progress), a.max.lerp(b.max, progress))
    }
}

impl Animatable for AABB2D {
    fn lerp(a: Self, b: Self, progress: f32) -> Self {
        Self {
            min: Vec2::lerp(a.min, b.min, progress),
            max: Vec2::lerp(a.max, b.max, progress),
        }
    }
}

impl Animatable for AABB3D {
    fn lerp(a: Self, b: Self, progress: f32) -> Self {
        Self {
            min: Vec3::lerp(a.min, b.min, progress),
            max: Vec3::lerp(a.max, b.max, progress),
        }
    }
}

impl<T: Animatable + Copy, const N: usize> Animatable for [T; N] {
    fn lerp(a: Self, b: Self, progress: f32) -> Self {
        std::array::from_fn(|i| T::lerp(a[i], b[i], progress))
    }
}

impl<T: Animatable + Default> Animatable for Option<T> {
    fn lerp(a: Self, b: Self, progress: f32) -> Self {
        match (a, b) {
            (Some(a), Some(b)) => Some(T::lerp(a, b, progress)),
            (None, Some(b)) => Some(T::lerp(T::default(), b, progress)),
            (Some(a), None) => Some(T::lerp(a, T::default(), progress)),
            (None, None) => None,
        }
    }
}

pub struct LinearEasingFunction;
impl EasingFunction for LinearEasingFunction {
    fn progress(&self, t: f32) -> f32 {
        t
    }
}

pub struct EaseInSine;
impl EasingFunction for EaseInSine {
    fn progress(&self, t: f32) -> f32 {
        1.0 - ((t * PI) / 2.0).cos()
    }
}

pub struct EaseOutSine;
impl EasingFunction for EaseOutSine {
    fn progress(&self, t: f32) -> f32 {
        ((t * PI) / 2.0).sin()
    }
}

pub struct EaseInOutSine;
impl EasingFunction for EaseInOutSine {
    fn progress(&self, t: f32) -> f32 {
        -((t * PI).cos() - 1.0) / 2.0
    }
}

pub struct EaseInQuad;
impl EasingFunction for EaseInQuad {
    fn progress(&self, t: f32) -> f32 {
        t * t
    }
}

pub struct EaseOutQuad;
impl EasingFunction for EaseOutQuad {
    fn progress(&self, t: f32) -> f32 {
        1.0 - (1.0 - t) * (1.0 - t)
    }
}

pub struct EaseInOutQuad;
impl EasingFunction for EaseInOutQuad {
    fn progress(&self, t: f32) -> f32 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
        }
    }
}

pub struct EaseInCubic;
impl EasingFunction for EaseInCubic {
    fn progress(&self, t: f32) -> f32 {
        t * t * t
    }
}

pub struct EaseOutCubic;
impl EasingFunction for EaseOutCubic {
    fn progress(&self, t: f32) -> f32 {
        1.0 - (1.0 - t).powi(3)
    }
}

pub struct EaseInOutCubic;
impl EasingFunction for EaseInOutCubic {
    fn progress(&self, t: f32) -> f32 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
        }
    }
}

pub struct EaseInQuart;
impl EasingFunction for EaseInQuart {
    fn progress(&self, t: f32) -> f32 {
        t * t * t * t
    }
}

pub struct EaseOutQuart;
impl EasingFunction for EaseOutQuart {
    fn progress(&self, t: f32) -> f32 {
        1.0 - (1.0 - t).powi(4)
    }
}

pub struct EaseInOutQuart;
impl EasingFunction for EaseInOutQuart {
    fn progress(&self, t: f32) -> f32 {
        if t < 0.5 {
            8.0 * t * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
        }
    }
}

pub struct EaseInQuint;
impl EasingFunction for EaseInQuint {
    fn progress(&self, t: f32) -> f32 {
        t * t * t * t * t
    }
}

pub struct EaseOutQuint;
impl EasingFunction for EaseOutQuint {
    fn progress(&self, t: f32) -> f32 {
        1.0 - (1.0 - t).powi(5)
    }
}

pub struct EaseInOutQuint;
impl EasingFunction for EaseInOutQuint {
    fn progress(&self, t: f32) -> f32 {
        if t < 0.5 {
            16.0 * t * t * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(5) / 2.0
        }
    }
}

pub struct EaseInExpo;
impl EasingFunction for EaseInExpo {
    fn progress(&self, t: f32) -> f32 {
        if t == 0.0 {
            0.0
        } else {
            2.0f32.powf(10.0 * t - 10.0)
        }
    }
}

pub struct EaseOutExpo;
impl EasingFunction for EaseOutExpo {
    fn progress(&self, t: f32) -> f32 {
        if t == 1.0 {
            1.0
        } else {
            1.0 - 2.0f32.powf(-10.0 * t)
        }
    }
}

pub struct EaseInOutExpo;
impl EasingFunction for EaseInOutExpo {
    fn progress(&self, t: f32) -> f32 {
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else if t < 0.5 {
            2.0f32.powf(20.0 * t - 10.0) / 2.0
        } else {
            (2.0 - 2.0f32.powf(-20.0 * t + 10.0)) / 2.0
        }
    }
}

pub struct EaseInCirc;
impl EasingFunction for EaseInCirc {
    fn progress(&self, t: f32) -> f32 {
        1.0 - (1.0 - t * t).sqrt()
    }
}

pub struct EaseOutCirc;
impl EasingFunction for EaseOutCirc {
    fn progress(&self, t: f32) -> f32 {
        (1.0 - (t - 1.0).powi(2)).sqrt()
    }
}

pub struct EaseInOutCirc;
impl EasingFunction for EaseInOutCirc {
    fn progress(&self, t: f32) -> f32 {
        if t < 0.5 {
            (1.0 - (1.0 - (2.0 * t).powi(2)).sqrt()) / 2.0
        } else {
            ((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) / 2.0
        }
    }
}

const C1: f32 = 1.70158;
const C2: f32 = C1 * 1.525;
const C3: f32 = C1 + 1.0;

pub struct EaseInBack;
impl EasingFunction for EaseInBack {
    fn progress(&self, t: f32) -> f32 {
        C3 * t * t * t - C1 * t * t
    }
}

pub struct EaseOutBack;
impl EasingFunction for EaseOutBack {
    fn progress(&self, t: f32) -> f32 {
        1.0 + C3 * (t - 1.0).powi(3) + C1 * (t - 1.0).powi(2)
    }
}

pub struct EaseInOutBack;
impl EasingFunction for EaseInOutBack {
    fn progress(&self, t: f32) -> f32 {
        if t < 0.5 {
            ((2.0 * t).powi(2) * ((C2 + 1.0) * 2.0 * t - C2)) / 2.0
        } else {
            ((2.0 * t - 2.0).powi(2) * ((C2 + 1.0) * (t * 2.0 - 2.0) + C2) + 2.0) / 2.0
        }
    }
}

const C4: f32 = (2.0 * PI) / 3.0;
const C5: f32 = (2.0 * PI) / 4.5;

pub struct EaseInElastic;
impl EasingFunction for EaseInElastic {
    fn progress(&self, t: f32) -> f32 {
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else {
            -2.0f32.powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * C4).sin()
        }
    }
}

pub struct EaseOutElastic;
impl EasingFunction for EaseOutElastic {
    fn progress(&self, t: f32) -> f32 {
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else {
            2.0f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * C4).sin() + 1.0
        }
    }
}

pub struct EaseInOutElastic;
impl EasingFunction for EaseInOutElastic {
    fn progress(&self, t: f32) -> f32 {
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else if t < 0.5 {
            -(2.0f32.powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * C5).sin()) / 2.0
        } else {
            (2.0f32.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * C5).sin()) / 2.0 + 1.0
        }
    }
}

fn bounce_out(t: f32) -> f32 {
    const N1: f32 = 7.5625;
    const D1: f32 = 2.75;

    if t < 1.0 / D1 {
        N1 * t * t
    } else if t < 2.0 / D1 {
        N1 * (t - 1.5 / D1) * (t - 1.5 / D1) + 0.75
    } else if t < 2.5 / D1 {
        N1 * (t - 2.25 / D1) * (t - 2.25 / D1) + 0.9375
    } else {
        N1 * (t - 2.625 / D1) * (t - 2.625 / D1) + 0.984375
    }
}

pub struct EaseInBounce;
impl EasingFunction for EaseInBounce {
    fn progress(&self, t: f32) -> f32 {
        1.0 - bounce_out(1.0 - t)
    }
}

pub struct EaseOutBounce;
impl EasingFunction for EaseOutBounce {
    fn progress(&self, t: f32) -> f32 {
        bounce_out(t)
    }
}

pub struct EaseInOutBounce;
impl EasingFunction for EaseInOutBounce {
    fn progress(&self, t: f32) -> f32 {
        if t < 0.5 {
            (1.0 - bounce_out(1.0 - 2.0 * t)) / 2.0
        } else {
            (1.0 + bounce_out(2.0 * t - 1.0)) / 2.0
        }
    }
}
