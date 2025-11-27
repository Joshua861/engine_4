#version 140

in vec2 position;
in vec2 tex_coords;

in vec2 instance_position;
in float instance_z;
in vec2 instance_size;

out vec2 v_tex_coords;

uniform mat4 projection;

void main() {
    v_tex_coords = tex_coords;

    vec2 scaled_pos = position * instance_size;
    vec2 world_pos = scaled_pos + instance_position + instance_size;

    gl_Position = projection * vec4(world_pos, instance_z, 1.0);
}
