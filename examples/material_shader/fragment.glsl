#version 150

in vec3 v_normal;
out vec4 color;

uniform sampler2D tex;
uniform vec2 screen_size;
uniform vec3 light_pos;
uniform vec4 light_color;

void main() {
    vec2 screen_coord = gl_FragCoord.xy - vec2(0.5, 0.5);
    vec2 tex_coord = screen_coord / screen_size;

    float light_alpha = light_color.w;
    vec4 light_color = vec4(light_color.xyz, 1.0);
    float brightness = dot(normalize(v_normal), normalize(light_pos));

    vec4 dark = texture(tex, tex_coord);
    vec4 bright = light_color;

    color = mix(dark, bright, brightness * light_alpha);
}
