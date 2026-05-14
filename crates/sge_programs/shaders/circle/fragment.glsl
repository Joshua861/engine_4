#version 140

in vec2 v_center;
in vec2 v_radius;
in float v_outline_thickness;
in vec4 v_fill_color;
in vec4 v_outline_color;
in vec2 frag_position;
in float v_start_angle;
in float v_end_angle;

out vec4 color;

const float TAU = 6.283185307179586;

float ellipse_sdf(vec2 p, vec2 r)
{
    p = abs(p);

    float k0 = length(p / r);
    float k1 = length(p / (r * r));

    return k0 * (k0 - 1.0) / k1;
}

float sector_mask(vec2 p, float start, float end)
{
    float angle = atan(p.y, p.x);

    start = mod(start, TAU);
    end   = mod(end, TAU);
    angle = mod(angle, TAU);

    float sweep = mod(end - start, TAU);

    
    if (sweep >= TAU - 0.0001)
        return 1.0;

    float rel = mod(angle - start, TAU);

    
    float d;

    if (rel <= sweep)
    {
        d = -min(rel, sweep - rel);
    }
    else
    {
        d = min(rel - sweep, TAU - rel);
    }

    float radial = max(length(p), 1e-4);
    float pixel_dist = d * radial;

    float aa = fwidth(pixel_dist);

    return 1.0 - smoothstep(0.0, aa, pixel_dist);
}

void main()
{
    vec2 p = frag_position - v_center;

    float sector_coverage =
        sector_mask(p, v_start_angle, v_end_angle);

    bool is_full_circle = (v_start_angle == 0.0 && v_end_angle == 0.0);

    if (sector_coverage <= 0.0 && !is_full_circle)
        discard;

    float dist =
        ellipse_sdf(p, v_radius);

    float outer_dist =
        ellipse_sdf(
            p,
            v_radius + vec2(v_outline_thickness)
        );

    float aa = fwidth(dist);

    float fill_coverage =
        1.0 - smoothstep(0.0, aa, dist);

    float outer_coverage =
        1.0 - smoothstep(0.0, aa, outer_dist);

    float outline_coverage =
        outer_coverage - fill_coverage;

    if (!is_full_circle) {
        fill_coverage *= sector_coverage;
        outline_coverage *= sector_coverage;
        outer_coverage *= sector_coverage;
    }

    vec3 final_rgb =
        (fill_coverage >= outline_coverage)
            ? v_fill_color.rgb
            : v_outline_color.rgb;

    float final_alpha =
        v_fill_color.a * fill_coverage +
        v_outline_color.a * outline_coverage;

    if (final_alpha <= 0.0)
        discard;

    color = vec4(final_rgb, final_alpha);
}
