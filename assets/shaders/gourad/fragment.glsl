#version 140

in vec3 v_normal;
out vec4 color;
uniform vec3 light_pos;
uniform vec4 dark_color;
uniform vec4 regular_color;

void main() {
    float brightness = dot(normalize(v_normal), normalize(light_pos));
    color = mix(dark_color, regular_color, brightness);
}
