// slop mostly

use glium::{
    IndexBuffer, Program, Surface, VertexBuffer,
    framebuffer::SimpleFrameBuffer,
    texture::{Texture2d, TextureCreationError},
    uniform,
};
use sge_color::Color;
use sge_config::get_dithering;
use sge_error_union::ErrorUnion;
use sge_programs::{COPY_PROGRAM, ProgramRef, load_program_sync};
use sge_shapes::d2::QUAD_INDICES;
use sge_textures::TextureRef;
use sge_types::TexturedVertex2D;
use sge_vectors::Vec2;
use sge_window::{SgeDisplay, get_display};

#[derive(Clone, Debug)]
pub enum PostProcessingEffect {
    Bloom {
        threshold: f32,
        knee: f32,
        intensity: f32,
        blur_radius: f32,
        iterations: u32,
    },
    GaussianBlur {
        sigma: f32,
    },
    Pixelate {
        pixel_size: f32,
    },
    Saturate(f32),
    HueRotate(f32),
    Brighten(f32),
    Vignette {
        color: Color,
        intensity: f32,
    },
    Contrast(f32),
    Grayscale,
    Invert,
    ChromaticAberration {
        strength: f32,
    },
    Sharpen {
        strength: f32,
    },
    FilmGrain {
        strength: f32,
        seed: f32,
    },
    Quantization {
        colors_per_channel: f32,
    },
    Wobbly {
        time: f32,
        speed: f32,
        amplitude: f32,
        frequency: f32,
    },
    FishEye {
        strength: f32,
    },
}

#[derive(ErrorUnion, Debug)]
pub enum PostProcessingError {
    Texture(TextureCreationError),
    Quad(RenderFullscreenQuadError),
    Framebuffer(glium::framebuffer::ValidationError),
}

