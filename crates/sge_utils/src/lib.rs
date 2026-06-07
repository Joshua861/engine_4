use std::{
    fmt::{Debug, Display},
    mem::MaybeUninit,
    ops::{Add, Index, IndexMut, Mul, Sub},
};

pub struct RotatingArray<T, const N: usize> {
    n: usize,
    arr: [T; N],
}

impl<T, const N: usize> RotatingArray<T, N> {
    pub fn new(arr: [T; N]) -> Self {
        Self { n: 0, arr }
    }

    #[allow(clippy::uninit_assumed_init)]
    pub fn empty() -> Self {
        Self {
            n: 0,
            arr: unsafe { MaybeUninit::uninit().assume_init() },
        }
    }

    pub fn push(&mut self, item: T) {
        self.arr[self.n] = item;
        self.n = (self.n + 1) % N;
    }

    pub fn get(&self, i: usize) -> &T {
        &self.arr[(self.n + i) % N]
    }

    pub fn get_mut(&mut self, i: usize) -> &mut T {
        &mut self.arr[(self.n + i) % N]
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        (0..N).map(move |i| self.get(i))
    }

    pub fn current_value(&self) -> &T {
        self.get(N - 1)
    }

    pub fn previous_value(&self) -> &T {
        self.get(N - 2)
    }

    pub fn past_value(&self, i: usize) -> &T {
        self.get(N - 1 - i)
    }

    pub fn len(&self) -> usize {
        N
    }

    pub fn is_empty(&self) -> bool {
        N == 0
    }
}

impl<T: ToF32 + FromF32 + Copy, const N: usize> RotatingArray<T, N> {
    pub fn avg(&self) -> T {
        let sum = self.iter().fold(0.0, |acc, &item| acc + item.to_f32());
        FromF32::from_f32(sum / N as f32)
    }
}

impl<T, const N: usize> IntoIterator for RotatingArray<T, N> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.arr.into_iter()
    }
}

impl<T, const N: usize> Index<usize> for RotatingArray<T, N> {
    type Output = T;

    fn index(&self, n: usize) -> &Self::Output {
        self.get(n)
    }
}

impl<T, const N: usize> IndexMut<usize> for RotatingArray<T, N> {
    fn index_mut(&mut self, n: usize) -> &mut Self::Output {
        self.get_mut(n)
    }
}

pub trait Lerpable:
    Add<Self, Output = Self> + Sub<Self, Output = Self> + Mul<f32, Output = Self> + Sized + Copy
{
    fn lerp(self, other: Self, progress: f32) -> Self {
        self + (other - self) * progress
    }
}

impl<T> Lerpable for T where
    T: Add<Self, Output = Self> + Sub<Self, Output = Self> + Mul<f32, Output = Self> + Sized + Copy
{
}

pub struct Lerped<T: Lerpable> {
    start: T,
    end: T,
}

impl<T: Lerpable> Lerped<T> {
    pub fn new(start: T, end: T) -> Lerped<T> {
        Lerped { start, end }
    }

    pub fn value(&self, progress: f32) -> T {
        self.start.lerp(self.end, progress)
    }

    pub fn now_offset_towards(&mut self, new_end: T) {
        self.start = self.end;
        self.end = new_end;
    }
}

pub trait ToF32 {
    fn to_f32(self) -> f32;
}

impl ToF32 for f32 {
    fn to_f32(self) -> f32 {
        self
    }
}

impl ToF32 for f64 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for i32 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for u32 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for i16 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for u16 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for i8 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for u8 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for isize {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for usize {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

pub trait FromF32 {
    fn from_f32(value: f32) -> Self;
}

impl FromF32 for f32 {
    fn from_f32(value: f32) -> Self {
        value
    }
}

impl FromF32 for f64 {
    fn from_f32(value: f32) -> Self {
        value as f64
    }
}

impl FromF32 for i32 {
    fn from_f32(value: f32) -> Self {
        value as i32
    }
}

impl FromF32 for u32 {
    fn from_f32(value: f32) -> Self {
        value as u32
    }
}

impl FromF32 for i16 {
    fn from_f32(value: f32) -> Self {
        value as i16
    }
}

impl FromF32 for u16 {
    fn from_f32(value: f32) -> Self {
        value as u16
    }
}

impl FromF32 for i8 {
    fn from_f32(value: f32) -> Self {
        value as i8
    }
}

impl FromF32 for u8 {
    fn from_f32(value: f32) -> Self {
        value as u8
    }
}

impl FromF32 for isize {
    fn from_f32(value: f32) -> Self {
        value as isize
    }
}

impl FromF32 for usize {
    fn from_f32(value: f32) -> Self {
        value as usize
    }
}

pub trait PartialClamp: PartialOrd + Sized {
    fn clamp(self, min: Self, max: Self) -> Self {
        assert!(min <= max);
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

impl<T: PartialOrd + Sized> PartialClamp for T {}

pub struct ConstantArray<T, const N: usize> {
    data: [T; N],
    len: usize,
}

impl<T: Debug, const N: usize> Debug for ConstantArray<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

#[derive(Debug)]
pub struct CapacityReached;

impl Display for CapacityReached {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Capacity reached")
    }
}

impl std::error::Error for CapacityReached {}

impl<T, const N: usize> ConstantArray<T, N> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn push(&mut self, item: T) -> Result<(), CapacityReached> {
        if self.len >= N {
            return Err(CapacityReached);
        }
        self.data[self.len] = item;
        self.len += 1;

