#version 140
in vec2 position;
in vec3 center;
in vec2 bounding_box;
in vec4 shape_params_a;
in vec4 shape_params_b;
in vec4 fill_color_a;
in vec4 fill_color_b;
in vec4 stroke_color;
in vec4 shadow_color;
// misc_a: rotation, corner_radius, fill_angle, fill_scale
in vec4 misc_a;
// misc_b: stroke_width, shadow_radius, fill_offset.x, fill_offset.y
in vec4 misc_b;
// misc_c: shadow_offset.x, shadow_offset.y, fill_type, stroke_type
in vec4 misc_c;
// misc_d: shape_type, 0, 0, 0
in vec4 misc_d;

uniform mat4 transform;

out vec2 v_local_pos;
out vec2 v_dimensions;
flat out float v_rotation;
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
    float rotation = misc_a.x;
    float corner_radius = misc_a.y;
    float fill_angle = misc_a.z;
    float fill_scale = misc_a.w;

    float stroke_width = misc_b.x;
    float shadow_radius = misc_b.y;
    vec2 fill_offset = misc_b.zw;

    vec2 shadow_offset = misc_c.xy;
    int fill_type = int(misc_c.z);
    int stroke_type = int(misc_c.w);

    int shape_type = int(misc_d.x);

    int st = stroke_type;
    float outer_stroke = (st == 2 || st == 3) ? stroke_width : 0.0;
    float shadow_reach = shadow_radius + length(shadow_offset);
    float padding = max(outer_stroke, shadow_reach) + 2.0;
    float shape_radius = length(bounding_box);
    vec2 half_size = vec2(shape_radius) + vec2(padding);
    vec2 world_pos = center.xy + position * half_size;
    v_local_pos = position * half_size;

    v_local_pos = position * half_size;
    v_dimensions = bounding_box;
    v_rotation = rotation;
    v_shape_type = shape_type;
    v_corner_radius = corner_radius;
    v_shape_params[0] = shape_params_a.x;
    v_shape_params[1] = shape_params_a.y;
    v_shape_params[2] = shape_params_a.z;
    v_shape_params[3] = shape_params_a.w;
    v_shape_params[4] = shape_params_b.x;
    v_shape_params[5] = shape_params_b.y;
    v_shape_params[6] = shape_params_b.z;
    v_shape_params[7] = shape_params_b.w;
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
