use std::{io::Cursor, marker::PhantomData};

use image::{ImageEncoder, ImageFormat};
use sge_color::u8::ColorU8;
use sge_error_union::ErrorUnion;
use sge_macros::gen_ref_type;
use sge_math::usize_rect::USizeRect;
use sge_vectors::{USizeVec2, UVec2};
use thiserror::Error;

pub use ::image as image_rs;

mod rendering;

#[derive(Clone)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub buf: Vec<ColorU8>,
}

#[derive(Debug, Error)]
pub enum SgeImageError {
    #[error("Buffer size mismatch, expected {0} bytes, found {1} bytes")]
    BufferSizeMismatch(usize, usize),
}

gen_ref_type!(Image, ImageRef, images);

impl Image {
    pub fn new(width: usize, height: usize, buf: Vec<ColorU8>) -> Self {
        Self { width, height, buf }
    }

    pub fn to_png_bytes(&self) -> Vec<u8> {
        let mut data = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut data);
        encoder
            .write_image(
                self.bytes(),
                self.width as u32,
                self.height as u32,
                image::ExtendedColorType::Rgba8,
            )
            .expect("Failed to encode image to PNG");

        data
    }

    pub fn empty(width: usize, height: usize) -> Self {
        Self::gen_color(width, height, ColorU8::TRANSPARENT)
    }

    pub fn gen_color(width: usize, height: usize, color: ColorU8) -> Self {
        let buf = vec![color; width * height];
        Self { width, height, buf }
    }

    pub fn from_bytes(width: usize, height: usize, buf: Vec<u8>) -> Result<Self, SgeImageError> {
        let len = buf.len();

        if width * height * 4 != len {
            Err(SgeImageError::BufferSizeMismatch(width * height * 4, len))
        } else {
            let size = width * height;
            let ptr = buf.as_ptr() as *const ColorU8;
            std::mem::forget(buf); // prevent free
            let buf = unsafe { Vec::from_raw_parts(ptr as *mut ColorU8, size, size) };
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

    pub fn bytes(&self) -> &[u8] {
        let ptr = self.buf.as_ptr() as *const u8;
        let len = self.buf.len() * 4;
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }

    pub fn bytes_mut(&mut self) -> &mut [u8] {
        let ptr = self.buf.as_ptr() as *mut u8;
        let len = self.buf.len() * 4;
        unsafe { std::slice::from_raw_parts_mut(ptr, len) }
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

    #[inline]
    pub fn clear(&mut self, color: ColorU8) {
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
    unsafe fn clear_simd_x86(&mut self, color: ColorU8) {
        use std::arch::x86_64::*;

        let len = self.buf.len();
        let ptr = self.buf.as_mut_ptr() as *mut u8;
        let total_bytes = len * 4;

        unsafe {
            let pixel_u32 = std::mem::transmute::<ColorU8, u32>(color);
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

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<&ColorU8> {
        if x < self.width && y < self.height {
            Some(&self.buf[y * self.width + x])
        } else {
            None
        }
    }

    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> Option<&mut ColorU8> {
        if x < self.width && y < self.height {
            Some(&mut self.buf[y * self.width + x])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, to: ColorU8) {
        if let Some(pixel) = self.get_pixel_mut(x, y) {
            *pixel = to;
        }
    }

    pub fn seti(&mut self, x: i32, y: i32, to: ColorU8) {
        if x < 0 || y < 0 {
            return;
        }

        let x = x as usize;
        let y = y as usize;

        if let Some(pixel) = self.get_pixel_mut(x, y) {
            *pixel = to;
        }
    }

    pub fn set_blend(&mut self, x: usize, y: usize, color: ColorU8) {
        if let Some(pixel) = self.get_pixel_mut(x, y) {
            *pixel = color.blend_over(*pixel);
        }
    }

    pub fn seti_blend(&mut self, x: i32, y: i32, color: ColorU8) {
        if x < 0 || y < 0 {
            return;
        }

        let x = x as usize;
        let y = y as usize;

        if let Some(pixel) = self.get_pixel_mut(x, y) {
            *pixel = color.blend_over(*pixel);
        }
    }

    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [ColorU8]> {
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
    buf: &'a [ColorU8],
}

pub struct IterMut<'a> {
    current: *mut ColorU8,
    end: *mut ColorU8,
    x: usize,
    y: usize,
    width: usize,
    _marker: PhantomData<&'a mut [ColorU8]>,
}

impl Iterator for Iter<'_> {
    type Item = (usize, usize, ColorU8);

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
    type Item = (usize, usize, &'a mut ColorU8);

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

#[derive(ErrorUnion, Debug)]
pub enum LoadImageError {
    Image(image::error::ImageError),
    Engine(SgeImageError),
    Io(std::io::Error),
    Other(&'static str),
}

pub fn load_image_sync(bytes: &[u8], format: ImageFormat) -> Result<ImageRef, LoadImageError> {
    let image = image::load(Cursor::new(bytes), format)?.to_rgba8();
    let dim = image.dimensions();
    Ok(Image::from_bytes(dim.0 as usize, dim.1 as usize, image.into_raw())?.create())
}

pub fn init() {
    init_images_storage();
}
