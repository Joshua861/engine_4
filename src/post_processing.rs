use bevy_math::Vec2;
use glium::{Program, Surface, framebuffer::SimpleFrameBuffer, texture::Texture2d, uniform};

use crate::{EngineDisplay, color::Color, get_state, programs::ProgramRef, textures::TextureRef};

#[derive(Clone, Debug)]
pub enum PostProcessingEffect {
    Bloom {
        threshold: f32,
        intensity: f32,
        blur_radius: f32,
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
}

impl PostProcessingEffect {
    pub fn apply<T: Surface>(
        &self,
        source: TextureRef,
        target: &mut T,
        screen_size: Vec2,
    ) -> anyhow::Result<()> {
        let state = get_state();
        let display = &state.display;

        match self {
            Self::GaussianBlur { sigma } => {
                // Two-pass separable Gaussian blur
                let temp_texture = create_temp_texture(display, screen_size)?;
                let mut temp_fb = SimpleFrameBuffer::new(display, &temp_texture)?;

                // Horizontal pass
                let program = get_or_create_gaussian_blur_program();
                let uniforms = uniform! {
                    tex: source.get().gl_texture.sampled(),
                    sigma: *sigma,
                    direction: [1.0f32, 0.0f32],
                    screen_size: [screen_size.x, screen_size.y],
                };
                render_fullscreen_quad(&mut temp_fb, program.get(), &uniforms)?;

                // Vertical pass
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
                intensity,
                blur_radius,
            } => {
                // Step 1: Extract bright areas
                let bright_texture = create_temp_texture(display, screen_size)?;
                let mut bright_fb = SimpleFrameBuffer::new(display, &bright_texture)?;

                let bright_program = get_or_create_bright_pass_program();
                let uniforms = uniform! {
                    tex: source.get().gl_texture.sampled(),
                    threshold: *threshold,
                };
                render_fullscreen_quad(&mut bright_fb, bright_program.get(), &uniforms)?;

                // Step 2: Blur the bright areas (two-pass Gaussian)
                let temp_texture = create_temp_texture(display, screen_size)?;
                let mut temp_fb = SimpleFrameBuffer::new(display, &temp_texture)?;

                let blur_program = get_or_create_gaussian_blur_program();

                // Horizontal blur pass
                let uniforms = uniform! {
                    tex: bright_texture.sampled(),
                    sigma: *blur_radius,
                    direction: [1.0f32, 0.0f32],
                    screen_size: [screen_size.x, screen_size.y],
                };
                render_fullscreen_quad(&mut temp_fb, blur_program.get(), &uniforms)?;

                // Vertical blur pass (back to bright_fb)
                bright_fb.clear_color(0.0, 0.0, 0.0, 0.0);
                let uniforms = uniform! {
                    tex: temp_texture.sampled(),
                    sigma: *blur_radius,
                    direction: [0.0f32, 1.0f32],
                    screen_size: [screen_size.x, screen_size.y],
                };
                render_fullscreen_quad(&mut bright_fb, blur_program.get(), &uniforms)?;

                // Step 3: Combine original + blurred bright areas
                let combine_program = get_or_create_bloom_combine_program();
                let uniforms = uniform! {
                    tex: source.get().gl_texture.sampled(),
                    bloom_tex: bright_texture.sampled(),
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
        }

        Ok(())
    }
}

fn create_temp_texture(display: &EngineDisplay, size: Vec2) -> anyhow::Result<Texture2d> {
    let texture = Texture2d::empty(display, size.x as u32, size.y as u32)?;
    Ok(texture)
}

pub(crate) fn render_fullscreen_quad<T: Surface, U: glium::uniforms::Uniforms>(
    target: &mut T,
    program: &Program,
    uniforms: &U,
) -> anyhow::Result<()> {
    use crate::shapes_2d::QUAD_INDICES;
    use crate::textures::TexturedVertex2D;
    use glium::{IndexBuffer, VertexBuffer};

    let state = get_state();
    let display = &state.display;

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
        ..Default::default()
    };

    target.draw(&vertex_buffer, &index_buffer, program, uniforms, &params)?;

    Ok(())
}

use std::sync::OnceLock;

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

fn get_or_create_gaussian_blur_program() -> &'static ProgramRef {
    GAUSSIAN_BLUR_PROGRAM.get_or_init(|| {
        crate::programs::load_program(POSTPROCESS_VERTEX_SHADER, GAUSSIAN_BLUR_FRAGMENT_SHADER)
            .unwrap()
    })
}

fn get_or_create_pixelate_program() -> &'static ProgramRef {
    PIXELATE_PROGRAM.get_or_init(|| {
        crate::programs::load_program(POSTPROCESS_VERTEX_SHADER, PIXELATE_FRAGMENT_SHADER).unwrap()
    })
}

fn get_or_create_saturate_program() -> &'static ProgramRef {
    SATURATE_PROGRAM.get_or_init(|| {
        crate::programs::load_program(POSTPROCESS_VERTEX_SHADER, SATURATE_FRAGMENT_SHADER).unwrap()
    })
}

