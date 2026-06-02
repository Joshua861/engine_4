use std::{fs, path::Path};

use sge_color::Color;
use sge_error_union::ErrorUnion;
use sge_vectors::{FloatExt, Vec2, Vec3, Vec4};

pub use rkyv;
pub use sge_persistence_macros::persistent;

#[derive(ErrorUnion, Debug)]
pub enum Error {
    Rkyv(rkyv::rancor::Error),
    Io(std::io::Error),
}

pub trait Persistent: Sized {
    fn to_bytes(&self) -> Result<Vec<u8>>;
    fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self>;

    fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let bytes = self.to_bytes()?;
        fs::write(path, bytes)?;
        Ok(())
    }

    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let bytes = fs::read(path)?;
        Self::from_bytes(bytes)
    }
}

pub trait PartialLerp {
    fn partial_lerp(&self, other: &Self, t: f32) -> Self;
}

pub trait Diffable {
    type Diff;

    fn diff(&self, old: &Self) -> Self::Diff;
    fn apply_diff(&mut self, diff: Self::Diff);
}

pub trait Diff {
    type Data;

    fn has_changes(&self) -> bool;
}

pub type Result<T> = std::result::Result<T, Error>;

impl PartialLerp for Vec2 {
    fn partial_lerp(&self, other: &Self, t: f32) -> Self {
        self.lerp(*other, t)
    }
}

impl PartialLerp for Vec3 {
    fn partial_lerp(&self, other: &Self, t: f32) -> Self {
        self.lerp(*other, t)
    }
}

impl PartialLerp for Vec4 {
    fn partial_lerp(&self, other: &Self, t: f32) -> Self {
        self.lerp(*other, t)
    }
}

impl PartialLerp for f32 {
    fn partial_lerp(&self, other: &Self, t: f32) -> Self {
        self.lerp(*other, t)
    }
}

impl PartialLerp for f64 {
    fn partial_lerp(&self, other: &Self, t: f32) -> Self {
        self.lerp(*other, t as f64)
    }
}

impl PartialLerp for Color {
    fn partial_lerp(&self, other: &Self, t: f32) -> Self {
        Self {
            r: self.r.lerp(other.r, t),
            g: self.g.lerp(other.g, t),
            b: self.b.lerp(other.b, t),
            a: self.a.lerp(other.a, t),
        }
    }
}
