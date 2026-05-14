#version 140

in vec2 position;

in vec3 center;
in vec2 dimensions;
in int shape_type;
in float corner_radius;
in float shape_params[8];
in int fill_type;
in vec4 fill_color_a;
in vec4 fill_color_b;
in float fill_angle;
in vec2 fill_offset;
in float fill_scale;
in float stroke_width;
in vec4 stroke_color;
in int stroke_type;
in vec2 shadow_offset;
in float shadow_radius;
in vec4 shadow_color;

uniform mat4 transform;

out vec2 v_local_pos;
out vec2 v_dimensions;
flat out int v_shape_type;
flat out float v_corner_radius;
flat out float v_shape_params[8];
flat out int v_fill_type;
out vec4 v_fill_color_a;
out vec4 v_fill_color_b;
flat out float v_fill_angle;
flat out vec2 v_fill_offset;
flat out float v_fill_scale;
flat out float v_stroke_width;
out vec4 v_stroke_color;
flat out int v_stroke_type;
flat out vec2 v_shadow_offset;
flat out float v_shadow_radius;
out vec4 v_shadow_color;

void main() {
    float outer_stroke = (stroke_type == 2 || stroke_type == 3)
        ? stroke_width : 0.0;
    float shadow_reach = shadow_radius + max(abs(shadow_offset.x), abs(shadow_offset.y));
    float padding = max(outer_stroke, shadow_reach) + 2.0;

    vec2 half_size = dimensions + vec2(padding);

    vec2 world_pos = center.xy + position * half_size * 2.0;

    v_local_pos = position * half_size * 2.0;

    v_dimensions = dimensions;
    v_shape_type = shape_type;
    v_corner_radius = corner_radius;
    for (int i = 0; i < 8; i++) v_shape_params[i] = shape_params[i];
    v_fill_type = fill_type;
    v_fill_color_a = fill_color_a;
    v_fill_color_b = fill_color_b;
    v_fill_angle = fill_angle;
    v_fill_offset = fill_offset;
    v_fill_scale = fill_scale;
    v_stroke_width = stroke_width;
    v_stroke_color = stroke_color;
    v_stroke_type = stroke_type;
    v_shadow_offset = shadow_offset;
    v_shadow_radius = shadow_radius;
    v_shadow_color = shadow_color;

    gl_Position = transform * vec4(world_pos, center.z, 1.0);
}
