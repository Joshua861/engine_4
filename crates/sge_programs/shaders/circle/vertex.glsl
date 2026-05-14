#version 140
in vec2 position;
in vec3 center;
in vec2 radius;
in float outline_thickness;
in vec4 fill_color;
in vec4 outline_color;
in float start_angle;
in float end_angle;
out vec2 v_center;
out vec2 v_radius;
out float v_outline_thickness;
out vec4 v_fill_color;
out vec4 v_outline_color;
out vec2 frag_position;
out float v_start_angle;
out float v_end_angle;
uniform mat4 transform;
void main() {
    float max_radius = max(radius.x, radius.y) + outline_thickness;
    float aa_padding = 2.0;
    vec2 expanded_position = position * (max_radius + aa_padding);
    vec3 scaled_position = vec3(expanded_position, 0.0) + center;
    
    frag_position = scaled_position.xy;
    v_center = center.xy;
    v_radius = radius;
    v_outline_thickness = outline_thickness;
    v_fill_color = fill_color;
    v_outline_color = outline_color;
    v_start_angle = start_angle;
    v_end_angle = end_angle;
    gl_Position = transform * vec4(scaled_position, 1.0);
}
