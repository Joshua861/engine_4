use background::Background;
use sge::{math::collision::HasBounds2D, prelude::*};
use undo::UndoCache;

mod background;
mod undo;

struct State {
    controller: PanningCameraController,
    canvas: Canvas,
    background: Background,
    picker: ColorPicker,

    brush_texture: TextureRef,
    line_texture: TextureRef,
    rubber_texture: TextureRef,
    eyedropper_texture: TextureRef,
    undo_texture: TextureRef,
    redo_texture: TextureRef,
    dithering_none: TextureRef,
    dithering_vertical: TextureRef,
    dithering_horizontal: TextureRef,
    dithering_checkerboard: TextureRef,
    dithering_checkerboard_alternate: TextureRef,
    dithering_crosshatch: TextureRef,
    dithering_dotted: TextureRef,
    dithering_random: TextureRef,
    thick_texture: TextureRef,
    thin_texture: TextureRef,
    medium_texture: TextureRef,
}

struct ColorPicker {
    image: Image,
    texture: TextureRef,
}

impl State {
    fn new() -> Self {
        let mut controller = PanningCameraController::new();
        controller.set_pan_button(MouseButton::Middle);
        Self {
            controller,
            canvas: Canvas::new(300, 200, ColorU8::NEUTRAL_100),
            background: Background::new(192 * 2, 108 * 2, Color::BLUE_400, Color::NEUTRAL_200, 20),
            picker: gen_color_picker(),

            brush_texture: include_texture!("../assets/brush.png"),
            line_texture: include_texture!("../assets/line.png"),
            rubber_texture: include_texture!("../assets/rubber.png"),
            eyedropper_texture: include_texture!("../assets/eyedropper.png"),
            undo_texture: include_texture!("../assets/undo.png"),
            redo_texture: include_texture!("../assets/redo.png"),
            dithering_none: include_texture!("../assets/no-dithering.png"),
            dithering_checkerboard: include_texture!("../assets/checkerboard.png"),
            dithering_checkerboard_alternate: include_texture!(
                "../assets/checkerboard-alternate.png"
            ),
            dithering_horizontal: include_texture!("../assets/horizontal-dithering.png"),
            dithering_vertical: include_texture!("../assets/vertical-dithering.png"),
            dithering_crosshatch: include_texture!("../assets/crosshatch-dithering.png"),
            dithering_dotted: include_texture!("../assets/dotted-dithering.png"),
            dithering_random: include_texture!("../assets/random-dithering.png"),
            thick_texture: include_texture!("../assets/thick.png"),
            thin_texture: include_texture!("../assets/thin.png"),
            medium_texture: include_texture!("../assets/medium.png"),
        }
    }

    fn update(&mut self) {
        if !held_control() {
            self.controller.update();
        }

        let picker_margin = 15.0;
        let picker_position = vec2(
            picker_margin,
            window_height() - PICKER_SCALE.y - picker_margin,
        );

        if let Some(c) = cursor()
            && mouse_held(MouseButton::Left)
        {
            let sf = PICKER_SCALE / self.picker.texture.dimensions.as_vec2();
            let offset = c - picker_position;
            if offset.x <= PICKER_SCALE.x
                && offset.y <= PICKER_SCALE.y
                && offset.y >= 0.0
                && offset.x >= 0.0
            {
                let color = self
                    .picker
                    .image
                    .get_pixel((offset.x / sf.x) as usize, (offset.y / sf.y) as usize);
                self.canvas.color = *color.unwrap();
            } else {
                self.canvas.update();
            }
        } else {
            self.canvas.update();
        }

        // self.background.bg = self.canvas.background.to_color();
        // self.background.fg = self.canvas.color.to_color();

        draw_texture(
            self.background.frame(),
            Vec2::ZERO,
            (window_height() * 2.0).max(window_width()),
        );

        // temporary band aid to create a new render step and
        // draw the world texture over the screen textrue
        saturate_screen(1.0);

        draw_texture_world(
            self.canvas.texture(),
            Vec2::ZERO,
            self.canvas.texture.dimensions.x as f32,
        );

        self.show_buttons();

        draw_rect(
            picker_position - Vec2::splat(picker_margin),
            PICKER_SCALE + Vec2::splat(picker_margin * 2.0),
            self.canvas.color.to_color(),
        );
        draw_texture_scaled(self.picker.texture, picker_position, PICKER_SCALE);

        // show_debug_info();
        // run_ui(|_| {});
    }

