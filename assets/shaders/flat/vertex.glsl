#version 140
in vec3 position;
in vec4 color;
out vec4 vertex_color;

uniform mat4 transform;

void main() {
    vertex_color = color;
    gl_Position = transform * vec4(position, 1.0);
}
