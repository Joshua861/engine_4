use engine_4_macros::gen_ref_type;
use glium::Program;

use crate::{EngineDisplay, EngineStorage, get_state};

macro_rules! include_program_internal {
    ($display: tt, $vertex: literal, $fragment: literal) => {{
        let vertex_shader_src = include_str!($vertex);
        let fragment_shader_src = include_str!($fragment);
        Program::from_source($display, vertex_shader_src, fragment_shader_src, None)
    }};
}

#[macro_export]
macro_rules! include_program {
    ($vertex: literal, $fragment: literal) => {{
        let vertex_shader_src = include_str!($vertex);
        let fragment_shader_src = include_str!($fragment);
        ::engine_4::prelude::load_program(vertex_shader_src, fragment_shader_src)
    }};
}

pub const FLAT_PROGRAM: ProgramRef = ProgramRef(0);
pub const CIRCLE_PROGRAM: ProgramRef = ProgramRef(1);
pub const TEXTURED_PROGRAM: ProgramRef = ProgramRef(2);
pub const FLAT_3D_PROGRAM: ProgramRef = ProgramRef(3);
pub const GOURAUD_3D_PROGRAM: ProgramRef = ProgramRef(4);
pub const TEXTURED_3D_PROGRAM: ProgramRef = ProgramRef(5);
pub const BLINN_PHONG_3D_PROGRAM: ProgramRef = ProgramRef(6);

gen_ref_type!(Program, ProgramRef, programs);

pub(crate) fn init_programs(
    display: &EngineDisplay,
    storage: &mut EngineStorage,
) -> anyhow::Result<()> {
    let program = include_program_internal!(
        display,
        "../assets/shaders/flat/vertex.glsl",
        "../assets/shaders/flat/fragment.glsl"
    )?;
    storage.programs.push(program);

    let program = include_program_internal!(
        display,
        "../assets/shaders/circle/vertex.glsl",
        "../assets/shaders/circle/fragment.glsl"
    )?;
    storage.programs.push(program);

    let program = include_program_internal!(
        display,
        "../assets/shaders/sprite/vertex.glsl",
        "../assets/shaders/sprite/fragment.glsl"
    )?;
    storage.programs.push(program);

    let program = include_program_internal!(
        display,
        "../assets/shaders/flat_3d/vertex.glsl",
        "../assets/shaders/flat_3d/fragment.glsl"
    )?;
    storage.programs.push(program);

    let program = include_program_internal!(
        display,
        "../assets/shaders/gourad/vertex.glsl",
        "../assets/shaders/gourad/fragment.glsl"
    )?;
    storage.programs.push(program);

    let program = include_program_internal!(
        display,
        "../assets/shaders/textured/vertex.glsl",
        "../assets/shaders/textured/fragment.glsl"
    )?;
    storage.programs.push(program);

    let program = include_program_internal!(
        display,
        "../assets/shaders/blinn_phong/vertex.glsl",
        "../assets/shaders/blinn_phong/fragment.glsl"
    )?;
    storage.programs.push(program);

    Ok(())
}

pub fn load_program(vertex: &str, fragment: &str) -> anyhow::Result<ProgramRef> {
    let state = get_state();
    let program = Program::from_source(&state.display, vertex, fragment, None)?;
    let id = state.storage.programs.len();
    state.storage.programs.push(program);
    Ok(ProgramRef(id))
}
