#version 140

in vec2 v_center;
in float v_radius;
in vec4 v_color;
in vec2 frag_position;

out vec4 color;

void main() {
    float dist = distance(frag_position, v_center);

    float delta = fwidth(dist);
    float alpha = 1.0 - smoothstep(v_radius - delta, v_radius + delta, dist);

    color = vec4(v_color.rgb, v_color.a * alpha);
}
