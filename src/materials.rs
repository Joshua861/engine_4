use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut, Index, IndexMut},
};

use crate::{
    EngineStorage,
    color::Color,
    get_state,
    programs::{
        BLINN_PHONG_3D_PROGRAM, FLAT_3D_PROGRAM, GOURAUD_3D_PROGRAM, ProgramRef,
        TEXTURED_3D_PROGRAM,
    },
    textures::TextureRef,
};
use bevy_math::{Mat3, Mat4, Vec2, Vec3, Vec4};
use glium::uniforms::{SamplerBehavior, UniformValue};

pub const DEFAULT_MATERIAL: MaterialRef = MaterialRef(0);

pub struct Material {
    pub(crate) program: ProgramRef,
    pub(crate) uniforms: BTreeMap<String, UniformData>,
    pub draw_param_overrides: Option<glium::DrawParameters<'static>>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum UniformData {
    Float(f32),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
    Color(Color),
    Texture(TextureRef),
    Mat4(Mat4),
    Mat3(Mat3),
}

impl Material {
    pub fn new(program: ProgramRef) -> Self {
        Self {
            program,
            uniforms: BTreeMap::new(),
            draw_param_overrides: None,
        }
    }

    pub fn create(self) -> MaterialRef {
        let state = get_state();

        let id = state.storage.materials.len();
        let id = MaterialRef(id);
        state.storage.materials.push(self);

        id
    }

    pub fn with_draw_param_overrides(mut self, params: glium::DrawParameters<'static>) -> Self {
        self.draw_param_overrides = Some(params);
        self
    }

    pub fn with_float(mut self, name: impl Into<String>, value: f32) -> Self {
        self.uniforms.insert(name.into(), UniformData::Float(value));
        self
    }

    pub fn with_vec2(mut self, name: impl Into<String>, value: Vec2) -> Self {
        self.uniforms.insert(name.into(), UniformData::Vec2(value));
        self
    }

    pub fn with_vec3(mut self, name: impl Into<String>, value: Vec3) -> Self {
        self.uniforms.insert(name.into(), UniformData::Vec3(value));
        self
    }

    pub fn with_vec4(mut self, name: impl Into<String>, value: Vec4) -> Self {
        self.uniforms.insert(name.into(), UniformData::Vec4(value));
        self
    }

    pub fn with_mat4(mut self, name: impl Into<String>, value: Mat4) -> Self {
        self.uniforms.insert(name.into(), UniformData::Mat4(value));
        self
    }

    pub fn with_mat3(mut self, name: impl Into<String>, value: Mat3) -> Self {
        self.uniforms.insert(name.into(), UniformData::Mat3(value));
        self
    }

    pub fn with_texture(mut self, name: impl Into<String>, texture: TextureRef) -> Self {
        self.uniforms
            .insert(name.into(), UniformData::Texture(texture));
        self
    }

    pub fn with_color(mut self, name: impl Into<String>, color: Color) -> Self {
        self.uniforms.insert(name.into(), UniformData::Color(color));
        self
    }

    // ----------------------------------------------------------------------------------

    pub fn set_draw_param_overrides(&mut self, params: glium::DrawParameters<'static>) {
        self.draw_param_overrides = Some(params);
    }

    pub fn set_float(&mut self, name: impl Into<String>, value: f32) {
        self.uniforms.insert(name.into(), UniformData::Float(value));
    }

    pub fn set_vec2(&mut self, name: impl Into<String>, value: Vec2) {
        self.uniforms.insert(name.into(), UniformData::Vec2(value));
    }

    pub fn set_vec3(&mut self, name: impl Into<String>, value: Vec3) {
        self.uniforms.insert(name.into(), UniformData::Vec3(value));
    }

    pub fn set_vec4(&mut self, name: impl Into<String>, value: Vec4) {
        self.uniforms.insert(name.into(), UniformData::Vec4(value));
    }

    pub fn set_mat4(&mut self, name: impl Into<String>, value: Mat4) {
        self.uniforms.insert(name.into(), UniformData::Mat4(value));
    }

    pub fn set_mat3(&mut self, name: impl Into<String>, value: Mat3) {
        self.uniforms.insert(name.into(), UniformData::Mat3(value));
    }