    fn show_buttons(&mut self) {
        let width = window_width();
        let num_buttons = 4;
        let tools = [
            Tool::Brush,
            Tool::Line { point: None },
            Tool::Rubber,
            Tool::Eyedropper,
        ];
        let keybinds = [KeyCode::KeyB, KeyCode::KeyF, KeyCode::KeyR, KeyCode::KeyE];
        let textures = [
            self.brush_texture,
            self.line_texture,
            self.rubber_texture,
            self.eyedropper_texture,
        ];
        let button_scale = Vec2::splat(100.0);
        let origin = (width - (button_scale.x * num_buttons as f32)) / 2.0;
        let margin = 30.0;

        for i in 0..num_buttons {
            let x = origin + (button_scale.x + margin) * i as f32;
            let y = margin;

            draw_button(vec2(x, y), button_scale, textures[i], keybinds[i], || {
                self.canvas.tool = tools[i]
            });
        }

        let dither_texture = match self.canvas.dithering {
            Dithering::Checkerboard => self.dithering_checkerboard,
            Dithering::CheckerboardAlt => self.dithering_checkerboard_alternate,
            Dithering::Horizontal => self.dithering_horizontal,
            Dithering::None => self.dithering_none,
            Dithering::Vertical => self.dithering_vertical,
            Dithering::Crosshatch => self.dithering_crosshatch,
            Dithering::Random => self.dithering_random,
            Dithering::Dotted => self.dithering_dotted,
        };

        if let Some(clicked) = draw_button_with_alt(
            Vec2::splat(margin),
            button_scale,
            dither_texture,
            KeyCode::KeyD,
        ) {
            if clicked {
                self.canvas.dithering.cycle();
            } else {
                self.canvas.dithering.cycle_backward();
            }
        }

        draw_button(
            vec2(window_width() - button_scale.x * 2.0 - margin * 2.0, margin),
            button_scale,
            self.undo_texture,
            KeyCode::KeyZ,
            || {
                self.canvas
                    .undo_cache
                    .undo_buffer(self.canvas.image.bytes_mut())
            },
        );
        draw_button(
            vec2(window_width() - button_scale.x - margin, margin),
            button_scale,
            self.redo_texture,
            KeyCode::KeyY,
            || {
                self.canvas
                    .undo_cache
                    .redo_buffer(self.canvas.image.bytes_mut())
            },
        );
    }
}

fn draw_button<F: FnOnce()>(
    top_left: Vec2,
    size: Vec2,
    texture: TextureRef,
    keybind: KeyCode,
    onclick: F,
) {
    if let Some(true) = draw_button_with_alt(top_left, size, texture, keybind) {
        onclick();
    }
}

fn draw_button_with_alt(
    top_left: Vec2,
    size: Vec2,
    texture: TextureRef,
    keybind: KeyCode,
) -> Option<bool> {
    let mut rect = Rect::new(top_left, size, Color::NEUTRAL_100);
    let bounds = rect.bounds();

    if bounds.is_mouse_over() {
        rect.color = Color::NEUTRAL_200;
    }

    if bounds.is_mouse_held_on(MouseButton::Left) {
        rect.color = Color::NEUTRAL_300;
    }

    let result = if ((held_alt() || held_shift())
        && (bounds.is_mouse_clicked_on(MouseButton::Left) || key_pressed(keybind)))
        || bounds.is_mouse_clicked_on(MouseButton::Right)
    {
        Some(false)
    } else if bounds.is_mouse_clicked_on(MouseButton::Left) || key_pressed(keybind) {
        Some(true)
    } else {
        None
    };

    draw(&rect);

    if held_shift() {
        let key = keybind.format_as_string();
        let dimensions = measure_text_ex(key, MONO, 60);
        let offset = (size - dimensions.size) / 2.0;
        let position = top_left + offset;
        draw_text_ex(key, position, Color::BLACK, 60);
    } else {
        draw_texture_scaled(texture, rect.top_left, rect.size);
    }
    draw_rect_outline(rect.top_left, rect.size, 5.0, Color::NEUTRAL_200);

    result
}

struct Canvas {
    color: ColorU8,
    background: ColorU8,
    thickness: BrushThickness,
    dithering: Dithering,
    tool: Tool,

    previous_brush_pos: Option<Vec2>,
    stroke_idx: u64,