impl PostProcessingEffect {
    pub fn apply<T: Surface>(
        &self,
        source: TextureRef,
        target: &mut T,
        screen_size: Vec2,
    ) -> Result<(), PostProcessingError> {
        let display = get_display();

        match self {
            Self::Sharpen { strength } => {
                let program = get_or_create_sharpen_program();
                let uniforms = uniform! {
                    tex: source.get().gl_texture.sampled(),
                    strength: *strength,
                    screen_size: [screen_size.x, screen_size.y],
                };
                render_fullscreen_quad(target, program.get(), &uniforms)?;
            }
            Self::GaussianBlur { sigma } => {
                let temp_texture = create_temp_texture(display, screen_size)?;
                let mut temp_fb = SimpleFrameBuffer::new(display, &temp_texture)?;

                let program = get_or_create_gaussian_blur_program();
                let uniforms = uniform! {
                    tex: source.get().gl_texture.sampled(),
                    sigma: *sigma,
                    direction: [1.0f32, 0.0f32],
                    screen_size: [screen_size.x, screen_size.y],
                };
                render_fullscreen_quad(&mut temp_fb, program.get(), &uniforms)?;

                let uniforms = uniform! {
                    tex: temp_texture.sampled(),
                    sigma: *sigma,
                    direction: [0.0f32, 1.0f32],
                    screen_size: [screen_size.x, screen_size.y],
                };
                render_fullscreen_quad(target, program.get(), &uniforms)?;
            }
            Self::Pixelate { pixel_size } => {
                let program = get_or_create_pixelate_program();
                let uniforms = uniform! {
                    tex: source.get().gl_texture.sampled(),
                    pixel_size: *pixel_size,
                    screen_size: [screen_size.x, screen_size.y],
                };
                render_fullscreen_quad(target, program.get(), &uniforms)?;
            }
            Self::Saturate(amount) => {
                let program = get_or_create_saturate_program();
                let uniforms = uniform! {
                    tex: source.get().gl_texture.sampled(),
                    saturation: *amount,
                };
                render_fullscreen_quad(target, program.get(), &uniforms)?;
            }
            Self::HueRotate(degrees) => {
                let program = get_or_create_hue_rotate_program();
                let uniforms = uniform! {
                    tex: source.get().gl_texture.sampled(),
                    hue_shift: degrees.to_radians(),
                };
                render_fullscreen_quad(target, program.get(), &uniforms)?;
            }
            Self::Brighten(amount) => {
                let program = get_or_create_brighten_program();
                let uniforms = uniform! {
                    tex: source.get().gl_texture.sampled(),
                    brightness: *amount,
                };
                render_fullscreen_quad(target, program.get(), &uniforms)?;
            }
            Self::Vignette { color, intensity } => {
                let program = get_or_create_vignette_program();
                let uniforms = uniform! {
                    tex: source.get().gl_texture.sampled(),
                    vignette_color: color.for_gpu(),
                    vignette_intensity: *intensity,
                    screen_size: [screen_size.x, screen_size.y],
                };
                render_fullscreen_quad(target, program.get(), &uniforms)?;
            }
            Self::Bloom {
                threshold,
                knee,
                intensity,
                blur_radius,
                iterations,
            } => {
                let bright_texture = create_temp_texture(display, screen_size)?;
                {
                    let mut bright_fb = SimpleFrameBuffer::new(display, &bright_texture)?;
                    let bright_program = get_or_create_bright_pass_program();
                    let uniforms = uniform! {
                        tex: source.get().gl_texture.sampled(),
                        threshold: *threshold,
                        knee: *knee,
                    };
                    render_fullscreen_quad(&mut bright_fb, bright_program.get(), &uniforms)?;
                }

                let ping_texture = create_temp_texture(display, screen_size)?;
                let pong_texture = create_temp_texture(display, screen_size)?;

                {
                    let mut ping_fb = SimpleFrameBuffer::new(display, &ping_texture)?;
                    let copy_program = COPY_PROGRAM;
                    let uniforms = uniform! { tex: bright_texture.sampled() };
                    render_fullscreen_quad(&mut ping_fb, copy_program.get(), &uniforms)?;
                }

                for i in 0..*iterations {
                    let current_radius = *blur_radius * (1.0 - (i as f32 / *iterations as f32));

                    {
                        let mut pong_fb = SimpleFrameBuffer::new(display, &pong_texture)?;
                        let blur_program = get_or_create_gaussian_blur_program();
                        let uniforms = uniform! {
                            tex: ping_texture.sampled(),
                            sigma: current_radius,
                            direction: [1.0f32, 0.0f32],
                            screen_size: [screen_size.x, screen_size.y],
                        };
                        render_fullscreen_quad(&mut pong_fb, blur_program.get(), &uniforms)?;
                    }

                    {
                        let mut ping_fb = SimpleFrameBuffer::new(display, &ping_texture)?;
                        let blur_program = get_or_create_gaussian_blur_program();
                        let uniforms = uniform! {
                            tex: pong_texture.sampled(),
                            sigma: current_radius,
                            direction: [0.0f32, 1.0f32],
                            screen_size: [screen_size.x, screen_size.y],
                        };
                        render_fullscreen_quad(&mut ping_fb, blur_program.get(), &uniforms)?;
                    }
                }

                let combine_program = get_or_create_bloom_combine_program();
                let uniforms = uniform! {
                    tex: source.get().gl_texture.sampled(),
                    bloom_tex: ping_texture.sampled(),
                    intensity: *intensity,
                };
                render_fullscreen_quad(target, combine_program.get(), &uniforms)?;
            }
            Self::Contrast(amount) => {
                let program = get_or_create_contrast_program();
                let uniforms = uniform! {
                    tex: source.get().gl_texture.sampled(),
                    contrast: *amount,
                };
                render_fullscreen_quad(target, program.get(), &uniforms)?;
            }
            Self::Grayscale => {
                let program = get_or_create_grayscale_program();
                let uniforms = uniform! {
                    tex: source.get().gl_texture.sampled(),
                };
                render_fullscreen_quad(target, program.get(), &uniforms)?;
            }
            Self::Invert => {
                let program = get_or_create_invert_program();
                let uniforms = uniform! {
                    tex: source.get().gl_texture.sampled(),
                };
                render_fullscreen_quad(target, program.get(), &uniforms)?;
            }
            Self::ChromaticAberration { strength } => {
                let program = get_or_create_chromatic_aberration_program();
                let uniforms = uniform! {
                    tex: source.get().gl_texture.sampled(),
                    strength: *strength,
                    screen_size: [screen_size.x, screen_size.y],
                };
                render_fullscreen_quad(target, program.get(), &uniforms)?;
            }
            Self::FilmGrain { strength, seed } => {
                let program = get_or_create_film_grain_program();
                let uniforms = uniform! {
                    tex: source.get().gl_texture.sampled(),
                    strength: *strength,
                    seed: *seed,
                    screen_size: [screen_size.x, screen_size.y],
                };
                render_fullscreen_quad(target, program.get(), &uniforms)?;
            }
            Self::Quantization { colors_per_channel } => {
                let program = get_or_create_quantization_program();
                let uniforms = uniform! {
                    tex: source.get().gl_texture.sampled(),
                    colors_per_channel: *colors_per_channel,
                };
                render_fullscreen_quad(target, program.get(), &uniforms)?;
            }
            Self::Wobbly {
                time,
                speed,
                amplitude,
                frequency,
            } => {
                let program = get_or_create_wobbly_program();
                let uniforms = uniform! {
                    tex: source.get().gl_texture.sampled(),
                    time: *time,
                    speed: *speed,
                    amplitude: *amplitude,
                    frequency: *frequency,
                };
                render_fullscreen_quad(target, program.get(), &uniforms)?;
            }
            Self::FishEye { strength } => {
                let program = get_or_create_fish_eye_program();
                let uniforms = uniform! {
                    tex: source.get().gl_texture.sampled(),
                    strength: *strength,
                };
                render_fullscreen_quad(target, program.get(), &uniforms)?;
            }
        }

        Ok(())
    }
}

