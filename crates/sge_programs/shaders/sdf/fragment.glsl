#version 140

in vec2 v_local_pos;
in vec2 v_dimensions;
flat in int v_shape_type;
flat in float v_corner_radius;
flat in float v_rotation;
flat in float v_shape_params[8];
flat in int v_fill_type;
in vec4 v_fill_color_a;
in vec4 v_fill_color_b;
flat in float v_fill_angle;
flat in vec2 v_fill_offset;
flat in float v_fill_scale;
flat in float v_stroke_width;
in vec4 v_stroke_color;
flat in int v_stroke_type;
flat in vec2 v_shadow_offset;
flat in float v_shadow_radius;
in vec4 v_shadow_color;

out vec4 frag_color;

float dot2(vec2 v) {
    return dot(v, v);
}

vec4 alpha_blend(vec4 dst, vec4 src) {
    return src + dst * (1.0 - src.a);
}

float sdf_box(vec2 p, vec2 b) {
    vec2 d = abs(p) - b;
    return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0);
}

float sdf_rounded_box(vec2 p, vec2 b, float r) {
    r = min(r, min(b.x, b.y));
    vec2 q = abs(p) - b + r;
    return length(max(q, 0.0)) + min(max(q.x, q.y), 0.0) - r;
}

float sdf_ellipse(vec2 p, vec2 r)
{
    vec2 q = p / r;
    return (length(q) - 1.0) * min(r.x, r.y);
}

float sdf_triangle(vec2 p, vec2 p0, vec2 p1, vec2 p2) {
    vec2 e0 = p1 - p0, e1 = p2 - p1, e2 = p0 - p2;
    vec2 v0 = p - p0, v1 = p - p1, v2 = p - p2;
    vec2 pq0 = v0 - e0 * clamp(dot(v0, e0) / dot(e0, e0), 0.0, 1.0);
    vec2 pq1 = v1 - e1 * clamp(dot(v1, e1) / dot(e1, e1), 0.0, 1.0);
    vec2 pq2 = v2 - e2 * clamp(dot(v2, e2) / dot(e2, e2), 0.0, 1.0);
    float s = sign(e0.x * e2.y - e0.y * e2.x);
    vec2 d = min(min(
                vec2(dot2(pq0), s * (v0.x * e0.y - v0.y * e0.x)),
                vec2(dot2(pq1), s * (v1.x * e1.y - v1.y * e1.x))),
            vec2(dot2(pq2), s * (v2.x * e2.y - v2.y * e2.x)));
    return -sqrt(d.x) * sign(d.y);
}

float sdf_quad(vec2 p, vec2 a, vec2 b, vec2 c, vec2 dd) {
    vec2 e0 = b - a, e1 = c - b, e2 = dd - c, e3 = a - dd;
    vec2 v0 = p - a, v1 = p - b, v2 = p - c, v3 = p - dd;
    vec2 pq0 = v0 - e0 * clamp(dot(v0, e0) / dot(e0, e0), 0.0, 1.0);
    vec2 pq1 = v1 - e1 * clamp(dot(v1, e1) / dot(e1, e1), 0.0, 1.0);
    vec2 pq2 = v2 - e2 * clamp(dot(v2, e2) / dot(e2, e2), 0.0, 1.0);
    vec2 pq3 = v3 - e3 * clamp(dot(v3, e3) / dot(e3, e3), 0.0, 1.0);
    float d = min(min(min(dot2(pq0), dot2(pq1)), dot2(pq2)), dot2(pq3));
    float s = sign(min(min(
                    e0.x * v0.y - e0.y * v0.x,
                    e1.x * v1.y - e1.y * v1.x),
                min(
                    e2.x * v2.y - e2.y * v2.x,
                    e3.x * v3.y - e3.y * v3.x)));
    return sqrt(d) * s;
}

float sdf_sector(vec2 p, float radius, float angle_start, float angle_end) {
    float mid = (angle_start + angle_end) * 0.5;
    float h = abs(angle_end - angle_start) * 0.5;
    float cs = cos(-mid), sn = sin(-mid);
    p = vec2(cs * p.x - sn * p.y, sn * p.x + cs * p.y);
    vec2 sc = vec2(sin(h), cos(h));
    p.x = abs(p.x);
    float l = length(p) - radius;
    float m = length(p - sc * clamp(dot(p, sc), 0.0, radius));
    return max(l, m * sign(sc.y * p.x - sc.x * p.y));
}