        Ok(())
    }

    pub fn pop(&mut self) -> Option<()> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(())
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len >= N
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.as_mut_slice().iter_mut()
    }

    #[allow(clippy::uninit_assumed_init)]
    pub fn new() -> Self {
        Self {
            data: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    pub fn from_filled(data: [T; N]) -> Self {
        Self { data, len: N }
    }

    pub fn from_const_slice(slice: [T; N]) -> Self {
        Self {
            data: slice,
            len: N,
        }
    }

    pub fn from_slice(slice: &[T]) -> Result<Self, CapacityReached>
    where
        T: Copy,
    {
        if slice.len() > N {
            return Err(CapacityReached);
        }
        #[allow(clippy::uninit_assumed_init)]
        let mut data: [T; N] = unsafe { MaybeUninit::uninit().assume_init() };
        for (i, item) in slice.iter().enumerate() {
            data[i] = *item;
        }
        Ok(Self {
            data,
            len: slice.len(),
        })
    }

    pub unsafe fn data(&self) -> &[T; N] {
        &self.data
    }

    pub unsafe fn data_mut(&mut self) -> &mut [T; N] {
        &mut self.data
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data[..self.len]
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data[..self.len]
    }

    pub fn drain(&mut self) -> impl Iterator<Item = T> + '_
    where
        T: Copy,
    {
        let len = self.len;
        self.len = 0;
        self.data[..len].iter().copied()
    }

    pub fn extend_from_slice(&mut self, slice: &[T]) -> Result<(), CapacityReached>
    where
        T: Copy,
    {
        if self.len + slice.len() > N {
            return Err(CapacityReached);
        }
        for (i, item) in slice.iter().enumerate() {
            self.data[self.len + i] = *item;
        }
        self.len += slice.len();
        Ok(())
    }
}

impl<T, const N: usize> Index<usize> for ConstantArray<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for ConstantArray<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T, const N: usize> IntoIterator for ConstantArray<T, N> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T: PartialEq, const N: usize> PartialEq for ConstantArray<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: Eq, const N: usize> Eq for ConstantArray<T, N> {}

impl<T: PartialOrd, const N: usize> PartialOrd for ConstantArray<T, N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

pub trait OptionLoggingUtils {
    fn info_if_none(self, msg: &'static str) -> Self;
    fn debug_if_none(self, msg: &'static str) -> Self;
    fn warn_if_none(self, msg: &'static str) -> Self;
    fn error_if_none(self, msg: &'static str) -> Self;
}

impl<T> OptionLoggingUtils for Option<T> {
    fn debug_if_none(self, msg: &'static str) -> Self {
        if self.is_none() {
            log::debug!("{}", msg);
        }
        self
    }

    fn info_if_none(self, msg: &'static str) -> Self {
        if self.is_none() {
            log::info!("{}", msg);
        }
        self
    }

    fn warn_if_none(self, msg: &'static str) -> Self {
        if self.is_none() {
            log::warn!("{}", msg);
        }
        self
    }

    fn error_if_none(self, msg: &'static str) -> Self {
        if self.is_none() {
            log::error!("{}", msg);
        }
        self
    }
}

pub trait ResultLoggingUtils {
    fn info_if_err(self, msg: &'static str) -> Self;
    fn debug_if_err(self, msg: &'static str) -> Self;
    fn warn_if_err(self, msg: &'static str) -> Self;
    fn error_if_err(self, msg: &'static str) -> Self;
}

impl<T, E> ResultLoggingUtils for Result<T, E> {
    fn debug_if_err(self, msg: &'static str) -> Self {
        if self.is_err() {
            log::debug!("{}", msg);
        }
        self
    }

    fn info_if_err(self, msg: &'static str) -> Self {
        if self.is_err() {
            log::info!("{}", msg);
        }
        self
    }

    fn warn_if_err(self, msg: &'static str) -> Self {
        if self.is_err() {
            log::warn!("{}", msg);
        }
        self
    }

    fn error_if_err(self, msg: &'static str) -> Self {
        if self.is_err() {
            log::error!("{}", msg);
        }
        self
    }
}