    image: Image,
    texture: TextureRef,
    undo_cache: UndoCache,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tool {
    Brush,
    Line { point: Option<IVec2> },
    Rubber,
    Eyedropper,
}

fn draw_function(
    image: &mut Image,
    x: i32,
    y: i32,
    color: ColorU8,
    dithering: &Dithering,
    thickness: &BrushThickness,
    stroke_idx: u64,
) {
    let mut set = |x: i32, y: i32| {
        if dithering.should_place(x as usize, y as usize, stroke_idx) {
            image.seti(x, y, color);
        }
    };

    match thickness {
        BrushThickness::Thin => set(x, y),
        BrushThickness::Medium => {
            for x in x..=x + 1 {
                for y in y..=y + 1 {
                    set(x, y);
                }
            }
        }
        BrushThickness::Thick => {
            for x in x - 1..=x + 1 {
                for y in y - 1..=y + 1 {
                    set(x, y);
                }
            }
        }
    }
}

impl Canvas {
    fn new(width: usize, height: usize, background: ColorU8) -> Self {
        let image = Image::gen_color(width, height, background);
        let mut undo_cache = UndoCache::new();
        undo_cache.handle_buffer_update(image.bytes());

        let camera = get_camera_2d_mut();
        let scale_1 = window_width() / width as f32;
        let scale_2 = window_height() / height as f32;
        camera.set_scale((scale_1.min(scale_2)) * 0.9);
        *camera.translation_mut() += usizevec2(width, height).as_vec2() / 2.0;

        Self {
            tool: Tool::Brush,
            thickness: BrushThickness::Medium,
            dithering: Dithering::None,
            color: background.invert(),
            background,

            previous_brush_pos: None,

            texture: SgeTexture::from_engine_image(image.clone())
                .unwrap()
                .create(),
            image,
            undo_cache,
            stroke_idx: 0,
        }
    }

    fn draw_color(&self) -> ColorU8 {
        match self.tool {
            Tool::Brush => self.color,
            Tool::Rubber => self.background,
            _ => self.color,
        }
    }

    fn posi() -> IVec2 {
        screen_to_world(last_cursor_pos()).as_ivec2()
    }

    fn update(&mut self) {
        if mouse_pressed(MouseButton::Left) {
            self.stroke_idx += 1;
        }

        if self.tool == Tool::Brush || self.tool == Tool::Rubber {
            if mouse_held(MouseButton::Left) {
                let posi = Self::posi();
                self.draw_point(posi);
                for pos in cursor_movements() {
                    let posi = screen_to_world(pos).as_ivec2();

                    if let Some(previous_pos) = self.previous_brush_pos {
                        let previous_posi = screen_to_world(previous_pos).as_ivec2();
                        self.draw_line(posi, previous_posi);
                    }

                    self.draw_point(posi);
                    self.previous_brush_pos = Some(pos);
                }
            }

            if mouse_released(MouseButton::Left) {
                self.previous_brush_pos = None;
                self.undo_cache.handle_buffer_update(self.image.bytes());
            }
        }

        let pos = Self::posi();
        let pos_inside_canvas = self.pos_inside_canvas(pos);
        if let Tool::Line { point } = &mut self.tool {
            let pos = Self::posi();
            if mouse_pressed(MouseButton::Left) && pos_inside_canvas {
                if let Some(a) = point {
                    let b = pos;

                    self.image
                        .line_internal(*a, b, self.color, |image, x, y, c| {
                            draw_function(
                                image,
                                x,
                                y,
                                c,
                                &self.dithering,
                                &self.thickness,
                                self.stroke_idx,
                            );
                        });

                    *point = None;
                    self.undo_cache.handle_buffer_update(self.image.bytes());
                } else {
                    *point = Some(pos);
                }
            }

            if mouse_pressed(MouseButton::Right) {
                *point = None;
            }
        }

        if self.tool == Tool::Eyedropper {
            if mouse_pressed(MouseButton::Left) {
                let pos = Self::posi();
                if self.pos_inside_canvas(pos) {
                    self.color = *self
                        .image
                        .get_pixel(pos.x as usize, pos.y as usize)
                        .unwrap();
                }
            }
        }

        if held_control() {
            match scroll_diff().y {
                0.0 => (),
                a => match a.signum() {
                    -1.0 => self.thickness.decrement(),
                    1.0 => self.thickness.increment(),
                    _ => (),
                },
            }
        }
    }

    #[inline]
    fn pos_inside_canvas(&self, pos: IVec2) -> bool {
        pos.x >= 0
            && pos.y >= 0
            && pos.x < self.image.dimensions().x as i32
            && pos.y < self.image.dimensions().y as i32
    }

    #[inline]
    fn draw_line(&mut self, a: IVec2, b: IVec2) {
        self.image
            .line_internal(a, b, self.draw_color(), |image, x, y, c| {
                draw_function(
                    image,
                    x,
                    y,
                    c,
                    &self.dithering,
                    &self.thickness,
                    self.stroke_idx,
                );
            });
    }

    #[inline]
    fn draw_point(&mut self, a: IVec2) {
        let color = self.draw_color();
        draw_function(
            &mut self.image,
            a.x,
            a.y,
            color,
            &self.dithering,
            &self.thickness,
            self.stroke_idx,
        );
    }

