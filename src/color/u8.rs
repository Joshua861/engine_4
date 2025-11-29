use std::{fmt::Debug, hash::Hash};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[repr(C)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone, Copy)]
pub union Pixel {
    rgba: Rgba,
    raw: [u8; 4],
}

unsafe impl Send for Pixel {}
unsafe impl Sync for Pixel {}

impl Pixel {
    pub fn splat_f32(v: f32) -> Self {
        Self::from_rgba_f32(v, v, v, 1.0)
    }

    pub const fn r(&self) -> u8 {
        unsafe { self.rgba.r }
    }

    pub const fn g(&self) -> u8 {
        unsafe { self.rgba.g }
    }

    pub const fn b(&self) -> u8 {
        unsafe { self.rgba.b }
    }

    pub const fn a(&self) -> u8 {
        unsafe { self.rgba.a }
    }

    pub const fn r_f32(&self) -> f32 {
        self.r() as f32 / 256.0
    }

    pub const fn g_f32(&self) -> f32 {
        self.g() as f32 / 256.0
    }

    pub const fn b_f32(&self) -> f32 {
        self.b() as f32 / 256.0
    }

    pub const fn a_f32(&self) -> f32 {
        self.a() as f32 / 256.0
    }

    pub const fn rgba(&self) -> Rgba {
        // SAFETY: repr of Rgba is identical to [u8; 4]
        unsafe { self.rgba }
    }

    pub const fn raw(&self) -> [u8; 4] {
        // SAFETY: repr of Rgba is identical to [u8; 4]
        unsafe { self.raw }
    }

    pub const fn rgba_mut(&mut self) -> &mut Rgba {
        // SAFETY: repr of Rgba is identical to [u8; 4]
        unsafe { &mut self.rgba }
    }

    pub const fn raw_mut(&mut self) -> &mut [u8; 4] {
        // SAFETY: repr of Rgba is identical to [u8; 4]
        unsafe { &mut self.raw }
    }

