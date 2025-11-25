#version 140
in vec2 position;
in vec3 center;
in vec2 radius;
in float outline_thickness;
in vec4 fill_color;
in vec4 outline_color;

out vec2 v_center;
out vec2 v_radius;
out float v_outline_thickness;
out vec4 v_fill_color;
out vec4 v_outline_color;
out vec2 frag_position;

uniform mat4 transform;

void main() {
    float max_radius = max(radius.x, radius.y) + outline_thickness;
    vec3 scaled_position = vec3(position * max_radius, 0.0) + center;
    frag_position = scaled_position.xy;

    v_center = center.xy;
    v_radius = radius;
    v_outline_thickness = outline_thickness;
    v_fill_color = fill_color;
    v_outline_color = outline_color;

    gl_Position = transform * vec4(scaled_position, 1.0);
}