fn create_temp_texture(
    display: &SgeDisplay,
    size: Vec2,
) -> Result<Texture2d, TextureCreationError> {
    let texture = Texture2d::empty(display, size.x as u32, size.y as u32)?;
    Ok(texture)
}

#[derive(ErrorUnion, Debug)]
pub enum RenderFullscreenQuadError {
    VertexBuffer(glium::vertex::BufferCreationError),
    IndexBuffer(glium::index::BufferCreationError),
    Draw(glium::DrawError),
}

pub(crate) fn render_fullscreen_quad<T: Surface, U: glium::uniforms::Uniforms>(
    target: &mut T,
    program: &Program,
    uniforms: &U,
) -> Result<(), RenderFullscreenQuadError> {
    let display = get_display();

    let vertices = [
        TexturedVertex2D {
            position: [-1.0, -1.0],
            tex_coords: [0.0, 0.0],
        },
        TexturedVertex2D {
            position: [1.0, -1.0],
            tex_coords: [1.0, 0.0],
        },
        TexturedVertex2D {
            position: [-1.0, 1.0],
            tex_coords: [0.0, 1.0],
        },
        TexturedVertex2D {
            position: [1.0, 1.0],
            tex_coords: [1.0, 1.0],
        },
    ];

    let vertex_buffer = VertexBuffer::new(display, &vertices)?;
    let index_buffer = IndexBuffer::new(
        display,
        glium::index::PrimitiveType::TrianglesList,
        &QUAD_INDICES,
    )?;

    let params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        dithering: get_dithering(),
        ..Default::default()
    };

    target.draw(&vertex_buffer, &index_buffer, program, uniforms, &params)?;

    Ok(())
}

use std::sync::OnceLock;

use crate::pipeline::current_render_pipeline;