    pub const fn from_rgba_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            raw: [
                Self::f32_to_u8(r),
                Self::f32_to_u8(g),
                Self::f32_to_u8(b),
                Self::f32_to_u8(a),
            ],
        }
    }

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            rgba: Rgba { r, g, b, a: 255 },
        }
    }

    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            rgba: Rgba { r, g, b, a },
        }
    }

    const fn f32_to_u8(n: f32) -> u8 {
        (n * 255.999) as u8
    }

    pub const fn with_alpha(mut self, a: u8) -> Self {
        self.rgba.a = a;
        self
    }

    pub fn blend_over(self, background: Pixel) -> Pixel {
        let src_a = self.a() as u32;
        let dst_a = background.a() as u32;

        if src_a == 255 {
            return self;
        }

        if src_a == 0 {
            return background;
        }

        if dst_a == 0 {
            return self;
        }

        if background == Pixel::BLACK {
            return self;
        }

        let inv_src_a = 255 - src_a;

        let r = ((self.r() as u32 * src_a + background.r() as u32 * inv_src_a) / 255) as u8;
        let g = ((self.g() as u32 * src_a + background.g() as u32 * inv_src_a) / 255) as u8;
        let b = ((self.b() as u32 * src_a + background.b() as u32 * inv_src_a) / 255) as u8;

        let a = (src_a + (dst_a * inv_src_a) / 255) as u8;

        Pixel::from_rgba(r, g, b, a)
    }

    /// Ignores and erases alpha
    pub fn mix_two(a: Pixel, b: Pixel, fac: u8) -> Pixel {
        let fac = fac as u32;
        let inv_fac = 255 - fac;

        let r = ((a.r() as u32 * fac + b.r() as u32 * inv_fac) / 255) as u8;
        let g = ((a.g() as u32 * fac + b.g() as u32 * inv_fac) / 255) as u8;
        let b = ((a.b() as u32 * fac + b.b() as u32 * inv_fac) / 255) as u8;

        Pixel::from_rgb(r, g, b)
    }

    /// Ignores and erases alpha
    pub fn mix_two_f32(a: Pixel, b: Pixel, fac: f32) -> Pixel {
        let fac = (fac * 255.999) as u8;
        Self::mix_two(a, b, fac)
    }

    /// Ignores and erases alpha
    pub fn mix_with(self, b: Pixel, fac: u8) -> Pixel {
        Self::mix_two(self, b, fac)
    }

    /// Ignores and erases alpha
    pub fn mix_with_f32(self, b: Pixel, fac: f32) -> Pixel {
        Self::mix_two_f32(self, b, fac)
    }

    pub const fn splat(v: u8) -> Self {
        Self::from_rgb(v, v, v)
    }

    pub const fn splat_a(v: u8) -> Self {
        Self::from_rgba(v, v, v, v)
    }

    pub const WHITE: Pixel = Pixel::splat(255);
    pub const BLACK: Pixel = Pixel::splat(0);
    pub const TRANSPARENT: Pixel = Pixel::splat_a(0);

    pub const SLATE_50: Pixel = Pixel::from_rgb(248, 250, 252);
    pub const SLATE_100: Pixel = Pixel::from_rgb(241, 245, 249);
    pub const SLATE_200: Pixel = Pixel::from_rgb(226, 232, 240);
    pub const SLATE_300: Pixel = Pixel::from_rgb(202, 213, 226);
    pub const SLATE_400: Pixel = Pixel::from_rgb(144, 161, 185);
    pub const SLATE_500: Pixel = Pixel::from_rgb(98, 116, 142);
    pub const SLATE_600: Pixel = Pixel::from_rgb(69, 85, 108);
    pub const SLATE_700: Pixel = Pixel::from_rgb(49, 65, 88);
    pub const SLATE_800: Pixel = Pixel::from_rgb(29, 41, 61);
    pub const SLATE_900: Pixel = Pixel::from_rgb(15, 23, 43);
    pub const SLATE_950: Pixel = Pixel::from_rgb(2, 6, 24);
    pub const GRAY_50: Pixel = Pixel::from_rgb(249, 250, 251);
    pub const GRAY_100: Pixel = Pixel::from_rgb(243, 244, 246);
    pub const GRAY_200: Pixel = Pixel::from_rgb(229, 231, 235);
    pub const GRAY_300: Pixel = Pixel::from_rgb(209, 213, 220);
    pub const GRAY_400: Pixel = Pixel::from_rgb(153, 161, 175);
    pub const GRAY_500: Pixel = Pixel::from_rgb(106, 114, 130);
    pub const GRAY_600: Pixel = Pixel::from_rgb(74, 85, 101);
    pub const GRAY_700: Pixel = Pixel::from_rgb(54, 65, 83);
    pub const GRAY_800: Pixel = Pixel::from_rgb(30, 41, 57);
    pub const GRAY_900: Pixel = Pixel::from_rgb(16, 24, 40);
    pub const GRAY_950: Pixel = Pixel::from_rgb(3, 7, 18);
    pub const ZINC_50: Pixel = Pixel::from_rgb(250, 250, 250);
    pub const ZINC_100: Pixel = Pixel::from_rgb(244, 244, 245);
    pub const ZINC_200: Pixel = Pixel::from_rgb(228, 228, 231);
    pub const ZINC_300: Pixel = Pixel::from_rgb(212, 212, 216);
    pub const ZINC_400: Pixel = Pixel::from_rgb(159, 159, 169);
    pub const ZINC_500: Pixel = Pixel::from_rgb(113, 113, 123);
    pub const ZINC_600: Pixel = Pixel::from_rgb(82, 82, 92);
    pub const ZINC_700: Pixel = Pixel::from_rgb(63, 63, 70);
    pub const ZINC_800: Pixel = Pixel::from_rgb(39, 39, 42);
    pub const ZINC_900: Pixel = Pixel::from_rgb(24, 24, 27);
    pub const ZINC_950: Pixel = Pixel::from_rgb(9, 9, 11);
    pub const NEUTRAL_50: Pixel = Pixel::from_rgb(250, 250, 250);
    pub const NEUTRAL_100: Pixel = Pixel::from_rgb(245, 245, 245);
    pub const NEUTRAL_200: Pixel = Pixel::from_rgb(229, 229, 229);
    pub const NEUTRAL_300: Pixel = Pixel::from_rgb(212, 212, 212);
    pub const NEUTRAL_400: Pixel = Pixel::from_rgb(161, 161, 161);
    pub const NEUTRAL_500: Pixel = Pixel::from_rgb(115, 115, 115);
    pub const NEUTRAL_600: Pixel = Pixel::from_rgb(82, 82, 82);
    pub const NEUTRAL_700: Pixel = Pixel::from_rgb(64, 64, 64);
    pub const NEUTRAL_800: Pixel = Pixel::from_rgb(38, 38, 38);
    pub const NEUTRAL_900: Pixel = Pixel::from_rgb(23, 23, 23);
    pub const NEUTRAL_950: Pixel = Pixel::from_rgb(10, 10, 10);
    pub const STONE_50: Pixel = Pixel::from_rgb(250, 250, 249);
    pub const STONE_100: Pixel = Pixel::from_rgb(245, 245, 244);
    pub const STONE_200: Pixel = Pixel::from_rgb(231, 229, 228);
    pub const STONE_300: Pixel = Pixel::from_rgb(214, 211, 209);
    pub const STONE_400: Pixel = Pixel::from_rgb(166, 160, 155);
    pub const STONE_500: Pixel = Pixel::from_rgb(121, 113, 107);
    pub const STONE_600: Pixel = Pixel::from_rgb(87, 83, 77);
    pub const STONE_700: Pixel = Pixel::from_rgb(68, 64, 59);
    pub const STONE_800: Pixel = Pixel::from_rgb(41, 37, 36);
    pub const STONE_900: Pixel = Pixel::from_rgb(28, 25, 23);
    pub const STONE_950: Pixel = Pixel::from_rgb(12, 10, 9);
    pub const RED_50: Pixel = Pixel::from_rgb(254, 242, 242);
    pub const RED_100: Pixel = Pixel::from_rgb(255, 226, 226);
    pub const RED_200: Pixel = Pixel::from_rgb(255, 201, 201);
    pub const RED_300: Pixel = Pixel::from_rgb(255, 162, 162);
    pub const RED_400: Pixel = Pixel::from_rgb(255, 100, 103);
    pub const RED_500: Pixel = Pixel::from_rgb(251, 44, 54);
    pub const RED_600: Pixel = Pixel::from_rgb(231, 0, 11);
    pub const RED_700: Pixel = Pixel::from_rgb(193, 0, 7);
    pub const RED_800: Pixel = Pixel::from_rgb(159, 7, 18);
    pub const RED_900: Pixel = Pixel::from_rgb(130, 24, 26);
    pub const RED_950: Pixel = Pixel::from_rgb(70, 8, 9);
    pub const ORANGE_50: Pixel = Pixel::from_rgb(255, 247, 237);
    pub const ORANGE_100: Pixel = Pixel::from_rgb(255, 237, 212);
    pub const ORANGE_200: Pixel = Pixel::from_rgb(255, 214, 167);
    pub const ORANGE_300: Pixel = Pixel::from_rgb(255, 184, 106);
    pub const ORANGE_400: Pixel = Pixel::from_rgb(255, 137, 4);
    pub const ORANGE_500: Pixel = Pixel::from_rgb(255, 105, 0);
    pub const ORANGE_600: Pixel = Pixel::from_rgb(245, 73, 0);
    pub const ORANGE_700: Pixel = Pixel::from_rgb(202, 53, 0);
    pub const ORANGE_800: Pixel = Pixel::from_rgb(159, 45, 0);
    pub const ORANGE_900: Pixel = Pixel::from_rgb(126, 42, 12);
    pub const ORANGE_950: Pixel = Pixel::from_rgb(68, 19, 6);
    pub const AMBER_50: Pixel = Pixel::from_rgb(255, 251, 235);
    pub const AMBER_100: Pixel = Pixel::from_rgb(254, 243, 198);
    pub const AMBER_200: Pixel = Pixel::from_rgb(254, 230, 133);
    pub const AMBER_300: Pixel = Pixel::from_rgb(255, 210, 48);
    pub const AMBER_400: Pixel = Pixel::from_rgb(255, 185, 0);
    pub const AMBER_500: Pixel = Pixel::from_rgb(254, 154, 0);
    pub const AMBER_600: Pixel = Pixel::from_rgb(225, 113, 0);
    pub const AMBER_700: Pixel = Pixel::from_rgb(187, 77, 0);
    pub const AMBER_800: Pixel = Pixel::from_rgb(151, 60, 0);
    pub const AMBER_900: Pixel = Pixel::from_rgb(123, 51, 6);
    pub const AMBER_950: Pixel = Pixel::from_rgb(70, 25, 1);
    pub const YELLOW_50: Pixel = Pixel::from_rgb(254, 252, 232);
    pub const YELLOW_100: Pixel = Pixel::from_rgb(254, 249, 194);
    pub const YELLOW_200: Pixel = Pixel::from_rgb(255, 240, 133);
    pub const YELLOW_300: Pixel = Pixel::from_rgb(255, 223, 32);
    pub const YELLOW_400: Pixel = Pixel::from_rgb(253, 199, 0);
    pub const YELLOW_500: Pixel = Pixel::from_rgb(240, 177, 0);
    pub const YELLOW_600: Pixel = Pixel::from_rgb(208, 135, 0);
    pub const YELLOW_700: Pixel = Pixel::from_rgb(166, 95, 0);
    pub const YELLOW_800: Pixel = Pixel::from_rgb(137, 75, 0);
    pub const YELLOW_900: Pixel = Pixel::from_rgb(115, 62, 10);
    pub const YELLOW_950: Pixel = Pixel::from_rgb(67, 32, 4);
    pub const LIME_50: Pixel = Pixel::from_rgb(247, 254, 231);
    pub const LIME_100: Pixel = Pixel::from_rgb(236, 252, 202);
    pub const LIME_200: Pixel = Pixel::from_rgb(216, 249, 153);
    pub const LIME_300: Pixel = Pixel::from_rgb(187, 244, 81);
    pub const LIME_400: Pixel = Pixel::from_rgb(154, 230, 0);
    pub const LIME_500: Pixel = Pixel::from_rgb(124, 207, 0);
    pub const LIME_600: Pixel = Pixel::from_rgb(94, 165, 0);
    pub const LIME_700: Pixel = Pixel::from_rgb(73, 125, 0);
    pub const LIME_800: Pixel = Pixel::from_rgb(60, 99, 0);
    pub const LIME_900: Pixel = Pixel::from_rgb(53, 83, 14);
    pub const LIME_950: Pixel = Pixel::from_rgb(25, 46, 3);
    pub const GREEN_50: Pixel = Pixel::from_rgb(240, 253, 244);
    pub const GREEN_100: Pixel = Pixel::from_rgb(220, 252, 231);
    pub const GREEN_200: Pixel = Pixel::from_rgb(185, 248, 207);
    pub const GREEN_300: Pixel = Pixel::from_rgb(123, 241, 168);
    pub const GREEN_400: Pixel = Pixel::from_rgb(5, 223, 114);
    pub const GREEN_500: Pixel = Pixel::from_rgb(0, 201, 80);
    pub const GREEN_600: Pixel = Pixel::from_rgb(0, 166, 62);
    pub const GREEN_700: Pixel = Pixel::from_rgb(0, 130, 54);
    pub const GREEN_800: Pixel = Pixel::from_rgb(1, 102, 48);
    pub const GREEN_900: Pixel = Pixel::from_rgb(13, 84, 43);
    pub const GREEN_950: Pixel = Pixel::from_rgb(3, 46, 21);
    pub const EMERALD_50: Pixel = Pixel::from_rgb(236, 253, 245);
    pub const EMERALD_100: Pixel = Pixel::from_rgb(208, 250, 229);
    pub const EMERALD_200: Pixel = Pixel::from_rgb(164, 244, 207);
    pub const EMERALD_300: Pixel = Pixel::from_rgb(94, 233, 181);
    pub const EMERALD_400: Pixel = Pixel::from_rgb(0, 212, 146);
    pub const EMERALD_500: Pixel = Pixel::from_rgb(0, 188, 125);
    pub const EMERALD_600: Pixel = Pixel::from_rgb(0, 153, 102);
    pub const EMERALD_700: Pixel = Pixel::from_rgb(0, 122, 85);
    pub const EMERALD_800: Pixel = Pixel::from_rgb(0, 96, 69);
    pub const EMERALD_900: Pixel = Pixel::from_rgb(0, 79, 59);
    pub const EMERALD_950: Pixel = Pixel::from_rgb(0, 44, 34);
    pub const TEAL_50: Pixel = Pixel::from_rgb(240, 253, 250);
    pub const TEAL_100: Pixel = Pixel::from_rgb(203, 251, 241);
    pub const TEAL_200: Pixel = Pixel::from_rgb(150, 247, 228);
    pub const TEAL_300: Pixel = Pixel::from_rgb(70, 236, 213);
    pub const TEAL_400: Pixel = Pixel::from_rgb(0, 213, 190);
    pub const TEAL_500: Pixel = Pixel::from_rgb(0, 187, 167);
    pub const TEAL_600: Pixel = Pixel::from_rgb(0, 150, 137);
    pub const TEAL_700: Pixel = Pixel::from_rgb(0, 120, 111);
    pub const TEAL_800: Pixel = Pixel::from_rgb(0, 95, 90);
    pub const TEAL_900: Pixel = Pixel::from_rgb(11, 79, 74);
    pub const TEAL_950: Pixel = Pixel::from_rgb(2, 47, 46);
    pub const CYAN_50: Pixel = Pixel::from_rgb(236, 254, 255);
    pub const CYAN_100: Pixel = Pixel::from_rgb(206, 250, 254);
    pub const CYAN_200: Pixel = Pixel::from_rgb(162, 244, 253);
    pub const CYAN_300: Pixel = Pixel::from_rgb(83, 234, 253);
    pub const CYAN_400: Pixel = Pixel::from_rgb(0, 211, 242);
    pub const CYAN_500: Pixel = Pixel::from_rgb(0, 184, 219);
    pub const CYAN_600: Pixel = Pixel::from_rgb(0, 146, 184);
    pub const CYAN_700: Pixel = Pixel::from_rgb(0, 117, 149);
    pub const CYAN_800: Pixel = Pixel::from_rgb(0, 95, 120);
    pub const CYAN_900: Pixel = Pixel::from_rgb(16, 78, 100);
    pub const CYAN_950: Pixel = Pixel::from_rgb(5, 51, 69);
    pub const SKY_50: Pixel = Pixel::from_rgb(240, 249, 255);
    pub const SKY_100: Pixel = Pixel::from_rgb(223, 242, 254);
    pub const SKY_200: Pixel = Pixel::from_rgb(184, 230, 254);
    pub const SKY_300: Pixel = Pixel::from_rgb(116, 212, 255);
    pub const SKY_400: Pixel = Pixel::from_rgb(0, 188, 255);
    pub const SKY_500: Pixel = Pixel::from_rgb(0, 166, 244);
    pub const SKY_600: Pixel = Pixel::from_rgb(0, 132, 209);
    pub const SKY_700: Pixel = Pixel::from_rgb(0, 105, 168);
    pub const SKY_800: Pixel = Pixel::from_rgb(0, 89, 138);
    pub const SKY_900: Pixel = Pixel::from_rgb(2, 74, 112);
    pub const SKY_950: Pixel = Pixel::from_rgb(5, 47, 74);
    pub const BLUE_50: Pixel = Pixel::from_rgb(239, 246, 255);
    pub const BLUE_100: Pixel = Pixel::from_rgb(219, 234, 254);
    pub const BLUE_200: Pixel = Pixel::from_rgb(190, 219, 255);
    pub const BLUE_300: Pixel = Pixel::from_rgb(142, 197, 255);
    pub const BLUE_400: Pixel = Pixel::from_rgb(81, 162, 255);
    pub const BLUE_500: Pixel = Pixel::from_rgb(43, 127, 255);
    pub const BLUE_600: Pixel = Pixel::from_rgb(21, 93, 252);
    pub const BLUE_700: Pixel = Pixel::from_rgb(20, 71, 230);
    pub const BLUE_800: Pixel = Pixel::from_rgb(25, 60, 184);
    pub const BLUE_900: Pixel = Pixel::from_rgb(28, 57, 142);
    pub const BLUE_950: Pixel = Pixel::from_rgb(22, 36, 86);
    pub const INDIGO_50: Pixel = Pixel::from_rgb(238, 242, 255);
    pub const INDIGO_100: Pixel = Pixel::from_rgb(224, 231, 255);
    pub const INDIGO_200: Pixel = Pixel::from_rgb(198, 210, 255);
    pub const INDIGO_300: Pixel = Pixel::from_rgb(163, 179, 255);
    pub const INDIGO_400: Pixel = Pixel::from_rgb(124, 134, 255);
    pub const INDIGO_500: Pixel = Pixel::from_rgb(97, 95, 255);
    pub const INDIGO_600: Pixel = Pixel::from_rgb(79, 57, 246);
    pub const INDIGO_700: Pixel = Pixel::from_rgb(67, 45, 215);
    pub const INDIGO_800: Pixel = Pixel::from_rgb(55, 42, 172);
    pub const INDIGO_900: Pixel = Pixel::from_rgb(49, 44, 133);
    pub const INDIGO_950: Pixel = Pixel::from_rgb(30, 26, 77);
    pub const VIOLET_50: Pixel = Pixel::from_rgb(245, 243, 255);
    pub const VIOLET_100: Pixel = Pixel::from_rgb(237, 233, 254);
    pub const VIOLET_200: Pixel = Pixel::from_rgb(221, 214, 255);
    pub const VIOLET_300: Pixel = Pixel::from_rgb(196, 180, 255);
    pub const VIOLET_400: Pixel = Pixel::from_rgb(166, 132, 255);
    pub const VIOLET_500: Pixel = Pixel::from_rgb(142, 81, 255);
    pub const VIOLET_600: Pixel = Pixel::from_rgb(127, 34, 254);
    pub const VIOLET_700: Pixel = Pixel::from_rgb(112, 8, 231);
    pub const VIOLET_800: Pixel = Pixel::from_rgb(93, 14, 192);
    pub const VIOLET_900: Pixel = Pixel::from_rgb(77, 23, 154);
    pub const VIOLET_950: Pixel = Pixel::from_rgb(47, 13, 104);
    pub const PURPLE_50: Pixel = Pixel::from_rgb(250, 245, 255);
    pub const PURPLE_100: Pixel = Pixel::from_rgb(243, 232, 255);
    pub const PURPLE_200: Pixel = Pixel::from_rgb(233, 212, 255);
    pub const PURPLE_300: Pixel = Pixel::from_rgb(218, 178, 255);
    pub const PURPLE_400: Pixel = Pixel::from_rgb(194, 122, 255);
    pub const PURPLE_500: Pixel = Pixel::from_rgb(173, 70, 255);
    pub const PURPLE_600: Pixel = Pixel::from_rgb(152, 16, 250);
    pub const PURPLE_700: Pixel = Pixel::from_rgb(130, 0, 219);
    pub const PURPLE_800: Pixel = Pixel::from_rgb(110, 17, 176);
    pub const PURPLE_900: Pixel = Pixel::from_rgb(89, 22, 139);
    pub const PURPLE_950: Pixel = Pixel::from_rgb(60, 3, 102);
    pub const FUCHSIA_50: Pixel = Pixel::from_rgb(253, 244, 255);
    pub const FUCHSIA_100: Pixel = Pixel::from_rgb(250, 232, 255);
    pub const FUCHSIA_200: Pixel = Pixel::from_rgb(246, 207, 255);
    pub const FUCHSIA_300: Pixel = Pixel::from_rgb(244, 168, 255);
    pub const FUCHSIA_400: Pixel = Pixel::from_rgb(237, 106, 255);
    pub const FUCHSIA_500: Pixel = Pixel::from_rgb(225, 42, 251);
    pub const FUCHSIA_600: Pixel = Pixel::from_rgb(200, 0, 222);
    pub const FUCHSIA_700: Pixel = Pixel::from_rgb(168, 0, 183);
    pub const FUCHSIA_800: Pixel = Pixel::from_rgb(138, 1, 148);
    pub const FUCHSIA_900: Pixel = Pixel::from_rgb(114, 19, 120);
    pub const FUCHSIA_950: Pixel = Pixel::from_rgb(75, 0, 79);
    pub const PINK_50: Pixel = Pixel::from_rgb(253, 242, 248);
    pub const PINK_100: Pixel = Pixel::from_rgb(252, 231, 243);
    pub const PINK_200: Pixel = Pixel::from_rgb(252, 206, 232);
    pub const PINK_300: Pixel = Pixel::from_rgb(253, 165, 213);
    pub const PINK_400: Pixel = Pixel::from_rgb(251, 100, 182);
    pub const PINK_500: Pixel = Pixel::from_rgb(246, 51, 154);
    pub const PINK_600: Pixel = Pixel::from_rgb(230, 0, 118);
    pub const PINK_700: Pixel = Pixel::from_rgb(198, 0, 92);
    pub const PINK_800: Pixel = Pixel::from_rgb(163, 0, 76);
    pub const PINK_900: Pixel = Pixel::from_rgb(134, 16, 67);
    pub const PINK_950: Pixel = Pixel::from_rgb(81, 4, 36);
    pub const ROSE_50: Pixel = Pixel::from_rgb(255, 241, 242);
    pub const ROSE_100: Pixel = Pixel::from_rgb(255, 228, 230);
    pub const ROSE_200: Pixel = Pixel::from_rgb(255, 204, 211);
    pub const ROSE_300: Pixel = Pixel::from_rgb(255, 161, 173);
    pub const ROSE_400: Pixel = Pixel::from_rgb(255, 99, 126);
    pub const ROSE_500: Pixel = Pixel::from_rgb(255, 32, 86);
    pub const ROSE_600: Pixel = Pixel::from_rgb(236, 0, 63);
    pub const ROSE_700: Pixel = Pixel::from_rgb(199, 0, 54);
    pub const ROSE_800: Pixel = Pixel::from_rgb(165, 0, 54);
    pub const ROSE_900: Pixel = Pixel::from_rgb(139, 8, 54);
    pub const ROSE_950: Pixel = Pixel::from_rgb(77, 2, 24);
}

impl From<image::Rgba<u8>> for Pixel {
    fn from(value: image::Rgba<u8>) -> Self {
        Self { raw: value.0 }
    }
}

impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.rgba == other.rgba }
    }
}

impl Eq for Pixel {}

impl Hash for Pixel {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unsafe {
            self.rgba.hash(state);
        }
    }
}

impl Debug for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", unsafe { self.rgba })
    }
}