float sdf_ring(vec2 p, float radius, float thickness, float angle_start, float angle_end) {
    bool full_ring = (abs(angle_end - angle_start) >= 6.2831853);
    float base = abs(length(p) - radius) - thickness * 0.5;
    if (full_ring) return base;
    float mid = (angle_start + angle_end) * 0.5;
    float h = abs(angle_end - angle_start) * 0.5;
    float cs = cos(-mid), sn = sin(-mid);
    vec2 rp = vec2(cs * p.x - sn * p.y, sn * p.x + cs * p.y);
    vec2 sc = vec2(sin(h), cos(h));
    rp.x = abs(rp.x);
    float m = length(rp - sc * clamp(dot(rp, sc), 0.0, radius));
    float mask = m * sign(sc.y * rp.x - sc.x * rp.y);
    return max(base, mask);
}

float sdf_pentagon(vec2 p, float r) {
    const vec3 k = vec3(0.809016994, 0.587785252, 0.726542528);
    p.x = abs(p.x);
    p -= 2.0 * min(dot(vec2(-k.x, k.y), p), 0.0) * vec2(-k.x, k.y);
    p -= 2.0 * min(dot(vec2(k.x, k.y), p), 0.0) * vec2(k.x, k.y);
    p -= vec2(clamp(p.x, -r * k.z, r * k.z), r);
    return length(p) * sign(p.y);
}

float sdf_hexagon(vec2 p, float r) {
    const vec3 k = vec3(-0.866025404, 0.5, 0.577350269);
    p = abs(p);
    p -= 2.0 * min(dot(k.xy, p), 0.0) * k.xy;
    p -= vec2(clamp(p.x, -k.z * r, k.z * r), r);
    return length(p) * sign(p.y);
}

float sdf_octogon(vec2 p, float r) {
    const vec3 k = vec3(-0.9238795325, 0.3826834323, 0.4142135623);
    p = abs(p);
    p -= 2.0 * min(dot(vec2(k.x, k.y), p), 0.0) * vec2(k.x, k.y);
    p -= 2.0 * min(dot(vec2(-k.x, k.y), p), 0.0) * vec2(-k.x, k.y);
    p -= vec2(clamp(p.x, -k.z * r, k.z * r), r);
    return length(p) * sign(p.y);
}

float sdf_hexagram(vec2 p, float r) {
    const vec4 k = vec4(-0.5, 0.8660254038, 0.5773502692, 1.7320508076);
    p = abs(p);
    p -= 2.0 * min(dot(k.xy, p), 0.0) * k.xy;
    p -= 2.0 * min(dot(k.yx, p), 0.0) * k.yx;
    p -= vec2(clamp(p.x, r * k.z, r * k.w), r);
    return length(p) * sign(p.y);
}

float sdf_pentagram(vec2 p, float r) {
    const float k1x = 0.809016994;
    const float k2x = 0.309016994;
    const float k1y = 0.587785252;
    const float k2y = 0.951056516;
    const float k1z = 0.726542528;
    const vec2 v1 = vec2(k1x, -k1y);
    const vec2 v2 = vec2(-k1x, -k1y);
    const vec2 v3 = vec2(k2x, -k2y);
    p.x = abs(p.x);
    p -= 2.0 * max(dot(v1, p), 0.0) * v1;
    p -= 2.0 * max(dot(v2, p), 0.0) * v2;
    p.x = abs(p.x);
    p.y -= r;
    return length(p - v3 * clamp(dot(p, v3), 0.0, k1z * r)) * sign(p.y * v3.x - p.x * v3.y);
}

float sdf_star(vec2 p, float r, float n, float m) {
    float an = 3.141593 / n;
    float en = 3.141593 / m;
    vec2 acs = vec2(cos(an), sin(an));
    vec2 ecs = vec2(cos(en), sin(en));
    float bn = mod(atan(p.x, p.y), 2.0 * an) - an;
    p = length(p) * vec2(cos(bn), abs(sin(bn)));
    p -= r * acs;
    p += ecs * clamp(-dot(p, ecs), 0.0, r * acs.y / ecs.y);
    return length(p) * sign(p.x);
}

