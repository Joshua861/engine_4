use sge_api::{area::AreaExt, shapes_2d::draw_sdf};
use sge_types::{SdfFill, SdfInstance, SdfStroke};

use super::*;

#[derive(Debug)]
pub struct BoxFill {
    color: Color,
    child: Child,
}

impl BoxFill {
    pub fn new(color: Color, child: Child) -> UiRef {
        Self { color, child }.to_ref()
    }
}

impl UiNode for BoxFill {
    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        area.fill(self.color);
        self.child.draw(area, ui);
        area.size
    }

    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions()
    }

    fn size(&self, area: Area) -> Vec2 {
        self.child.size(area)
    }
}

#[derive(Debug)]
pub struct Fill {
    base: FillStyle,
    hover: FillStyle,
    active: FillStyle,
    child: Child,
}

#[derive(Debug)]
pub struct FillStyle {
    pub fill_color_a: Color,
    pub fill_color_b: Color,
    pub fill_type: SdfFill,
    pub fill_angle: f32,
    pub fill_offset: Vec2,
    pub fill_scale: f32,
    pub stroke_width: f32,
    pub stroke_type: SdfStroke,
    pub stroke_color: Color,
    pub shadow_offset: Vec2,
    pub shadow_radius: f32,
    pub shadow_color: Color,
    pub corner_radius: f32,
}

impl Default for FillStyle {
    fn default() -> Self {
        Self {
            fill_color_a: Color::WHITE,
            fill_color_b: Color::WHITE,
            fill_type: SdfFill::Solid,
            fill_angle: 0.0,
            fill_offset: Vec2::ZERO,
            fill_scale: 1.0,
            shadow_offset: Vec2::ZERO,
            shadow_radius: 0.0,
            shadow_color: Color::BLACK,
            corner_radius: 0.0,
            stroke_width: 0.0,
            stroke_type: SdfStroke::Inside,
            stroke_color: Color::BLACK,
        }
    }
}

#[bon::bon]
impl Fill {
    pub fn new(fill_color: Color, child: Child) -> UiRef {
        Self::builder().color(fill_color).child(child).build()
    }

    pub fn rounded(fill_color: Color, corner_radius: f32, child: Child) -> UiRef {
        Self::builder()
            .color(fill_color)
            .corner_radius(corner_radius)
            .child(child)
            .build()
    }

    pub fn hover(fill_color: Color, hover_color: Color, child: Child) -> UiRef {
        Self::builder()
            .color(fill_color)
            .hover_color(hover_color)
            .child(child)
            .build()
    }

    pub fn rounded_hover(
        fill_color: Color,
        hover_color: Color,
        corner_radius: f32,
        child: Child,
    ) -> UiRef {
        Self::builder()
            .color(fill_color)
            .hover_color(hover_color)
            .corner_radius(corner_radius)
            .child(child)
            .build()
    }

    pub fn gradient(a: Color, b: Color, angle: f32, child: Child) -> UiRef {
        Self::builder()
            .color(a)
            .alt_color(b)
            .pattern(SdfFill::Gradient)
            .pattern_angle(angle)
            .child(child)
            .build()
    }

    pub fn active(base: Color, hover: Color, active: Color, radius: f32, child: Child) -> UiRef {
        Self::builder()
            .color(base)
            .hover_color(hover)
            .active_color(active)
            .corner_radius(radius)
            .child(child)
            .build()
    }

