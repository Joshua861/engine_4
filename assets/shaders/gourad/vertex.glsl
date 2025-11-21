#version 150

in vec3 position;
in vec3 normal;

out vec3 v_normal;

uniform mat4 model_matrix;
uniform mat4 view_proj_matrix;

void main() {
    mat3 normal_matrix = transpose(inverse(mat3(model_matrix)));
    v_normal = normal_matrix * normal;

    gl_Position = view_proj_matrix * model_matrix * vec4(position, 1.0);
}
