use sge::prelude::*;

pub struct Background {
    pub bg: Color,
    pub fg: Color,
    width: usize,
    height: usize,
    grid_size: usize,
    texture: TextureRef,

    wave_speed: f32,
    wave_amplitude: f32,
    wave_frequency: f32,
}

impl Background {
    pub fn new(width: usize, height: usize, fg: Color, bg: Color, grid_size: usize) -> Self {
        Self {
            fg,
            bg,
            width,
            height,
            texture: SgeTexture::empty(width as u32, height as u32)
                .unwrap()
                .create(),
            grid_size,
            wave_speed: 3.0,
            wave_amplitude: 3.0,
            wave_frequency: 1.0,
        }
    }

    fn calculate_wave_offset(&self, x: f32, y: f32, t: f32) -> f32 {
        let base_wave =
            ((x + y * 0.5) * self.wave_frequency + t * self.wave_speed).sin() * self.wave_amplitude;
        let slow_wave = (t * 0.5).sin();
        base_wave + slow_wave
    }

    pub fn frame(&mut self) -> TextureRef {
        let cols = (self.width / self.grid_size) as isize + 3;
        let rows = (self.height / self.grid_size) as isize + 3;
        let t = time();

        let mut image = Image::gen_color(self.width, self.height, self.bg.to_color_u8());

        let get_point = |col: isize, row: isize| -> Vec2 {
            let px = (col * self.grid_size as isize) as f32
                + self.calculate_wave_offset(row as f32, col as f32, t);
            let py = (row * self.grid_size as isize) as f32
                + self.calculate_wave_offset(col as f32, row as f32, t);
            vec2(px, py)
        };

        for row in -3..=rows {
            for col in -3..=cols {
                let current = get_point(col, row);

                if col < cols {
                    let right = get_point(col + 1, row);
                    image.line(current.as_ivec2(), right.as_ivec2(), self.fg.to_color_u8());
                }

                if row < rows {
                    let bottom = get_point(col, row + 1);
                    image.line(current.as_ivec2(), bottom.as_ivec2(), self.fg.to_color_u8());
                }
            }
        }

        self.texture
            .replace(SgeTexture::from_engine_image(image).unwrap());

        self.texture
    }
}