float sdf_moon(vec2 p, float d, float ra, float rb) {
    p.y = abs(p.y);
    float a = (ra * ra - rb * rb + d * d) / (2.0 * d);
    float b = sqrt(max(ra * ra - a * a, 0.0));
    if (d * (p.x * b - p.y * a) > d * d * max(b - p.y, 0.0))
        return length(p - vec2(a, b));
    return max(length(p) - ra, -(length(p - vec2(d, 0.0)) - rb));
}

float sdf_heart(vec2 p) {
    p.x = abs(p.x);
    if (p.y + p.x > 1.0)
        return sqrt(dot2(p - vec2(0.25, 0.75))) - sqrt(2.0) / 4.0;
    return sqrt(min(dot2(p - vec2(0.0, 1.0)),
            dot2(p - 0.5 * max(p.x + p.y, 0.0)))) * sign(p.x - p.y);
}

float sdf_cross(vec2 p, vec2 b, float r) {
    p = abs(p);
    p = (p.y > p.x) ? p.yx : p.xy;
    vec2 q = p - b;
    float k = max(q.y, q.x);
    vec2 w = (k > 0.0) ? q : vec2(b.y - p.x, -k);
    return sign(k) * length(max(w, 0.0)) + r;
}

float sdf_x(vec2 p, float w, float r) {
    p = abs(p);
    return length(p - min(p.x + p.y, w) * 0.5) - r;
}

float sdf_quadratic_bezier(vec2 pos, vec2 A, vec2 B, vec2 C) {
    vec2 a = B - A;
    vec2 b = A - 2.0 * B + C;
    vec2 c = a * 2.0;
    vec2 d = A - pos;
    float kk = 1.0 / dot(b, b);
    float kx = kk * dot(a, b);
    float ky = kk * (2.0 * dot(a, a) + dot(d, b)) / 3.0;
    float kz = kk * dot(d, a);
    float res;
    float p2 = ky - kx * kx;
    float p3 = p2 * p2 * p2;
    float q = kx * (2.0 * kx * kx - 3.0 * ky) + kz;
    float h = q * q + 4.0 * p3;
    if (h >= 0.0) {
        h = sqrt(h);
        vec2 x = (vec2(h, -h) - q) / 2.0;
        vec2 uv = sign(x) * pow(abs(x), vec2(1.0 / 3.0));
        float t = clamp(uv.x + uv.y - kx, 0.0, 1.0);
        res = dot2(d + (c + b * t) * t);
    } else {
        float z = sqrt(-p2);
        float v = acos(q / (p2 * z * 2.0)) / 3.0;
        float m = cos(v);
        float n = sin(v) * 1.732050808;
        vec3 t = clamp(vec3(m + m, -n - m, n - m) * z - kx, 0.0, 1.0);
        res = min(dot2(d + (c + b * t.x) * t.x),
                dot2(d + (c + b * t.y) * t.y));
    }
    return sqrt(res);
}

float sdf_quadratic_circle(vec2 p) {
    p = abs(p);
    if (p.y > p.x) p = p.yx;
    float a = p.x - p.y;
    float b = p.x + p.y;
    float c = (2.0 * b - 1.0) / 3.0;
    float h = a * a + c * c * c;
    float t;
    if (h >= 0.0) {
        h = sqrt(h);
        t = sign(h - a) * pow(abs(h - a), 1.0 / 3.0) - pow(h + a, 1.0 / 3.0);
    } else {
        float z = sqrt(-c);
        float v = acos(a / (c * z)) / 3.0;
        t = -z * (cos(v) + sin(v) * 1.732050808);
    }
    t *= 0.5;
    vec2 w = vec2(-t, t) + 0.75 - t * t - p;
    return length(w) * sign(a * a * 0.5 + b - 1.5);
}

