#version 150

in vec3 position;
in vec3 normal;

out vec3 v_normal;

uniform mat4 model_matrix;
uniform mat4 view_proj_matrix;
uniform float time;

void main() {
    mat3 normal_matrix = transpose(inverse(mat3(model_matrix)));
    v_normal = normal_matrix * normal;
    vec3 offset = vec3(
        sin(time / 200 + position.x * 100),
        sin(time / 200 + position.y * 100),
        sin(time / 200 + position.z * 100)
    ) / 100.0;

    gl_Position = view_proj_matrix * model_matrix * vec4(position + offset, 1.0);
}

