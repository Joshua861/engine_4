#version 140

in vec3 world_position;

out vec4 color;

uniform vec3 camera_pos;
uniform float grid_scale;
uniform float grid_size;
uniform vec4 grid_color_thin;
uniform vec4 grid_color_thick;
uniform vec4 x_axis_color;
uniform vec4 z_axis_color;
uniform float axis_width;

float grid(vec2 pos, float scale) {
    vec2 coord = pos / scale;
    vec2 grid = abs(fract(coord - 0.5) - 0.5) / fwidth(coord);
    float line = min(grid.x, grid.y);
    return 1.0 - min(line, 1.0);
}

void main() {
    vec2 coord = world_position.xz;

    float dist = length(camera_pos - world_position);
    float max_dist = grid_size * 50.0;

    float fade_start = max_dist * 0.05;
    float fade_end = max_dist;
    float fade = 1.0 - smoothstep(fade_start, fade_end, dist);

    if (fade < 0.01) {
        discard;
    }

    float grid_line = grid(coord, grid_scale);

    float thick_grid = grid(coord, grid_scale * 10.0);

    vec4 grid_col = mix(
        grid_color_thin,
        grid_color_thick,
        thick_grid
    );

    float x_axis = step(abs(coord.y), axis_width);
    float z_axis = step(abs(coord.x), axis_width);

    vec4 final_color = grid_col * grid_line;

    if (x_axis > 0.5) {
        final_color = x_axis_color;
    }
    if (z_axis > 0.5) {
        final_color = z_axis_color;
    }

    final_color.a *= fade;

    if (x_axis < 0.5 && z_axis < 0.5) {
        float grid_fade = 1.0 - smoothstep(fade_start * 0.5, fade_end * 0.8, dist);
        final_color.a *= grid_fade;
    }

    if (final_color.a < 0.01) {
        discard;
    }

    color = final_color;
}
