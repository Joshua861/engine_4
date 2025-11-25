#version 150

in vec3 position;
in vec3 normal;
in vec2 tex_coords;

out vec3 v_normal;
out vec3 v_position;
out vec2 v_tex_coords;
out vec3 v_world_position;

uniform mat4 model_matrix;
uniform mat4 view_proj_matrix;
uniform mat3 normal_matrix;
uniform float time;

void main() {
    vec4 world_position = model_matrix * vec4(position, 1.0);
    v_world_position = world_position.xyz;
    v_position = position;
    v_normal = normal_matrix * normal;
    v_tex_coords = tex_coords;

    // Add a subtle wave effect
    vec3 displaced_position = position + normal * sin(time + position.x * 10.0) * 0.02;
    gl_Position = view_proj_matrix * model_matrix * vec4(displaced_position, 1.0);
}
