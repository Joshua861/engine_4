use glium::Program;

use crate::EngineDisplay;

pub struct Programs {
    pub circle: Program,
    pub flat: Program,
    pub textured: Program,
}

impl Programs {
    pub fn new(display: &EngineDisplay) -> anyhow::Result<Self> {
        let vertex_shader_src = include_str!("../assets/shaders/flat/vertex.glsl");
        let fragment_shader_src = include_str!("../assets/shaders/flat/fragment.glsl");
        let flat_program =
            Program::from_source(display, vertex_shader_src, fragment_shader_src, None)?;

        let vertex_shader_src = include_str!("../assets/shaders/circle/vertex.glsl");
        let fragment_shader_src = include_str!("../assets/shaders/circle/fragment.glsl");
        let circle_program =
            Program::from_source(display, vertex_shader_src, fragment_shader_src, None)?;

        let vertex_shader_src = include_str!("../assets/shaders/sprite/vertex.glsl");
        let fragment_shader_src = include_str!("../assets/shaders/sprite/fragment.glsl");
        let textured_program =
            Program::from_source(display, vertex_shader_src, fragment_shader_src, None)?;

        Ok(Programs {
            circle: circle_program,
            flat: flat_program,
            textured: textured_program,
        })
    }
}
