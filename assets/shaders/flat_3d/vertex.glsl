#version 140

in vec3 position;
in vec3 normal;
in vec2 tex_coords;
out vec4 vertex_color;

uniform mat4 model_matrix;
uniform mat4 view_proj_matrix;
uniform vec4 color;

void main() {
    vertex_color = color;
    gl_Position = view_proj_matrix * model_matrix * vec4(position, 1.0);
}
