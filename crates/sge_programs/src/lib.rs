pub use glium::{Program, ProgramCreationError};
use sge_error_union::ErrorUnion;
use sge_macros::gen_ref_type;
use sge_window::get_window_state;

#[derive(ErrorUnion, Debug)]
pub enum LoadProgramError {
    Create(ProgramCreationError),
    Io(std::io::Error),
    Other(&'static str),
}

macro_rules! include_program_internal {
    ($display: tt, $vertex: literal, $fragment: literal) => {{
        let vertex_shader_src = include_str!($vertex);
        let fragment_shader_src = include_str!($fragment);
        Program::from_source($display, vertex_shader_src, fragment_shader_src, None)
            .map(|p| SgeProgram { glium: p })
    }};
}

#[macro_export]
macro_rules! include_program {
    ($vertex: literal, $fragment: literal) => {{
        let vertex_shader_src = include_str!($vertex);
        let fragment_shader_src = include_str!($fragment);
        ::sge::prelude::load_program_sync(vertex_shader_src, fragment_shader_src)
    }};
}

pub const SDF_PROGRAM: ProgramRef = ProgramRef(0);
pub const TEXTURED_PROGRAM: ProgramRef = ProgramRef(1);
pub const FLAT_3D_PROGRAM: ProgramRef = ProgramRef(2);
pub const GOURAUD_3D_PROGRAM: ProgramRef = ProgramRef(3);
pub const TEXTURED_3D_PROGRAM: ProgramRef = ProgramRef(4);
pub const BLINN_PHONG_3D_PROGRAM: ProgramRef = ProgramRef(5);
pub const COPY_PROGRAM: ProgramRef = ProgramRef(6);
pub const METABALL_PROGRAM: ProgramRef = ProgramRef(7);
pub const FLAT_PROGRAM: ProgramRef = ProgramRef(8);
pub const UNRESOLVE_PROGRAM: ProgramRef = ProgramRef(9);

pub struct SgeProgram {
    glium: Program,
}

impl std::ops::Deref for SgeProgram {
    type Target = Program;

    fn deref(&self) -> &Self::Target {
        &self.glium
    }
}

impl std::ops::DerefMut for SgeProgram {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.glium
    }
}

impl From<Program> for SgeProgram {
    fn from(glium: Program) -> Self {
        SgeProgram { glium }
    }
}

impl From<SgeProgram> for Program {
    fn from(program: SgeProgram) -> Self {
        program.glium
    }
}

gen_ref_type!(SgeProgram, ProgramRef, programs);

pub fn init() -> Result<(), ProgramCreationError> {
    init_programs_storage();

    let display = &get_window_state().display;
    let storage = get_programs_state();

    let program = include_program_internal!(
        display,
        "../shaders/sdf/vertex.glsl",
        "../shaders/sdf/fragment.glsl"
    )?;
    storage.push(program);

    let program = include_program_internal!(
        display,
        "../shaders/sprite/vertex.glsl",
        "../shaders/sprite/fragment.glsl"
    )?;
    storage.push(program);

    let program = include_program_internal!(
        display,
        "../shaders/flat_3d/vertex.glsl",
        "../shaders/flat_3d/fragment.glsl"
    )?;
    storage.push(program);

    let program = include_program_internal!(
        display,
        "../shaders/gourad/vertex.glsl",
        "../shaders/gourad/fragment.glsl"
    )?;
    storage.push(program);

    let program = include_program_internal!(
        display,
        "../shaders/textured/vertex.glsl",
        "../shaders/textured/fragment.glsl"
    )?;
    storage.push(program);

    let program = include_program_internal!(
        display,
        "../shaders/blinn_phong/vertex.glsl",
        "../shaders/blinn_phong/fragment.glsl"
    )?;
    storage.push(program);

    let program = include_program_internal!(
        display,
        "../shaders/copy/vertex.glsl",
        "../shaders/copy/fragment.glsl"
    )?;
    storage.push(program);

    let program = include_program_internal!(
        display,
        "../shaders/metaball/vertex.glsl",
        "../shaders/metaball/fragment.glsl"
    )?;
    storage.push(program);

    let program = include_program_internal!(
        display,
        "../shaders/flat/vertex.glsl",
        "../shaders/flat/fragment.glsl"
    )?;
    storage.push(program);

    let program = include_program_internal!(
        display,
        "../shaders/unresolve/vertex.glsl",
        "../shaders/unresolve/fragment.glsl"
    )?;
    storage.push(program);

    log::info!("Initialized shaders (programs)");
    Ok(())
}

pub fn load_program_sync(vertex: &str, fragment: &str) -> Result<ProgramRef, ProgramCreationError> {
    let display = &get_window_state().display;
    let sge: SgeProgram = Program::from_source(display, vertex, fragment, None)?.into();
    Ok(sge.create())
}
