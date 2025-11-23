#version 140

in vec3 position;
in vec3 normal;
in vec2 tex_coords;

out vec3 world_position;

uniform mat4 model_matrix;
uniform mat4 view_proj_matrix;

void main() {
    vec4 world_pos = model_matrix * vec4(position, 1.0);
    world_position = world_pos.xyz;
    gl_Position = view_proj_matrix * world_pos;
}
