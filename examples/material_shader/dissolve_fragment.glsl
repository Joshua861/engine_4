#version 150

in vec3 v_normal;
in vec3 v_position;
in vec2 v_tex_coords;
in vec3 v_world_position;

out vec4 color;

uniform float time;
uniform float dissolve_threshold; // 0.0 to 1.0
uniform vec4 edge_color; // color of the dissolving edge
uniform float edge_width; // width of the edge

// Simple noise function
float random(vec2 st) {
    return fract(sin(dot(st.xy, vec2(12.9898,78.233))) * 43758.5453123);
}

float noise(vec2 st) {
    vec2 i = floor(st);
    vec2 f = fract(st);
    float a = random(i);
    float b = random(i + vec2(1.0, 0.0));
    float c = random(i + vec2(0.0, 1.0));
    float d = random(i + vec2(1.0, 1.0));
    vec2 u = f * f * (3.0 - 2.0 * f);
    return mix(a, b, u.x) + (c - a)* u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}

void main() {
    // Generate noise based on world position and time
    float n = noise(v_world_position.xy * 5.0 + time * 0.5);

    // If noise is below threshold, discard
    if (n < dissolve_threshold) {
        discard;
    }

    // Calculate edge
    float edge = smoothstep(dissolve_threshold, dissolve_threshold + edge_width, n);

    // Base color (white for simplicity)
    vec4 base_color = vec4(1.0);

    // Mix base color with edge color
    vec4 final_color = mix(edge_color, base_color, edge);

    color = vec4(final_color);
}
