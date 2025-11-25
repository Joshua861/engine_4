#version 150

in vec3 v_normal;
in vec3 v_position;
in vec2 v_tex_coords;
in vec3 v_world_position;

out vec4 color;

uniform vec3 camera_pos;
uniform vec4 outline_color;
uniform float outline_width;

void main() {
    vec3 normal = normalize(v_normal);
    vec3 view_dir = normalize(camera_pos - v_world_position);

    // Calculate the dot product for the outline
    float rim = 1.0 - max(dot(view_dir, normal), 0.0);

    // Create a smooth outline
    float outline = smoothstep(1.0 - outline_width, 1.0, rim);

    // Set the color to the outline color when on the edge, otherwise transparent
    color = vec4(outline_color.rgb, outline);
}