float eval_sdf(vec2 p) {
    float d = 1e9;
    float cr = v_corner_radius;

    if (v_shape_type == 0) {
        d = sdf_rounded_box(p, v_dimensions, cr);
    }
    else if (v_shape_type == 1) {
        d = sdf_ellipse(p, v_dimensions) - cr;
    }
    else if (v_shape_type == 2) {
        vec2 a = vec2(v_shape_params[0], v_shape_params[1]);
        vec2 b = vec2(v_shape_params[2], v_shape_params[3]);
        vec2 c = vec2(v_shape_params[4], v_shape_params[5]);
        d = sdf_triangle(p, a, b, c) - cr;
    }
    else if (v_shape_type == 3) {
        vec2 a = vec2(v_shape_params[0], v_shape_params[1]);
        vec2 b = vec2(v_shape_params[2], v_shape_params[3]);
        vec2 c = vec2(v_shape_params[4], v_shape_params[5]);
        vec2 dd = vec2(v_shape_params[6], v_shape_params[7]);
        d = sdf_quad(p, a, b, c, dd) - cr;
    }
    else if (v_shape_type == 4) {
        float radius = v_dimensions.x;
        d = sdf_sector(p, radius, v_shape_params[0], v_shape_params[1]) - cr;
    }
    else if (v_shape_type == 5) {
        float radius = v_dimensions.x;
        float thickness = v_shape_params[2];
        d = sdf_ring(p, radius, thickness, v_shape_params[0], v_shape_params[1]) - cr;
    }
    else if (v_shape_type == 6) {
        float mid = (v_shape_params[0] + v_shape_params[1]) * 0.5;
        float h = abs(v_shape_params[1] - v_shape_params[0]) * 0.5;
        float cs = cos(-mid), sn = sin(-mid);
        vec2 rp = vec2(cs * p.x - sn * p.y, sn * p.x + cs * p.y);
        vec2 sc = vec2(sin(h), cos(h));
        rp.x = abs(rp.x);
        float ra = v_dimensions.x;
        float rb = v_shape_params[2];
        d = ((sc.y * rp.x > sc.x * rp.y) ? length(rp - sc * ra) : abs(length(rp) - ra)) - rb - cr;
    }
    else if (v_shape_type == 7) {
        d = sdf_pentagon(p, v_dimensions.x) - cr;
    }
    else if (v_shape_type == 8) {
        d = sdf_hexagon(p, v_dimensions.x) - cr;
    }
    else if (v_shape_type == 9) {
        d = sdf_octogon(p, v_dimensions.x) - cr;
    }
    else if (v_shape_type == 10) {
        d = sdf_hexagram(p, v_dimensions.x) - cr;
    }
    else if (v_shape_type == 11) {
        d = sdf_pentagram(p, v_dimensions.x) - cr;
    }
    else if (v_shape_type == 12) {
        float n = max(v_shape_params[0], 2.0);
        float m = clamp(v_shape_params[1], 2.0, n);
        d = sdf_star(p, v_dimensions.x, n, m) - cr;
    }
    else if (v_shape_type == 13) {
        d = sdf_moon(p, v_shape_params[0], v_dimensions.x, v_shape_params[1]) - cr;
    }
    else if (v_shape_type == 14) {
        float s = v_dimensions.x;
        d = sdf_heart(p / s) * s - cr;
    }
    else if (v_shape_type == 15) {
        vec2 b = vec2(v_shape_params[0], v_shape_params[1]);
        d = sdf_cross(p, b, 0.0) - cr;
    }
    else if (v_shape_type == 16) {
        d = sdf_x(p, v_shape_params[0], cr);
    }
    else if (v_shape_type == 17) {
        vec2 a = vec2(v_shape_params[0], v_shape_params[1]);
        vec2 b = vec2(v_shape_params[2], v_shape_params[3]);
        vec2 c = vec2(v_shape_params[4], v_shape_params[5]);
        d = sdf_quadratic_bezier(p, a, b, c) - cr;
    }
    else if (v_shape_type == 18) {
        float s = v_dimensions.x;
        d = sdf_quadratic_circle(p / s) * s - cr;
    }

    return d;
}

vec2 rotate2d(vec2 p, float angle) {
    float c = cos(angle), s = sin(angle);
    return vec2(c * p.x - s * p.y, s * p.x + c * p.y);
}

