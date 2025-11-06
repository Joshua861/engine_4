#version 140
in vec2 position;
in vec2 center;
in float radius;
in vec4 color;

out vec2 v_center;
out float v_radius;
out vec4 v_color;
out vec2 frag_position;

uniform mat4 transform;

void main() {
    // Scale the unit quad by the radius and translate by the center
    vec2 scaled_position = position * radius + center;
    frag_position = scaled_position;
    v_center = center;
    v_radius = radius;
    v_color = color;

    gl_Position = transform * vec4(scaled_position, 0.0, 1.0);
}