    pub fn set_texture(&mut self, name: impl Into<String>, texture: TextureRef) {
        self.uniforms
            .insert(name.into(), UniformData::Texture(texture));
    }

    pub fn set_color(&mut self, name: impl Into<String>, color: Color) {
        self.uniforms.insert(name.into(), UniformData::Color(color));
    }

    pub fn get_uniform(&self, name: impl Into<String>) -> Option<UniformData> {
        self.uniforms.get(&name.into()).copied()
    }
}

impl UniformData {
    fn to_gpu<'a>(&'a self) -> UniformValue<'a> {
        match self {
            Self::Float(f) => UniformValue::Float(*f),
            Self::Mat3(m) => UniformValue::Mat3(m.to_cols_array_2d()),
            Self::Mat4(m) => UniformValue::Mat4(m.to_cols_array_2d()),
            Self::Texture(texture) => {
                let texture = texture.get();
                let mut behaviour = SamplerBehavior::default();
                behaviour.magnify_filter = texture.magnify_filter;
                behaviour.minify_filter = texture.minify_filter;

                UniformValue::Texture2d(&texture.gl_texture, Some(behaviour))
            }
            Self::Vec2(v) => UniformValue::Vec2((*v).into()),
            Self::Vec3(v) => UniformValue::Vec3((*v).into()),
            Self::Vec4(v) => UniformValue::Vec4((*v).into()),
            Self::Color(c) => UniformValue::Vec4(c.for_gpu()),
        }
    }
}

impl glium::uniforms::Uniforms for Material {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut add: F) {
        for key in self.uniforms.keys() {
            let value = self.uniforms.get(key).unwrap().to_gpu();
            add(key, value);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MaterialRef(usize);

impl Index<MaterialRef> for EngineStorage {
    type Output = Material;
    fn index(&self, index: MaterialRef) -> &Self::Output {
        &self.materials[index.0]
    }
}

impl IndexMut<MaterialRef> for EngineStorage {
    fn index_mut(&mut self, index: MaterialRef) -> &mut Self::Output {
        &mut self.materials[index.0]
    }
}

impl MaterialRef {
    pub fn get(&self) -> &Material {
        &get_state().storage.materials[self.0]
    }

    pub fn get_mut(&self) -> &mut Material {
        &mut get_state().storage.materials[self.0]
    }
}

impl Deref for MaterialRef {
    type Target = Material;
    fn deref(&self) -> &Self::Target {
        &get_state().storage[*self]
    }
}

impl DerefMut for MaterialRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut get_state().storage[*self]
    }
}

pub fn create_flat_3d_material(color: Color) -> MaterialRef {
    let material = Material::new(FLAT_3D_PROGRAM).with_color("color", color);
    material.create()
}

pub fn create_gouraud_material(
    regular_color: Color,
    dark_color: Color,
    light_pos: Vec3,
) -> MaterialRef {
    let material = Material::new(GOURAUD_3D_PROGRAM)
        .with_color("regular_color", regular_color)
        .with_color("dark_color", dark_color)
        .with_vec3("light_pos", light_pos);

    material.create()
}

pub fn create_textured_material(texture: TextureRef) -> MaterialRef {
    let material = Material::new(TEXTURED_3D_PROGRAM).with_texture("tex", texture);
    material.create()
}

pub fn create_blinn_phong_material(
    ambient: Color,
    diffuse: Color,
    specular: Color,
    rim: Color,
    light_pos: Vec3,
) -> MaterialRef {
    let material = Material::new(BLINN_PHONG_3D_PROGRAM)
        .with_color("ambient_color", ambient)
        .with_color("diffuse_color", diffuse)
        .with_color("specular_color", specular)
        .with_color("rim_color", rim)
        .with_vec3("light_pos", light_pos);
    material.create()
}

pub(crate) fn init_materials(storage: &mut EngineStorage) {
    let regular_color = Color::hex(0xBBBDBD);
    let dark_color = Color::hex(0x333333);
    let light_pos = Vec3::new(0.0, 3.0, 1.0);
    let material = Material::new(GOURAUD_3D_PROGRAM)
        .with_color("regular_color", regular_color)
        .with_color("dark_color", dark_color)
        .with_vec3("light_pos", light_pos);

    storage.materials.push(material);
}