static GAUSSIAN_BLUR_PROGRAM: OnceLock<ProgramRef> = OnceLock::new();
static PIXELATE_PROGRAM: OnceLock<ProgramRef> = OnceLock::new();
static SATURATE_PROGRAM: OnceLock<ProgramRef> = OnceLock::new();
static HUE_ROTATE_PROGRAM: OnceLock<ProgramRef> = OnceLock::new();
static BRIGHTEN_PROGRAM: OnceLock<ProgramRef> = OnceLock::new();
static VIGNETTE_PROGRAM: OnceLock<ProgramRef> = OnceLock::new();
static BRIGHT_PASS_PROGRAM: OnceLock<ProgramRef> = OnceLock::new();
static BLOOM_COMBINE_PROGRAM: OnceLock<ProgramRef> = OnceLock::new();
static CONTRAST_PROGRAM: OnceLock<ProgramRef> = OnceLock::new();
static GRAYSCALE_PROGRAM: OnceLock<ProgramRef> = OnceLock::new();
static INVERT_PROGRAM: OnceLock<ProgramRef> = OnceLock::new();
static CHROMATIC_ABERRATION_PROGRAM: OnceLock<ProgramRef> = OnceLock::new();
static SHARPEN_PROGRAM: OnceLock<ProgramRef> = OnceLock::new();
static FILM_GRAIN_PROGRAM: OnceLock<ProgramRef> = OnceLock::new();
static QUANTIZATION_PROGRAM: OnceLock<ProgramRef> = OnceLock::new();
static WOBBLY_PROGRAM: OnceLock<ProgramRef> = OnceLock::new();
static FISH_EYE_PROGRAM: OnceLock<ProgramRef> = OnceLock::new();

fn get_or_create_gaussian_blur_program() -> &'static ProgramRef {
    GAUSSIAN_BLUR_PROGRAM.get_or_init(|| {
        load_program_sync(POSTPROCESS_VERTEX_SHADER, GAUSSIAN_BLUR_FRAGMENT_SHADER).unwrap()
    })
}

fn get_or_create_pixelate_program() -> &'static ProgramRef {
    PIXELATE_PROGRAM.get_or_init(|| {
        load_program_sync(POSTPROCESS_VERTEX_SHADER, PIXELATE_FRAGMENT_SHADER).unwrap()
    })
}

fn get_or_create_saturate_program() -> &'static ProgramRef {
    SATURATE_PROGRAM.get_or_init(|| {
        load_program_sync(POSTPROCESS_VERTEX_SHADER, SATURATE_FRAGMENT_SHADER).unwrap()
    })
}

fn get_or_create_hue_rotate_program() -> &'static ProgramRef {
    HUE_ROTATE_PROGRAM.get_or_init(|| {
        load_program_sync(POSTPROCESS_VERTEX_SHADER, HUE_ROTATE_FRAGMENT_SHADER).unwrap()
    })
}

fn get_or_create_brighten_program() -> &'static ProgramRef {
    BRIGHTEN_PROGRAM.get_or_init(|| {
        load_program_sync(POSTPROCESS_VERTEX_SHADER, BRIGHTEN_FRAGMENT_SHADER).unwrap()
    })
}

fn get_or_create_vignette_program() -> &'static ProgramRef {
    VIGNETTE_PROGRAM.get_or_init(|| {
        load_program_sync(POSTPROCESS_VERTEX_SHADER, VIGNETTE_FRAGMENT_SHADER).unwrap()
    })
}

fn get_or_create_bright_pass_program() -> &'static ProgramRef {
    BRIGHT_PASS_PROGRAM.get_or_init(|| {
        load_program_sync(POSTPROCESS_VERTEX_SHADER, BRIGHT_PASS_FRAGMENT_SHADER).unwrap()
    })
}

fn get_or_create_bloom_combine_program() -> &'static ProgramRef {
    BLOOM_COMBINE_PROGRAM.get_or_init(|| {
        load_program_sync(POSTPROCESS_VERTEX_SHADER, BLOOM_COMBINE_FRAGMENT_SHADER).unwrap()
    })
}

fn get_or_create_contrast_program() -> &'static ProgramRef {
    CONTRAST_PROGRAM.get_or_init(|| {
        load_program_sync(POSTPROCESS_VERTEX_SHADER, CONTRAST_FRAGMENT_SHADER).unwrap()
    })
}

fn get_or_create_grayscale_program() -> &'static ProgramRef {
    GRAYSCALE_PROGRAM.get_or_init(|| {
        load_program_sync(POSTPROCESS_VERTEX_SHADER, GRAYSCALE_FRAGMENT_SHADER).unwrap()
    })
}

