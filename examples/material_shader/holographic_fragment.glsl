#version 150

in vec3 v_normal;
in vec3 v_position;
in vec2 v_tex_coords;
in vec3 v_world_position;

out vec4 color;

uniform float time;
uniform vec3 camera_pos;

void main() {
    vec3 normal = normalize(v_normal);
    vec3 view_dir = normalize(camera_pos - v_world_position);

    // Fresnel effect
    float fresnel = pow(1.0 - max(dot(view_dir, normal), 0.0), 2.0);

    // Rainbow effect based on position and time
    vec3 rainbow = vec3(
        sin(v_world_position.x * 2.0 + time),
        sin(v_world_position.y * 2.0 + time + 2.0),
        sin(v_world_position.z * 2.0 + time + 4.0)
    ) * 0.5 + 0.5;

    // Combine fresnel and rainbow
    vec3 hologram = rainbow * fresnel;

    // Add some scanlines
    float scanline = sin(v_tex_coords.y * 800.0) * 0.1 + 0.9;
    hologram *= scanline;

    // Set alpha based on fresnel and some constant
    float alpha = fresnel * 0.7 + 0.3;

    color = vec4(hologram, alpha);
}
