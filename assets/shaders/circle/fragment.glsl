#version 140
in vec2 v_center;
in vec2 v_radius;
in float v_outline_thickness;
in vec4 v_fill_color;
in vec4 v_outline_color;
in vec2 frag_position;
out vec4 color;

float ellipse_sdf(vec2 pos, vec2 center, vec2 radius) {
    vec2 diff = pos - center;
    vec2 normalized = diff / radius;
    return sqrt(dot(normalized, normalized));
}

void main() {
    float dist = ellipse_sdf(frag_position, v_center, v_radius);
    float outer_dist = ellipse_sdf(frag_position, v_center, v_radius + vec2(v_outline_thickness));
    float edge_width = fwidth(dist) * 0.5;

    float fill_coverage = 1.0 - smoothstep(1.0 - edge_width, 1.0 + edge_width, dist);
    float outer_coverage = 1.0 - smoothstep(1.0 - edge_width, 1.0 + edge_width, outer_dist);
    float outline_coverage = outer_coverage - fill_coverage;

    vec3 final_color = fill_coverage > outline_coverage ? v_fill_color.rgb : v_outline_color.rgb;
    float final_alpha = v_fill_color.a * fill_coverage + v_outline_color.a * outline_coverage;

    color = vec4(final_color, final_alpha);

    if (outer_coverage <= 0.0) {
        discard;
    }
}