fn get_or_create_invert_program() -> &'static ProgramRef {
    INVERT_PROGRAM.get_or_init(|| {
        load_program_sync(POSTPROCESS_VERTEX_SHADER, INVERT_FRAGMENT_SHADER).unwrap()
    })
}

fn get_or_create_chromatic_aberration_program() -> &'static ProgramRef {
    CHROMATIC_ABERRATION_PROGRAM.get_or_init(|| {
        load_program_sync(
            POSTPROCESS_VERTEX_SHADER,
            CHROMATIC_ABERRATION_FRAGMENT_SHADER,
        )
        .unwrap()
    })
}

fn get_or_create_sharpen_program() -> &'static ProgramRef {
    SHARPEN_PROGRAM.get_or_init(|| {
        load_program_sync(POSTPROCESS_VERTEX_SHADER, SHARPEN_FRAGMENT_SHADER).unwrap()
    })
}

fn get_or_create_film_grain_program() -> &'static ProgramRef {
    FILM_GRAIN_PROGRAM.get_or_init(|| {
        load_program_sync(POSTPROCESS_VERTEX_SHADER, FILM_GRAIN_FRAGMENT_SHADER).unwrap()
    })
}

fn get_or_create_quantization_program() -> &'static ProgramRef {
    QUANTIZATION_PROGRAM.get_or_init(|| {
        load_program_sync(POSTPROCESS_VERTEX_SHADER, QUANTIZATION_FRAGMENT_SHADER).unwrap()
    })
}

fn get_or_create_wobbly_program() -> &'static ProgramRef {
    WOBBLY_PROGRAM.get_or_init(|| {
        load_program_sync(POSTPROCESS_VERTEX_SHADER, WOBBLY_FRAGMENT_SHADER).unwrap()
    })
}

fn get_or_create_fish_eye_program() -> &'static ProgramRef {
    FISH_EYE_PROGRAM.get_or_init(|| {
        load_program_sync(POSTPROCESS_VERTEX_SHADER, FISH_EYE_FRAGMENT_SHADER).unwrap()
    })
}

const POSTPROCESS_VERTEX_SHADER: &str = r#"
#version 140
in vec2 position;
in vec2 tex_coords;
out vec2 v_tex_coords;

