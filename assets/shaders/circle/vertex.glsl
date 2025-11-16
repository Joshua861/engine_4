#version 140
in vec2 position;
in vec3 center;
in float radius;
in vec4 color;

out vec2 v_center;
out float v_radius;
out vec4 v_color;
out vec2 frag_position;

uniform mat4 transform;

void main() {
    vec3 scaled_position = vec3(position, 0.0) * radius + center;
    frag_position = scaled_position.xy;
    v_center = center.xy;
    v_radius = radius;
    v_color = color;

    gl_Position = transform * vec4(scaled_position, 1.0);
}