    #[builder]
    pub fn builder(
        color: Color,
        alt_color: Option<Color>,
        pattern: Option<SdfFill>,
        pattern_angle: Option<f32>,
        pattern_offset: Option<Vec2>,
        pattern_scale: Option<f32>,
        shadow_offset: Option<Vec2>,
        shadow_radius: Option<f32>,
        shadow_color: Option<Color>,
        corner_radius: Option<f32>,
        stroke_width: Option<f32>,
        stroke_type: Option<SdfStroke>,
        stroke_color: Option<Color>,

        hover_color: Option<Color>,
        hover_alt_color: Option<Color>,
        hover_pattern: Option<SdfFill>,
        hover_pattern_angle: Option<f32>,
        hover_pattern_offset: Option<Vec2>,
        hover_pattern_scale: Option<f32>,
        hover_shadow_offset: Option<Vec2>,
        hover_shadow_radius: Option<f32>,
        hover_shadow_color: Option<Color>,
        hover_corner_radius: Option<f32>,
        hover_stroke_width: Option<f32>,
        hover_stroke_type: Option<SdfStroke>,
        hover_stroke_color: Option<Color>,

        active_color: Option<Color>,
        active_alt_color: Option<Color>,
        active_pattern: Option<SdfFill>,
        active_pattern_angle: Option<f32>,
        active_pattern_offset: Option<Vec2>,
        active_pattern_scale: Option<f32>,
        active_shadow_offset: Option<Vec2>,
        active_shadow_radius: Option<f32>,
        active_shadow_color: Option<Color>,
        active_corner_radius: Option<f32>,
        active_stroke_width: Option<f32>,
        active_stroke_type: Option<SdfStroke>,
        active_stroke_color: Option<Color>,

        child: Child,
    ) -> UiRef {
        let base = FillStyle {
            fill_color_a: color,
            fill_color_b: alt_color.unwrap_or(Color::WHITE),
            fill_type: pattern.unwrap_or(SdfFill::Solid),
            fill_angle: pattern_angle.unwrap_or(0.0),
            fill_offset: pattern_offset.unwrap_or(Vec2::ZERO),
            fill_scale: pattern_scale.unwrap_or(1.0),
            shadow_offset: shadow_offset.unwrap_or(Vec2::ZERO),
            shadow_radius: shadow_radius.unwrap_or(0.0),
            shadow_color: shadow_color.unwrap_or(Color::BLACK),
            corner_radius: corner_radius.unwrap_or(0.0),
            stroke_width: stroke_width.unwrap_or(0.0),
            stroke_type: stroke_type.unwrap_or(SdfStroke::Inside),
            stroke_color: stroke_color.unwrap_or(Color::BLACK),
        };

        let hover = FillStyle {
            fill_color_a: hover_color.unwrap_or(base.fill_color_a),
            fill_color_b: hover_alt_color.unwrap_or(base.fill_color_b),
            fill_type: hover_pattern.unwrap_or(base.fill_type),
            fill_angle: hover_pattern_angle.unwrap_or(base.fill_angle),
            fill_offset: hover_pattern_offset.unwrap_or(base.fill_offset),
            fill_scale: hover_pattern_scale.unwrap_or(base.fill_scale),
            shadow_offset: hover_shadow_offset.unwrap_or(base.shadow_offset),
            shadow_radius: hover_shadow_radius.unwrap_or(base.shadow_radius),
            shadow_color: hover_shadow_color.unwrap_or(base.shadow_color),
            corner_radius: hover_corner_radius.unwrap_or(base.corner_radius),
            stroke_width: hover_stroke_width.unwrap_or(base.stroke_width),
            stroke_type: hover_stroke_type.unwrap_or(base.stroke_type),
            stroke_color: hover_stroke_color.unwrap_or(base.stroke_color),
        };

        let active = FillStyle {
            fill_color_a: active_color.unwrap_or(hover.fill_color_a),
            fill_color_b: active_alt_color.unwrap_or(hover.fill_color_b),
            fill_type: active_pattern.unwrap_or(hover.fill_type),
            fill_angle: active_pattern_angle.unwrap_or(hover.fill_angle),
            fill_offset: active_pattern_offset.unwrap_or(hover.fill_offset),
            fill_scale: active_pattern_scale.unwrap_or(hover.fill_scale),
            shadow_offset: active_shadow_offset.unwrap_or(hover.shadow_offset),
            shadow_radius: active_shadow_radius.unwrap_or(hover.shadow_radius),
            shadow_color: active_shadow_color.unwrap_or(hover.shadow_color),
            corner_radius: active_corner_radius.unwrap_or(hover.corner_radius),
            stroke_width: active_stroke_width.unwrap_or(hover.stroke_width),
            stroke_type: active_stroke_type.unwrap_or(hover.stroke_type),
            stroke_color: active_stroke_color.unwrap_or(hover.stroke_color),
        };

        Self {
            base,
            hover,
            active,

            child,
        }
        .to_ref()
    }
}

impl UiNode for Fill {
    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let style = if ui.is_active(area) {
            &self.active
        } else if ui.is_hovered(area) {
            &self.hover
        } else {
            &self.base
        };

        let sdf = SdfInstance::rect(area.center(), area.size)
            .with_corner_radius(style.corner_radius)
            .with_fill(
                style.fill_color_a,
                style.fill_color_b,
                style.fill_angle,
                style.fill_scale,
                style.fill_type,
            )
            .with_shadow(style.shadow_offset, style.shadow_radius, style.shadow_color)
            .with_stroke(style.stroke_width, style.stroke_color, style.stroke_type);
        draw_sdf(sdf);

        self.child.draw(area, ui);
        area.size
    }

    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions()
    }

    fn size(&self, area: Area) -> Vec2 {
        self.child.size(area)
    }
}

impl UiRef {
    pub fn fill(self, color: Color) -> UiRef {
        Fill::new(color, self)
    }

    pub fn rounded_fill(self, color: Color, radius: f32) -> UiRef {
        Fill::rounded(color, radius, self)
    }
}