void main() {
    v_tex_coords = tex_coords;
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;

const GAUSSIAN_BLUR_FRAGMENT_SHADER: &str = r#"
#version 140
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;
uniform float sigma;
uniform vec2 direction;
uniform vec2 screen_size;

const float PI = 3.14159265359;

float gaussian(float x, float sigma) {
    return exp(-(x * x) / (2.0 * sigma * sigma)) / (sqrt(2.0 * PI) * sigma);
}

void main() {

    int radius = int(ceil(3.0 * sigma));

    vec2 tex_offset = direction / screen_size;
    vec4 result = vec4(0.0);
    float weight_sum = 0.0;


    for(int i = -radius; i <= radius; ++i) {
        float weight = gaussian(float(i), sigma);
        vec2 offset = tex_offset * float(i);
        result += texture(tex, v_tex_coords + offset) * weight;
        weight_sum += weight;
    }

    color = result / weight_sum;
}
"#;

const FILM_GRAIN_FRAGMENT_SHADER: &str = r#"
#version 140
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;
uniform float strength;
uniform float seed;
uniform vec2 screen_size;


float random(vec2 st) {
    return fract(sin(dot(st.xy, vec2(12.9898, 78.233))) * 43758.5453123 + seed);
}

void main() {
    vec4 tex_color = texture(tex, v_tex_coords);


    vec2 st = gl_FragCoord.xy / screen_size.xy;
    float grain = random(st) * 2.0 - 1.0;


    vec3 result = tex_color.rgb + grain * strength;

    color = vec4(result, tex_color.a);
}
"#;

const PIXELATE_FRAGMENT_SHADER: &str = r#"
#version 140
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;
uniform float pixel_size;
uniform vec2 screen_size;

void main() {
    vec2 pixel_coord = floor(v_tex_coords * screen_size / pixel_size) * pixel_size;
    vec2 pixel_uv = pixel_coord / screen_size;
    color = texture(tex, pixel_uv);
}
"#;

const SATURATE_FRAGMENT_SHADER: &str = r#"
#version 140
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;
uniform float saturation;

void main() {
    vec4 tex_color = texture(tex, v_tex_coords);
    float gray = dot(tex_color.rgb, vec3(0.299, 0.587, 0.114));
    color = vec4(mix(vec3(gray), tex_color.rgb, saturation), tex_color.a);
}
"#;

const HUE_ROTATE_FRAGMENT_SHADER: &str = r#"
#version 140
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;
uniform float hue_shift;

vec3 rgb2hsv(vec3 c) {
    vec4 K = vec4(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
    vec4 p = mix(vec4(c.bg, K.wz), vec4(c.gb, K.xy), step(c.b, c.g));
    vec4 q = mix(vec4(p.xyw, c.r), vec4(c.r, p.yzx), step(p.x, c.r));
    float d = q.x - min(q.w, q.y);
    float e = 1.0e-10;
    return vec3(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x);
}

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main() {
    vec4 tex_color = texture(tex, v_tex_coords);
    vec3 hsv = rgb2hsv(tex_color.rgb);
    hsv.x = mod(hsv.x + hue_shift / (2.0 * 3.14159265), 1.0);
    color = vec4(hsv2rgb(hsv), tex_color.a);
}
"#;

const BRIGHTEN_FRAGMENT_SHADER: &str = r#"
#version 140
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;
uniform float brightness;

void main() {
    vec4 tex_color = texture(tex, v_tex_coords);
    color = vec4(tex_color.rgb + brightness, tex_color.a);
}
"#;

const VIGNETTE_FRAGMENT_SHADER: &str = r#"
#version 140
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;
uniform vec4 vignette_color;
uniform float vignette_intensity;
uniform vec2 screen_size;
const float BAYER_8X8[64] = float[64](
     0.0/64.0, 32.0/64.0,  8.0/64.0, 40.0/64.0,  2.0/64.0, 34.0/64.0, 10.0/64.0, 42.0/64.0,
    48.0/64.0, 16.0/64.0, 56.0/64.0, 24.0/64.0, 50.0/64.0, 18.0/64.0, 58.0/64.0, 26.0/64.0,
    12.0/64.0, 44.0/64.0,  4.0/64.0, 36.0/64.0, 14.0/64.0, 46.0/64.0,  6.0/64.0, 38.0/64.0,
    60.0/64.0, 28.0/64.0, 52.0/64.0, 20.0/64.0, 62.0/64.0, 30.0/64.0, 54.0/64.0, 22.0/64.0,
     3.0/64.0, 35.0/64.0, 11.0/64.0, 43.0/64.0,  1.0/64.0, 33.0/64.0,  9.0/64.0, 41.0/64.0,
    51.0/64.0, 19.0/64.0, 59.0/64.0, 27.0/64.0, 49.0/64.0, 17.0/64.0, 57.0/64.0, 25.0/64.0,
    15.0/64.0, 47.0/64.0,  7.0/64.0, 39.0/64.0, 13.0/64.0, 45.0/64.0,  5.0/64.0, 37.0/64.0,
    63.0/64.0, 31.0/64.0, 55.0/64.0, 23.0/64.0, 61.0/64.0, 29.0/64.0, 53.0/64.0, 21.0/64.0
);
void main() {
    vec4 tex_color = texture(tex, v_tex_coords);
    vec2 center = vec2(0.5, 0.5);
    float dist = distance(v_tex_coords, center);
    float outer = 0.5 + (1.0 - vignette_intensity) * 0.5;
    float inner = outer * 0.5;
    float vignette = smoothstep(outer, inner, dist);

    // Blend the two colours at the integer level
    vec4 blended = mix(vignette_color, tex_color, vignette);

    // Quantise to the number of representable steps per channel (8-bit = 255)
    float levels = 255.0;
    vec4 lo = floor(blended * levels) / levels;
    vec4 hi = ceil(blended * levels) / levels;

    // The fractional part is how far between lo and hi we are — dither that remainder
    vec4 frac_part = (blended - lo) * levels;
    ivec2 pixel = ivec2(gl_FragCoord.xy) % 8;
    float threshold = BAYER_8X8[pixel.y * 8 + pixel.x];
    vec4 dithered = mix(lo, hi, step(threshold, frac_part));

    color = dithered;
}
"#;

const BRIGHT_PASS_FRAGMENT_SHADER: &str = r#"
#version 140
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;
uniform float threshold;
uniform float knee;

void main() {
    vec4 tex_color = texture(tex, v_tex_coords);
    float brightness = dot(tex_color.rgb, vec3(0.2126, 0.7152, 0.0722));


    float soft = brightness - threshold + knee;
    soft = clamp(soft, 0.0, 2.0 * knee);
    soft = soft * soft / (4.0 * knee + 0.0001);


    float contribution = max(soft, brightness - threshold);
    contribution = max(contribution, 0.0);

    color = vec4(tex_color.rgb * contribution, tex_color.a);
}
"#;

const BLOOM_COMBINE_FRAGMENT_SHADER: &str = r#"
#version 140
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;
uniform sampler2D bloom_tex;
uniform float intensity;


vec3 ACESFilmicToneMapping(vec3 color) {
    float a = 2.51;
    float b = 0.03;
    float c = 2.43;
    float d = 0.59;
    float e = 0.14;
    return clamp((color*(a*color+b))/(color*(c*color+d)+e), 0.0, 1.0);
}

void main() {
    vec4 original = texture(tex, v_tex_coords);
    vec4 bloom = texture(bloom_tex, v_tex_coords);


    vec3 result = original.rgb + bloom.rgb * intensity;


    result = ACESFilmicToneMapping(result);

    color = vec4(result, original.a);
}
"#;

const CONTRAST_FRAGMENT_SHADER: &str = r#"
#version 140
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;
uniform float contrast;

void main() {
    vec4 tex_color = texture(tex, v_tex_coords);
    color = vec4(((tex_color.rgb - 0.5) * contrast) + 0.5, tex_color.a);
}
"#;

const GRAYSCALE_FRAGMENT_SHADER: &str = r#"
#version 140
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;

void main() {
    vec4 tex_color = texture(tex, v_tex_coords);
    float gray = dot(tex_color.rgb, vec3(0.299, 0.587, 0.114));
    color = vec4(vec3(gray), tex_color.a);
}
"#;

const INVERT_FRAGMENT_SHADER: &str = r#"
#version 140
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;

void main() {
    vec4 tex_color = texture(tex, v_tex_coords);
    color = vec4(1.0 - tex_color.rgb, tex_color.a);
}
"#;

const CHROMATIC_ABERRATION_FRAGMENT_SHADER: &str = r#"
#version 140
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;
uniform float strength;
uniform vec2 screen_size;

void main() {
    vec2 direction = v_tex_coords - vec2(0.5);
    vec2 offset = direction * strength / screen_size;

    float r = texture(tex, v_tex_coords + offset).r;
    float g = texture(tex, v_tex_coords).g;
    float b = texture(tex, v_tex_coords - offset).b;
    float a = texture(tex, v_tex_coords).a;

    color = vec4(r, g, b, a);
}
"#;

const SHARPEN_FRAGMENT_SHADER: &str = r#"
#version 140
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;
uniform float strength;
uniform vec2 screen_size;

void main() {
    vec2 texel_size = 1.0 / screen_size;


    vec4 center = texture(tex, v_tex_coords);
    vec4 top = texture(tex, v_tex_coords + vec2(0.0, -texel_size.y));
    vec4 bottom = texture(tex, v_tex_coords + vec2(0.0, texel_size.y));
    vec4 left = texture(tex, v_tex_coords + vec2(-texel_size.x, 0.0));
    vec4 right = texture(tex, v_tex_coords + vec2(texel_size.x, 0.0));


    vec3 result = center.rgb * (1.0 + 4.0 * strength);
    result -= top.rgb * strength;
    result -= bottom.rgb * strength;
    result -= left.rgb * strength;
    result -= right.rgb * strength;

    color = vec4(result, center.a);
}
"#;

const QUANTIZATION_FRAGMENT_SHADER: &str = r#"
#version 140
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;
uniform float colors_per_channel;

void main() {
    vec4 tex_color = texture(tex, v_tex_coords);

    // Round each channel into steps (e.g. 8.0 levels for retro look)
    vec3 quantized = floor(tex_color.rgb * colors_per_channel) / (colors_per_channel - 1.0);

    color = vec4(quantized, tex_color.a);
}
"#;

const WOBBLY_FRAGMENT_SHADER: &str = r#"
#version 140
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;
uniform float time;
uniform float speed;
uniform float amplitude;
uniform float frequency;

void main() {
    // Displace the UV coordinates based on a sine wave function over space and time
    vec2 uv = v_tex_coords;
    uv.x += sin(uv.y * frequency + time * speed) * amplitude;
    uv.y += cos(uv.x * frequency + time * speed) * amplitude;

    color = texture(tex, uv);
}
"#;

const FISH_EYE_FRAGMENT_SHADER: &str = r#"
#version 140
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;
uniform float strength;

void main() {
    // Map UV coordinates from [0, 1] to [-1, 1]
    vec2 uv = v_tex_coords - vec2(0.5);
    float r = length(uv);

    // Apply barrel distortion formula
    if (strength != 0.0) {
        float distortion = 1.0 + r * r * strength;
        uv *= distortion;
    }

    // Remap coordinates back to [0, 1] range
    uv += vec2(0.5);

    // Output black if the warped texture coordinates are outside the viewport bounds
    if (uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0) {
        color = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        color = texture(tex, uv);
    }
}
"#;

pub fn add_post_processing_effect(effect: PostProcessingEffect) {
    current_render_pipeline().add_effect(effect);
}

pub fn blur_screen(sigma: f32) {
    add_post_processing_effect(PostProcessingEffect::GaussianBlur { sigma });
}

pub fn pixelate_screen(pixel_size: f32) {
    add_post_processing_effect(PostProcessingEffect::Pixelate { pixel_size });
}

pub fn saturate_screen(amount: f32) {
    add_post_processing_effect(PostProcessingEffect::Saturate(amount));
}

pub fn hue_rotate_screen(degrees: f32) {
    add_post_processing_effect(PostProcessingEffect::HueRotate(degrees));
}

pub fn brighten_screen(amount: f32) {
    add_post_processing_effect(PostProcessingEffect::Brighten(amount));
}

pub fn vignette_screen(color: Color, intensity: f32) {
    add_post_processing_effect(PostProcessingEffect::Vignette { color, intensity });
}

pub fn bloom_screen(threshold: f32, knee: f32, intensity: f32, radius: f32, iterations: u32) {
    add_post_processing_effect(PostProcessingEffect::Bloom {
        threshold,
        knee,
        intensity,
        blur_radius: radius,
        iterations,
    });
}

pub fn contrast_screen(amount: f32) {
    add_post_processing_effect(PostProcessingEffect::Contrast(amount));
}

pub fn greyscale_screen() {
    add_post_processing_effect(PostProcessingEffect::Grayscale);
}

pub fn invert_screen() {
    add_post_processing_effect(PostProcessingEffect::Invert);
}

pub fn chromatic_abberation_screen(strength: f32) {
    add_post_processing_effect(PostProcessingEffect::ChromaticAberration { strength });
}

pub fn sharpen_screen(strength: f32) {
    add_post_processing_effect(PostProcessingEffect::Sharpen { strength });
}

pub fn film_grain_screen(strength: f32, seed: f32) {
    add_post_processing_effect(PostProcessingEffect::FilmGrain { strength, seed });
}

pub fn quantization_screen(colors_per_channel: f32) {
    add_post_processing_effect(PostProcessingEffect::Quantization { colors_per_channel });
}

pub fn wobbly_screen(time: f32, speed: f32, amplitude: f32, frequency: f32) {
    add_post_processing_effect(PostProcessingEffect::Wobbly {
        time,
        speed,
        amplitude,
        frequency,
    });
}

pub fn fish_eye_screen(strength: f32) {
    add_post_processing_effect(PostProcessingEffect::FishEye { strength });
}
