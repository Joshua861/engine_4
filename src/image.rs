use std::marker::PhantomData;

use crate::get_state;
use bevy_math::{USizeVec2, UVec2};
use engine_4_macros::gen_ref_type;

use crate::color::u8::Pixel;
use crate::utils::usize_rect::USizeRect;

#[derive(Clone)]
pub struct Image {
    width: usize,
    height: usize,
    buf: Vec<Pixel>,
}

gen_ref_type!(Image, ImageRef, images);

impl Image {
    pub fn new(width: usize, height: usize, buf: Vec<Pixel>) -> Self {
        Self { width, height, buf }
    }

    pub fn empty(width: usize, height: usize) -> Self {
        Self::gen_color(width, height, Pixel::TRANSPARENT)
    }

    pub fn gen_color(width: usize, height: usize, color: Pixel) -> Self {
        let buf = vec![color; width * height];
        Self { width, height, buf }
    }

    pub fn from_bytes(width: usize, height: usize, buf: Vec<u8>) -> anyhow::Result<Self> {
        let len = buf.len();

        if width * height * 4 != len {
            dbg!(width, height, len);
            Err(anyhow::anyhow!("Image size mismatch"))
        } else {
            let size = width * height;
            let ptr = buf.as_ptr() as *const Pixel;
            std::mem::forget(buf); // prevent free
            let buf = unsafe { Vec::from_raw_parts(ptr as *mut Pixel, size, size) };
            Ok(Self { width, height, buf })
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let ptr = self.buf.as_ptr() as *const u8;
        let len = self.size() * 4;
        let cap = self.size() * 4;
        std::mem::forget(self.buf); // prevent free
        unsafe { Vec::from_raw_parts(ptr as *mut u8, len, cap) }
    }

    pub const fn size(&self) -> usize {
        self.width * self.height
    }

    #[inline]
    pub const fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub const fn height(&self) -> usize {
        self.height
    }

    #[inline]
    pub const fn dimensions(&self) -> USizeVec2 {
        USizeVec2::new(self.width, self.height)
    }

    #[inline]
    pub const fn dimensions_u32(&self) -> UVec2 {
        UVec2::new(self.width as u32, self.height as u32)
    }

    pub fn clear(&mut self, color: Pixel) {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            self.clear_simd_x86(color);
        }

        #[cfg(not(target_arch = "x86_64"))]
        {
            self.buf.fill(color);
        }
    }

    #[cfg(target_arch = "x86_64")]
    unsafe fn clear_simd_x86(&mut self, color: Pixel) {
        use std::arch::x86_64::*;

        let len = self.buf.len();
        let ptr = self.buf.as_mut_ptr() as *mut u8;
        let total_bytes = len * 4;

        unsafe {
            let pixel_u32 = std::mem::transmute::<Pixel, u32>(color);
            let broadcast = _mm256_set1_epi32(pixel_u32 as i32);

            let mut offset = 0;

            if is_x86_feature_detected!("avx2") {
                while offset + 32 <= total_bytes {
                    _mm256_storeu_si256(ptr.add(offset) as *mut __m256i, broadcast);
                    offset += 32;
                }
            } else if is_x86_feature_detected!("sse2") {
                let broadcast_sse = _mm_set1_epi32(pixel_u32 as i32);
                while offset + 16 <= total_bytes {
                    _mm_storeu_si128(ptr.add(offset) as *mut __m128i, broadcast_sse);
                    offset += 16;
                }
            }

            let pixels_done = offset / 4;
            for i in pixels_done..len {
                self.buf[i] = color;
            }
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<&Pixel> {
        if x < self.width && y < self.height {
            Some(&self.buf[y * self.width + x])
        } else {
            None
        }
    }

    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        if x < self.width && y < self.height {
            Some(&mut self.buf[y * self.width + x])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, to: Pixel) {
        if let Some(pixel) = self.get_pixel_mut(x, y) {
            *pixel = to;
        }
    }

    pub fn seti(&mut self, x: i32, y: i32, to: Pixel) {
        if x < 0 || y < 0 {
            return;
        }

        let x = x as usize;
        let y = y as usize;

        if let Some(pixel) = self.get_pixel_mut(x, y) {
            *pixel = to;
        }
    }

    pub fn set_blend(&mut self, x: usize, y: usize, color: Pixel) {
        if let Some(pixel) = self.get_pixel_mut(x, y) {
            *pixel = color.blend_over(*pixel);
        }
    }

    pub fn seti_blend(&mut self, x: i32, y: i32, color: Pixel) {
        if x < 0 || y < 0 {
            return;
        }

        let x = x as usize;
        let y = y as usize;

        if let Some(pixel) = self.get_pixel_mut(x, y) {
            *pixel = color.blend_over(*pixel);
        }
    }

    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [Pixel]> {
        self.buf.chunks_exact_mut(self.width)
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter {
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
            buf: &self.buf,
        }
    }

    /// # Safety
    /// Implementation of the IterMut struct contains raw pointers to the origional image.
    /// If that image is moved or modified while this iter_mut exists, UB could occur probably.
    pub unsafe fn iter_mut(&mut self) -> IterMut<'_> {
        let ptr = self.buf.as_mut_ptr();

        IterMut {
            current: ptr,
            end: unsafe { ptr.add(self.buf.len()) },
            x: 0,
            y: 0,
            width: self.width,
            _marker: PhantomData,
        }
    }

    pub fn sub_image(&self, rect: USizeRect) -> Image {
        let mut buf = Vec::with_capacity(rect.width() * rect.height());

        for y in rect.min.y..rect.min.y + rect.height() {
            for x in rect.min.x..rect.min.x + rect.width() {
                buf.push(*self.get_pixel(x, y).unwrap());
            }
        }

        Self {
            width: rect.width(),
            height: rect.height(),
            buf,
        }
    }
}

pub struct Iter<'a> {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    buf: &'a [Pixel],
}

pub struct IterMut<'a> {
    current: *mut Pixel,
    end: *mut Pixel,
    x: usize,
    y: usize,
    width: usize,
    _marker: PhantomData<&'a mut [Pixel]>,
}

impl Iterator for Iter<'_> {
    type Item = (usize, usize, Pixel);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.height {
            return None;
        }

        let pixel = self.buf[self.y * self.width + self.x];
        let item = (self.x, self.y, pixel);

        self.x += 1;
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }

        Some(item)
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = (usize, usize, &'a mut Pixel);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.end {
            return None;
        }

        let pixel = unsafe { &mut *self.current };
        let item = (self.x, self.y, pixel);

        self.current = unsafe { self.current.add(1) };
        self.x += 1;

        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }

        Some(item)
    }
}
