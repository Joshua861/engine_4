#version 140
in vec3 v_normal;
in vec3 v_position;
in vec3 v_world_position;
out vec4 color;
uniform vec3 light_pos;
uniform vec4 ambient_color;
uniform vec4 diffuse_color;
uniform vec4 specular_color;
uniform vec4 rim_color;
uniform vec3 camera_pos;

void main() {
    vec3 normal = normalize(v_normal);

    vec3 ambient = ambient_color.rgb;

    vec3 light_dir = normalize(light_pos - v_world_position);
    float diffuse_intensity = max(dot(normal, light_dir), 0.0);
    vec3 diffuse = diffuse_intensity * diffuse_color.rgb;

    vec3 view_dir = normalize(camera_pos - v_world_position);
    vec3 half_dir = normalize(light_dir + view_dir);
    float specular_intensity = pow(max(dot(normal, half_dir), 0.0), 32.0);
    vec3 specular = specular_intensity * specular_color.rgb;

    float rim = 1.0 - max(dot(view_dir, normal), 0.0);
    rim = pow(rim, 3.0);
    vec3 rim_light = rim * (rim_color.xyz * rim_color.w);

    vec3 final_color = ambient + diffuse + specular + rim_light;
    color = vec4(final_color, 1.0);
}
