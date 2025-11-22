#version 150

in vec3 position;
in vec3 normal;

out vec3 v_normal;
out vec3 v_position;
out vec3 v_world_position;

uniform mat4 model_matrix;
uniform mat4 view_proj_matrix;
uniform mat3 normal_matrix;

void main() {
    vec4 world_position = model_matrix * vec4(position, 1.0);
    v_world_position = world_position.xyz;
    v_position = position;
    v_normal = normal_matrix * normal;

    gl_Position = view_proj_matrix * world_position;
}