fn get_or_create_hue_rotate_program() -> &'static ProgramRef {
    HUE_ROTATE_PROGRAM.get_or_init(|| {
        crate::programs::load_program(POSTPROCESS_VERTEX_SHADER, HUE_ROTATE_FRAGMENT_SHADER)
            .unwrap()
    })
}

fn get_or_create_brighten_program() -> &'static ProgramRef {
    BRIGHTEN_PROGRAM.get_or_init(|| {
        crate::programs::load_program(POSTPROCESS_VERTEX_SHADER, BRIGHTEN_FRAGMENT_SHADER).unwrap()
    })
}

fn get_or_create_vignette_program() -> &'static ProgramRef {
    VIGNETTE_PROGRAM.get_or_init(|| {
        crate::programs::load_program(POSTPROCESS_VERTEX_SHADER, VIGNETTE_FRAGMENT_SHADER).unwrap()
    })
}

fn get_or_create_bright_pass_program() -> &'static ProgramRef {
    BRIGHT_PASS_PROGRAM.get_or_init(|| {
        crate::programs::load_program(POSTPROCESS_VERTEX_SHADER, BRIGHT_PASS_FRAGMENT_SHADER)
            .unwrap()
    })
}

fn get_or_create_bloom_combine_program() -> &'static ProgramRef {
    BLOOM_COMBINE_PROGRAM.get_or_init(|| {
        crate::programs::load_program(POSTPROCESS_VERTEX_SHADER, BLOOM_COMBINE_FRAGMENT_SHADER)
            .unwrap()
    })
}

fn get_or_create_contrast_program() -> &'static ProgramRef {
    CONTRAST_PROGRAM.get_or_init(|| {
        crate::programs::load_program(POSTPROCESS_VERTEX_SHADER, CONTRAST_FRAGMENT_SHADER).unwrap()
    })
}

fn get_or_create_grayscale_program() -> &'static ProgramRef {
    GRAYSCALE_PROGRAM.get_or_init(|| {
        crate::programs::load_program(POSTPROCESS_VERTEX_SHADER, GRAYSCALE_FRAGMENT_SHADER).unwrap()
    })
}

fn get_or_create_invert_program() -> &'static ProgramRef {
    INVERT_PROGRAM.get_or_init(|| {
        crate::programs::load_program(POSTPROCESS_VERTEX_SHADER, INVERT_FRAGMENT_SHADER).unwrap()
    })
}

fn get_or_create_chromatic_aberration_program() -> &'static ProgramRef {
    CHROMATIC_ABERRATION_PROGRAM.get_or_init(|| {
        crate::programs::load_program(
            POSTPROCESS_VERTEX_SHADER,
            CHROMATIC_ABERRATION_FRAGMENT_SHADER,
        )
        .unwrap()
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

// Separable Gaussian blur (optimized)
const GAUSSIAN_BLUR_FRAGMENT_SHADER: &str = r#"
#version 140
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;
uniform float sigma;
uniform vec2 direction; // [1,0] for horizontal, [0,1] for vertical
uniform vec2 screen_size;

const float PI = 3.14159265359;

float gaussian(float x, float sigma) {
    return exp(-(x * x) / (2.0 * sigma * sigma)) / (sqrt(2.0 * PI) * sigma);
}

void main() {
    // Calculate kernel radius (3 sigma covers 99.7% of the distribution)
    int radius = int(ceil(3.0 * sigma));

    vec2 tex_offset = direction / screen_size;
    vec4 result = vec4(0.0);
    float weight_sum = 0.0;

    // Sample along the specified direction
    for(int i = -radius; i <= radius; ++i) {
        float weight = gaussian(float(i), sigma);
        vec2 offset = tex_offset * float(i);
        result += texture(tex, v_tex_coords + offset) * weight;
        weight_sum += weight;
    }

    color = result / weight_sum;
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

void main() {
    vec4 tex_color = texture(tex, v_tex_coords);
    vec2 center = vec2(0.5, 0.5);
    float dist = distance(v_tex_coords, center);
    float vignette = smoothstep(0.8, 0.2 * vignette_intensity, dist);
    color = mix(vignette_color, tex_color, vignette);
}
"#;

// Bright pass for bloom
const BRIGHT_PASS_FRAGMENT_SHADER: &str = r#"
#version 140
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;
uniform float threshold;

void main() {
    vec4 tex_color = texture(tex, v_tex_coords);
    float brightness = dot(tex_color.rgb, vec3(0.2126, 0.7152, 0.0722));

    if (brightness > threshold) {
        // Soft threshold
        float soft = (brightness - threshold) / (1.0 - threshold);
        color = tex_color * soft;
    } else {
        color = vec4(0.0);
    }
}
"#;

// Combine original + bloom
const BLOOM_COMBINE_FRAGMENT_SHADER: &str = r#"
#version 140
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;
uniform sampler2D bloom_tex;
uniform float intensity;

void main() {
    vec4 original = texture(tex, v_tex_coords);
    vec4 bloom = texture(bloom_tex, v_tex_coords);
    color = original + bloom * intensity;
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