    fn texture(&mut self) -> TextureRef {
        let mut image = self.image.clone();

        let pos = screen_to_world(last_cursor_pos()).as_ivec2();
        if self.tool == Tool::Brush || self.tool == Tool::Rubber {
            draw_function(
                &mut image,
                pos.x,
                pos.y,
                self.draw_color(),
                &self.dithering,
                &self.thickness,
                self.stroke_idx,
            );
        } else if let Tool::Line { point: Some(point) } = self.tool {
            image.line_internal(point, pos, self.color, |image, x, y, c| {
                draw_function(
                    image,
                    x,
                    y,
                    c,
                    &self.dithering,
                    &self.thickness,
                    self.stroke_idx,
                );
            });
        }

        self.texture
            .replace(SgeTexture::from_engine_image(image).unwrap());

        self.texture
    }
}

const PICKER_SCALE: Vec2 = vec2(360.0 + 30.0, 255.0);
fn gen_color_picker() -> ColorPicker {
    let height = 255;
    let width = 360;
    let grayscale_width = 30;

    let mut image = Image::gen_color(width + grayscale_width, height, ColorU8::WHITE);

    for y in 0..height {
        let v = y * (255 / height);
        let pixel = ColorU8::splat(v as u8);

        for x in 0..grayscale_width {
            image.set(x, y, pixel);
        }
    }

    for y in 0..height {
        for x in 0..width {
            let display_x = x + grayscale_width;

            let color = Color::from_hsl(
                x as f32 * (360.0 / width as f32),
                0.9,
                (height - y) as f32 / height as f32,
            );
            image.set(display_x, y, color.to_color_u8());
        }
    }

    ColorPicker {
        texture: SgeTexture::from_engine_image(image.clone())
            .unwrap()
            .create(),
        image,
    }
}

enum Dithering {
    None,
    Checkerboard,
    Vertical,
    Horizontal,
    CheckerboardAlt,
    Crosshatch,
    Dotted,
    Random,
}

impl Dithering {
    fn should_place(&self, x: usize, y: usize, stroke_idx: u64) -> bool {
        match self {
            Self::None => true,
            Self::Checkerboard => x % 2 == y % 2,
            Self::CheckerboardAlt => x % 2 != y % 2,
            Self::Vertical => x.is_multiple_of(2),
            Self::Horizontal => y.is_multiple_of(2),
            Self::Crosshatch => y.is_multiple_of(2) || x.is_multiple_of(2),
            Self::Dotted => y.is_multiple_of(2) && x.is_multiple_of(2),
            Self::Random => Self::hash_coords(x, y, stroke_idx) % 2 == 0,
        }
    }

    fn hash_coords(x: usize, y: usize, stroke_idx: u64) -> u64 {
        let x = x as u64;
        let y = y as u64;
        let i = stroke_idx;

        let mut hash = x.wrapping_mul(0x9e3779b97f4a7c15);
        hash ^= y.wrapping_mul(0xbf58476d1ce4e5b9);
        hash ^= i.wrapping_mul(0x94d049bb133111eb);
        hash ^= hash >> 27;
        hash = hash.wrapping_mul(0x94d049bb133111eb);
        hash ^= hash >> 31;
        hash
    }

    fn cycle(&mut self) {
        *self = match self {
            Self::None => Self::Checkerboard,
            Self::Checkerboard => Self::CheckerboardAlt,
            Self::CheckerboardAlt => Self::Vertical,
            Self::Vertical => Self::Horizontal,
            Self::Horizontal => Self::Crosshatch,
            Self::Crosshatch => Self::Dotted,
            Self::Dotted => Self::Random,
            Self::Random => Self::None,
        }
    }

    fn cycle_backward(&mut self) {
        *self = match self {
            Self::None => Self::Random,
            Self::Checkerboard => Self::None,
            Self::CheckerboardAlt => Self::Checkerboard,
            Self::Vertical => Self::CheckerboardAlt,
            Self::Horizontal => Self::Vertical,
            Self::Crosshatch => Self::Horizontal,
            Self::Dotted => Self::Crosshatch,
            Self::Random => Self::Dotted,
        }
    }
}

#[derive(Debug)]
enum BrushThickness {
    Thin,
    Medium,
    Thick,
}

impl BrushThickness {
    fn radius(&self) -> i32 {
        match self {
            Self::Thin => 0,
            Self::Medium => 1,
            Self::Thick => 2,
        }
    }

    fn increment(&mut self) {
        *self = match self {
            Self::Thin => Self::Medium,
            Self::Medium => Self::Thick,
            Self::Thick => Self::Thick,
        }
    }

    fn decrement(&mut self) {
        *self = match self {
            Self::Medium => Self::Thin,
            Self::Thick => Self::Medium,
            Self::Thin => Self::Thin,
        }
    }
}

#[main("Paint")]
fn main() -> anyhow::Result<()> {
    let mut state = State::new();

    loop {
        state.update();

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}