vec4 eval_fill(vec2 p) {
    vec2 uv = p / max(v_dimensions, vec2(0.001));
    uv = uv * 0.5 + 0.5;
    vec2 rp = rotate2d(uv + v_fill_offset, v_fill_angle) * v_fill_scale;

    if (v_fill_type == 0) {
        return v_fill_color_a;
    }
    else if (v_fill_type == 1) {
        float t = clamp(rp.x * 0.5 + 0.5, 0.0, 1.0);
        return mix(v_fill_color_a, v_fill_color_b, t);
    }
    else if (v_fill_type == 2) {
        vec2 q = floor(rp);
        float check = mod(q.x + q.y, 2.0);
        return mix(v_fill_color_a, v_fill_color_b, check);
    }
    else if (v_fill_type == 3) {
        float t = step(0.5, fract(rp.y));
        return mix(v_fill_color_a, v_fill_color_b, t);
    }
    else if (v_fill_type == 4) {
        vec2 cell = fract(rp) - 0.5;
        float inside = step(length(cell), 0.35);
        return mix(v_fill_color_b, v_fill_color_a, inside);
    }
    else if (v_fill_type == 5) {
        vec2 f = abs(fract(rp) - 0.5);
        float line = step(0.45, max(f.x, f.y));
        return mix(v_fill_color_a, v_fill_color_b, line);
    }
    else if (v_fill_type == 6) {
        float wave = sin(rp.x * 6.2831853) * 0.5 + 0.5;
        float t = step(wave, fract(rp.y));
        return mix(v_fill_color_a, v_fill_color_b, t);
    }
    else if (v_fill_type == 7) {
        float r = length(uv) * v_fill_scale;
        float t = step(0.5, fract(r));
        return mix(v_fill_color_a, v_fill_color_b, t);
    } else if (v_fill_type == 8) {
        float r = length(uv) * v_fill_scale;
        float t = clamp(r, 0.0, 1.0);
        return mix(v_fill_color_a, v_fill_color_b, t);
    }

    return v_fill_color_a;
}

void main() {
    float aa = fwidth(eval_sdf(rotate2d(v_local_pos, -v_rotation)));
    vec2 p = rotate2d(v_local_pos, -v_rotation);
    float dist = eval_sdf(p);

    vec4 shadow_layer = vec4(0.0);
    if (v_shadow_color.a > 0.0 && v_shadow_radius > 0.0) {
        vec2 shadow_p = rotate2d(v_local_pos - v_shadow_offset, -v_rotation);
        float shadow_dist = eval_sdf(shadow_p);
        float shadow_alpha = smoothstep(v_shadow_radius + aa, -v_shadow_radius - aa, shadow_dist);

        float outside_shape = step(0.0, dist);
        shadow_alpha *= outside_shape;

        shadow_layer = vec4(v_shadow_color.rgb, v_shadow_color.a * shadow_alpha);
    }

    float fill_mask = smoothstep(aa, -aa, dist);
    vec4 fill_color = eval_fill(p);
    fill_color.a *= fill_mask;

    vec4 stroke_layer = vec4(0.0);
    if (v_stroke_width > 0.0 && v_stroke_color.a > 0.0) {
        float stroke_dist;
        if (v_stroke_type == 0) {
            stroke_dist = abs(dist + v_stroke_width * 0.5) - v_stroke_width * 0.5;
        } else if (v_stroke_type == 1) {
            stroke_dist = abs(dist - v_stroke_width * 0.5) - v_stroke_width * 0.5;
        } else {
            stroke_dist = abs(dist) - v_stroke_width * 0.5;
        }
        float stroke_mask = smoothstep(aa, -aa, stroke_dist);
        stroke_layer = vec4(v_stroke_color.rgb, v_stroke_color.a * stroke_mask);
    }

    vec4 fill_final = vec4(fill_color.rgb * fill_color.a, fill_color.a);
    vec4 stroke_final = vec4(v_stroke_color.rgb * stroke_layer.a, stroke_layer.a);
    vec4 shadow_final = vec4(v_shadow_color.rgb * shadow_layer.a, shadow_layer.a);

    vec4 final_color = vec4(0.0);
    final_color = alpha_blend(final_color, shadow_final);
    final_color = alpha_blend(final_color, fill_final);
    final_color = alpha_blend(final_color, stroke_final);

    frag_color = final_color;
}
