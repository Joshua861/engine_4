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

const float BIAS = 0.0001;
const float SMOOTHNESS = 0.5;

float hash(vec2 p) {
    p = fract(p * vec2(234.34, 435.345));
    p += dot(p, p + 34.23);
    return fract(p.x * p.y);
}

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

float sdf_ellipse(in vec2 p, in vec2 ab)
{
    if (ab.x == ab.y) {
        return length(p) - ab.x;
    }

    p = abs(p);
    if (p.x > p.y) {
        p = p.yx;
        ab = ab.yx;
    }
    float l = ab.y * ab.y - ab.x * ab.x;
    float m = ab.x * p.x / l;
    float m2 = m * m;
    float n = ab.y * p.y / l;
    float n2 = n * n;
    float c = (m2 + n2 - 1.0) / 3.0;
    float c3 = c * c * c;
    float q = c3 + m2 * n2 * 2.0;
    float d = c3 + m2 * n2;
    float g = m + m * n2;
    float co;
    if (d < 0.0)
    {
        float h = acos(q / c3) / 3.0;
        float s = cos(h);
        float t = sin(h) * sqrt(3.0);
        float rx = sqrt(-c * (s + t + 2.0) + m2);
        float ry = sqrt(-c * (s - t + 2.0) + m2);
        co = (ry + sign(l) * rx + abs(g) / (rx * ry) - m) / 2.0;
    }
    else
    {
        float h = 2.0 * m * n * sqrt(d);
        float s = sign(q + h) * pow(abs(q + h), 1.0 / 3.0);
        float u = sign(q - h) * pow(abs(q - h), 1.0 / 3.0);
        float rx = -s - u - c * 4.0 + 2.0 * m2;
        float ry = (s - u) * sqrt(3.0);
        float rm = sqrt(rx * rx + ry * ry);
        co = (ry / sqrt(rm - rx) + 2.0 * g / rm - m) / 2.0;
    }
    vec2 r = ab * vec2(co, sqrt(1.0 - co * co));
    return length(r - p) * sign(p.y - r.y);
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

float sdf_quad(in vec2[4] v, in vec2 p)
{
    float d = dot(p - v[0], p - v[0]);
    float s = 1.0;
    for (int i = 0, j = 4 - 1; i < 4; j = i, i++)
    {
        vec2 e = v[j] - v[i];
        vec2 w = p - v[i];
        vec2 b = w - e * clamp(dot(w, e) / dot(e, e), 0.0, 1.0);
        d = min(d, dot(b, b));
        bvec3 c = bvec3(p.y >= v[i].y, p.y < v[j].y, e.x * w.y > e.y * w.x);
        if (all(c) || all(not(c))) s *= -1.0;
    }
    return s * sqrt(d);
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
    float r2 = r / 2.0;
    const vec4 k = vec4(-0.5, 0.8660254038, 0.5773502692, 1.7320508076);
    p = abs(p);
    p -= 2.0 * min(dot(k.xy, p), 0.0) * k.xy;
    p -= 2.0 * min(dot(k.yx, p), 0.0) * k.yx;
    p -= vec2(clamp(p.x, r2 * k.z, r2 * k.w), r2);
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

float sdf_orientedBox(in vec2 p, in vec2 a, in vec2 b, float th)
{
    float l = length(b - a);
    vec2 d = (b - a) / l;
    vec2 q = (p - (a + b) * 0.5);
    q = mat2(d.x, -d.y, d.y, d.x) * q;
    q = abs(q) - vec2(l, th) * 0.5;
    return length(max(q, 0.0)) + min(max(q.x, q.y), 0.0);
}

float sdf_segment(in vec2 p, in vec2 a, in vec2 b)
{
    vec2 pa = p - a, ba = b - a;
    float h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * h);
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

float sdf_cubic_bezier(vec2 pos, vec2 p0, vec2 p1, vec2 p2, vec2 p3)
{
    const int kNum = 48;

    float res = 1e10;
    vec2 a = p0;

    for (int i = 1; i < kNum; i++)
    {
        float t = float(i) / float(kNum - 1);
        float s = 1.0 - t;

        vec2 b =
            p0 * s * s * s +
                p1 * 3.0 * s * s * t +
                p2 * 3.0 * s * t * t +
                p3 * t * t * t;

        vec2 pa = pos - a;
        vec2 ba = b - a;

        float h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);

        float d = length(pa - ba * h);

        res = min(res, d);

        a = b;
    }

    return res;
}

float eval_sdf(vec2 p) {
    float d = 1e9;
    float cr = v_corner_radius;

    if (v_shape_type == 0) {
        // rounded box already handles this correctly
        d = sdf_rounded_box(p, v_dimensions, cr);
    }
    else if (v_shape_type == 1) {
        // shrink ellipse axes by cr, then expand — keeps outer boundary stable
        d = sdf_ellipse(p, max(v_dimensions - cr, vec2(0.001))) - cr;
    }
    else if (v_shape_type == 2) {
        vec2 a = vec2(v_shape_params[0], v_shape_params[1]);
        vec2 b = vec2(v_shape_params[2], v_shape_params[3]);
        vec2 c = vec2(v_shape_params[4], v_shape_params[5]);
        // scale points inward by cr to compensate
        vec2 center = (a + b + c) / 3.0;
        float scale = max(0.0, 1.0 - cr / max(length(a - center), 0.001));
        a = center + (a - center) * scale;
        b = center + (b - center) * scale;
        c = center + (c - center) * scale;
        d = sdf_triangle(p, a, b, c) - cr;
    }
    else if (v_shape_type == 3) {
        vec2 a = vec2(v_shape_params[0], v_shape_params[1]);
        vec2 b = vec2(v_shape_params[2], v_shape_params[3]);
        vec2 c = vec2(v_shape_params[4], v_shape_params[5]);
        vec2 dd = vec2(v_shape_params[6], v_shape_params[7]);
        vec2 center = (a + b + c + dd) / 4.0;
        float scale = max(0.0, 1.0 - cr / max(length(a - center), 0.001));
        a = center + (a - center) * scale;
        b = center + (b - center) * scale;
        c = center + (c - center) * scale;
        dd = center + (dd - center) * scale;
        vec2 points[4] = vec2[4](a, b, c, dd);
        d = sdf_quad(points, p) - cr;
    }
    else if (v_shape_type == 4) {
        float radius = max(v_dimensions.x - cr, 0.001);
        d = sdf_sector(p, radius, v_shape_params[0], v_shape_params[1]) - cr;
    }
    else if (v_shape_type == 5) {
        float radius = v_dimensions.x;
        float thickness = v_shape_params[2];
        d = sdf_ring(p, radius, max(thickness - cr * 2.0, 0.001), v_shape_params[0], v_shape_params[1]) - cr;
    }
    else if (v_shape_type == 6) {
        float mid = (v_shape_params[0] + v_shape_params[1]) * 0.5;
        float h = abs(v_shape_params[1] - v_shape_params[0]) * 0.5;
        float cs = cos(-mid), sn = sin(-mid);
        vec2 rp = vec2(cs * p.x - sn * p.y, sn * p.x + cs * p.y);
        vec2 sc = vec2(sin(h), cos(h));
        rp.x = abs(rp.x);
        float ra = v_dimensions.x;
        float rb = v_shape_params[2] / 4.0;
        d = ((sc.y * rp.x > sc.x * rp.y) ? length(rp - sc * ra) : abs(length(rp) - ra)) - rb;
    }
    else if (v_shape_type == 7) {
        d = sdf_pentagon(p, max(v_dimensions.x - cr, 0.001)) - cr;
    }
    else if (v_shape_type == 8) {
        d = sdf_hexagon(p, max(v_dimensions.x - cr, 0.001)) - cr;
    }
    else if (v_shape_type == 9) {
        d = sdf_octogon(p, max(v_dimensions.x - cr, 0.001)) - cr;
    }
    else if (v_shape_type == 10) {
        d = sdf_hexagram(p, max(v_dimensions.x - cr, 0.001)) - cr;
    }
    else if (v_shape_type == 11) {
        d = sdf_pentagram(p, max(v_dimensions.x - cr, 0.001)) - cr;
    }
    else if (v_shape_type == 12) {
        float n = max(v_shape_params[0], 2.0);
        float m = clamp(v_shape_params[1], 2.0, n);
        d = sdf_star(p, max(v_dimensions.x - cr, 0.001), n, m) - cr;
    }
    else if (v_shape_type == 13) {
        d = sdf_moon(p, v_shape_params[0], max(v_dimensions.x - cr, 0.001), v_shape_params[1]) - cr;
    }
    else if (v_shape_type == 14) {
        float s = v_dimensions.x * 2.0;
        d = sdf_heart(vec2(p.x, -(p.y + s * -0.6)) / s) * s - cr;
        // heart has no simple radius to shrink; offset-only rounding accepted
    }
    else if (v_shape_type == 15) {
        vec2 b = vec2(v_shape_params[0], v_shape_params[1]);
        d = sdf_cross(p, max(b - cr, vec2(0.001)), 0.0) - cr;
    }
    else if (v_shape_type == 16) {
        d = sdf_x(p, v_shape_params[0], cr);
    }
    else if (v_shape_type == 17) {
        vec2 a = vec2(v_shape_params[0], v_shape_params[1]);
        vec2 b = vec2(v_shape_params[2], v_shape_params[3]);
        vec2 c = vec2(v_shape_params[4], v_shape_params[5]);
        d = sdf_quadratic_bezier(p, a, b, c) - cr * 0.5;
    }
    else if (v_shape_type == 18) {
        float s = v_dimensions.x;
        d = sdf_quadratic_circle(p / s) * s - cr;
    } else if (v_shape_type == 19) {
        vec2 a = vec2(v_shape_params[0], v_shape_params[1]);
        vec2 b = vec2(v_shape_params[2], v_shape_params[3]);
        d = sdf_segment(p, a, b) - cr * 0.5;
    } else if (v_shape_type == 20) {
        vec2 a = vec2(v_shape_params[0], v_shape_params[1]);
        vec2 b = vec2(v_shape_params[2], v_shape_params[3]);
        float thickness = v_shape_params[4] - cr * 2.0;
        vec2 center = (a + b) * 0.5;
        d = sdf_orientedBox(p, a, b, thickness) - cr;
    }
    else if (v_shape_type == 21) {
        vec2 a = vec2(v_shape_params[0], v_shape_params[1]);
        vec2 b = vec2(v_shape_params[2], v_shape_params[3]);
        vec2 c = vec2(v_shape_params[4], v_shape_params[5]);
        vec2 d0 = vec2(v_shape_params[6], v_shape_params[7]);
        d = sdf_cubic_bezier(p, a, b, c, d0) - cr * 0.5;
    }

    return d;
}

vec2 rotate2d(vec2 p, float angle) {
    float c = cos(angle), s = sin(angle);
    return vec2(c * p.x - s * p.y, s * p.x + c * p.y);
}

float bayer_dither(vec2 pos) {
    int x = int(mod(pos.x, 4.0));
    int y = int(mod(pos.y, 4.0));
    int index = x + y * 4;
    float table[16];
    table[0] = 0.0 / 16.0;
    table[1] = 8.0 / 16.0;
    table[2] = 2.0 / 16.0;
    table[3] = 10.0 / 16.0;
    table[4] = 12.0 / 16.0;
    table[5] = 4.0 / 16.0;
    table[6] = 14.0 / 16.0;
    table[7] = 6.0 / 16.0;
    table[8] = 3.0 / 16.0;
    table[9] = 11.0 / 16.0;
    table[10] = 1.0 / 16.0;
    table[11] = 9.0 / 16.0;
    table[12] = 15.0 / 16.0;
    table[13] = 7.0 / 16.0;
    table[14] = 13.0 / 16.0;
    table[15] = 5.0 / 16.0;
    return table[index] - 0.5;
}

vec4 eval_fill(vec2 p, vec2 frag_coord, float dist) {
    vec2 uv = p / max(v_dimensions, vec2(0.001));
    float aspect = v_dimensions.x / v_dimensions.y;
    vec2 uv_square = vec2(uv.x * aspect, uv.y);
    vec2 tp = rotate2d(uv_square + v_fill_offset, v_fill_angle) * v_fill_scale;
    // world-like tiling position scaled by fill_scale
    vec2 wp = tp;

    float dither = bayer_dither(frag_coord) / 255.0;

    if (v_fill_type == 0) {
        return v_fill_color_a;
    }
    else if (v_fill_type == 1) {
        vec2 gp = rotate2d(uv + v_fill_offset, v_fill_angle);
        float t = clamp(gp.x * 0.5 + 0.5 + dither, 0.0, 1.0);
        return mix(v_fill_color_a, v_fill_color_b, t);
    }
    else if (v_fill_type == 2) {
        // checker
        int x = int(floor(wp.x + BIAS));
        int y = int(floor(wp.y + BIAS));
        float check = mod(float(x + y), 2.0);
        return mix(v_fill_color_a, v_fill_color_b, check);
    }
    else if (v_fill_type == 3) {
        // horizontal lines
        int y = int(floor(wp.y + BIAS));
        float t = mod(float(y), 2.0);
        return mix(v_fill_color_a, v_fill_color_b, t);
    }
    else if (v_fill_type == 4) {
        // dots
        float scale = 2.0;
        float cell_x = mod(wp.x, scale);
        float cell_y = mod(wp.y, scale);
        float dist = length(vec2(cell_x - scale * 0.5, cell_y - scale * 0.5));
        float inside = step(dist, scale * 0.3);
        return mix(v_fill_color_b, v_fill_color_a, inside);
    }
    else if (v_fill_type == 5) {
        // grid
        float cx = mod(wp.x, 2.0);
        float cy = mod(wp.y, 2.0);
        float line = float(cx < 1.0 || cy < 1.0);
        return mix(v_fill_color_a, v_fill_color_b, line);
    }
    else if (v_fill_type == 6) {
        // waves (from other shader)
        float wave_y = wp.y + sin(wp.x * 3.14159) * 0.5;
        int band = int(floor(wave_y + BIAS));
        float t = mod(float(band), 2.0);
        return mix(v_fill_color_a, v_fill_color_b, t);
    }
    else if (v_fill_type == 7) {
        // concentric rings
        float r = length(uv_square) * v_fill_scale;
        float t = step(0.5, fract(r));
        return mix(v_fill_color_a, v_fill_color_b, t);
    }
    else if (v_fill_type == 8) {
        // radial gradient
        vec2 gp = rotate2d(uv + v_fill_offset, v_fill_angle);
        float r = length(gp);
        float t = clamp(r + dither, 0.0, 1.0);
        return mix(v_fill_color_a, v_fill_color_b, t);
    }
    else if (v_fill_type == 9) {
        // cross hatch
        float d1 = (wp.x - wp.y);
        float d2 = (wp.x + wp.y);
        float line = float(mod(floor(d1 + BIAS), 2.0) == 0.0 || mod(floor(d2 + BIAS), 2.0) == 0.0);
        return mix(v_fill_color_b, v_fill_color_a, line);
    }
    else if (v_fill_type == 10) {
        // sparse dots
        float scale = 2.0;
        int cx = int(floor(wp.x / scale + BIAS));
        int cy = int(floor(wp.y / scale + BIAS));
        float cell_x = mod(wp.x, scale);
        float cell_y = mod(wp.y, scale);
        float dist = length(vec2(cell_x - scale * 0.5, cell_y - scale * 0.5));
        float inside = float(mod(float(cx + cy), 2.0) == 0.0 && dist < scale * 0.3);
        return mix(v_fill_color_b, v_fill_color_a, inside);
    }
    else if (v_fill_type == 11) {
        // bricks
        float brick_w = 4.0;
        float brick_h = 2.0;
        int row = int(floor(wp.y / brick_h + BIAS));
        float offset = mod(float(row), 2.0) == 0.0 ? 0.0 : 1.0;
        float cell_x = mod(wp.x + offset, brick_w);
        float cell_y = mod(wp.y, brick_h);
        float thickness = 0.4;
        float line = float(cell_x < thickness || cell_y < thickness);
        return mix(v_fill_color_b, v_fill_color_a, line);
    }
    else if (v_fill_type == 12) {
        // herringbone
        int cx = int(floor(wp.x + BIAS));
        int cy = int(floor(wp.y + BIAS));
        float lx = fract(wp.x);
        float ly = fract(wp.y);
        bool colored;
        if (mod(float(cx + cy), 2.0) == 0.0) {
            colored = ly < 0.5;
        } else {
            colored = lx < 0.5;
        }
        return colored ? v_fill_color_a : v_fill_color_b;
    }
    else if (v_fill_type == 13) {
        // triangles
        int cx = int(floor(wp.x + BIAS));
        int cy = int(floor(wp.y + BIAS));
        float lx = fract(wp.x);
        float ly = fract(wp.y);
        bool flip = mod(float(cx + cy), 2.0) == 0.0;
        bool upper = flip ? (lx + ly < 1.0) : (lx < ly);
        return upper ? v_fill_color_a : v_fill_color_b;
    }
    else if (v_fill_type == 14) {
        // concentric squares
        float dist = max(abs(uv_square.x), abs(uv_square.y)) * v_fill_scale;
        float t = step(0.5, fract(dist));
        return mix(v_fill_color_a, v_fill_color_b, t);
    }
    else if (v_fill_type == 15) {
        // textured
        int cx = int(floor(wp.x + BIAS));
        int cy = int(floor(wp.y + BIAS));
        float lx = fract(wp.x);
        float ly = fract(wp.y);
        float thickness = 0.2;
        bool colored;
        if (mod(float(cx + cy), 2.0) == 0.0) {
            colored = ly > 0.5 - thickness && ly < 0.5 + thickness;
        } else {
            colored = lx > 0.5 - thickness && lx < 0.5 + thickness;
        }
        return colored ? v_fill_color_a : v_fill_color_b;
    }
    else if (v_fill_type == 16) {
        // truchet
        float cx = floor(wp.x + BIAS);
        float cy = floor(wp.y + BIAS);
        float lx = fract(wp.x);
        float ly = fract(wp.y);
        float h = hash(vec2(cx, cy));
        float thickness = 0.2;
        float dist;
        if (h < 0.5) {
            dist = min(length(vec2(lx, ly)), length(vec2(lx - 1.0, ly - 1.0)));
        } else {
            dist = min(length(vec2(lx - 1.0, ly)), length(vec2(lx, ly - 1.0)));
        }
        float line = float(abs(dist - 0.5) < thickness * 0.5);
        return mix(v_fill_color_b, v_fill_color_a, line);
    }
    else if (v_fill_type == 17) {
        // random tiles
        float cx = floor(wp.x + BIAS);
        float cy = floor(wp.y + BIAS);
        float t = step(0.5, hash(vec2(cx, cy)));
        return mix(v_fill_color_a, v_fill_color_b, t);
    }
    else if (v_fill_type == 18) {
        // diagonal waves
        float diag = wp.x + wp.y;
        float perp = wp.x - wp.y;
        float wave = diag + sin(perp * 3.14159 * 0.5) * 0.8;
        float t = mod(floor(wave + BIAS), 2.0);
        return mix(v_fill_color_a, v_fill_color_b, t);
    }
    else if (v_fill_type == 19) {
        // topology
        float d = wp.x + wp.y;
        float perp = wp.x - wp.y;
        float wobble = sin(perp * 0.3) * 1.8
                + sin(perp * 0.7 + 1.4) * 0.9
                + sin(perp * 1.7 + 2.8) * 0.35;
        float stripe = sin((d + wobble) * 3.14159);
        return stripe > 0.25 ? v_fill_color_a : v_fill_color_b;
    }
    else if (v_fill_type == 20) {
        // zebra
        float scale = 2.0;
        float d = (wp.x + wp.y) / scale;
        float perp = (wp.x - wp.y) / scale;
        float stripe_id = floor(d);
        float r1 = hash(vec2(stripe_id, 0.0));
        float r2 = hash(vec2(stripe_id, 1.0));
        float r3 = hash(vec2(stripe_id + 1.0, 0.0));
        float wobble = sin(perp * 0.4 + r1 * 6.28) * (0.15 + r2 * 0.25)
                + sin(perp * 0.15 + r3 * 6.28) * 0.2;
        float local = fract(d + wobble);
        float width = 0.55 + r1 * 0.2;
        return local < width ? v_fill_color_a : v_fill_color_b;
    }
    else if (v_fill_type == 21) {
        // fish scales
        float scale = 2.0;
        float row_h = scale * 0.7;
        float row = floor(wp.y / row_h + BIAS);
        float x_off = mod(row, 2.0) * scale * 0.5;
        float col = floor((wp.x + x_off) / scale + BIAS);
        vec2 center = vec2(col * scale - x_off + scale * 0.5, row * row_h + row_h * 0.5);
        vec2 lp = wp - center;
        float dist = length(lp);
        if (dist > scale * 0.5) return v_fill_color_b;
        float ring = mod(floor(dist / (scale * 0.18) + BIAS), 2.0);
        return ring == 0.0 ? v_fill_color_a : v_fill_color_b;
    }
    else if (v_fill_type == 22) {
        // maze
        float cx = floor(wp.x + BIAS);
        float cy = floor(wp.y + BIAS);
        float lx = fract(wp.x);
        float ly = fract(wp.y);
        bool in_right = lx > 0.85;
        bool in_bottom = ly > 0.85;
        bool in_left = lx < 0.15;
        bool in_top = ly < 0.15;
        bool has_right = hash(vec2(cx, cy)) > 0.5;
        bool has_bottom = hash(vec2(cx + 17.3, cy + 3.7)) > 0.5;
        bool nb_right = hash(vec2(cx - 1.0, cy)) > 0.5;
        bool nb_bottom = hash(vec2(cx + 17.3, cy - 1.0 + 3.7)) > 0.5;
        bool wall = (in_right && has_right) || (in_bottom && has_bottom)
                || (in_left && nb_right) || (in_top && nb_bottom);
        return wall ? v_fill_color_a : v_fill_color_b;
    }
    else if (v_fill_type == 23) {
        // moire
        float offset = 8.0;
        vec2 c1 = vec2(0.0);
        vec2 c2 = vec2(offset, offset * 0.3);
        float r1 = sin(length(wp - c1) * 3.14159);
        float r2 = sin(length(wp - c2) * 3.14159);
        return r1 * r2 > 0.0 ? v_fill_color_a : v_fill_color_b;
    }
    else if (v_fill_type == 24) {
        // leopard spots
        float s = 3.0;
        vec2 cell = floor(wp / s);
        vec2 lp = mod(wp, s);
        float min_dist1 = 1e10;
        float min_dist2 = 1e10;
        vec2 nearest = vec2(0.0);
        for (int dy = -1; dy <= 1; dy++) {
            for (int dx = -1; dx <= 1; dx++) {
                vec2 nc = cell + vec2(float(dx), float(dy));
                vec2 point = vec2(hash(nc), hash(nc + vec2(43.7, 91.3))) * s
                        + vec2(float(dx), float(dy)) * s;
                float dist = length(lp - point);
                if (dist < min_dist1) {
                    min_dist2 = min_dist1;
                    min_dist1 = dist;
                    nearest = nc;
                } else if (dist < min_dist2) {
                    min_dist2 = dist;
                }
            }
        }
        float edge = min_dist2 - min_dist1;
        if (edge < s * 0.15) return v_fill_color_a;
        float spot_chance = hash(nearest + vec2(12.4, 56.7));
        return spot_chance > 0.45 ? v_fill_color_a : v_fill_color_b;
    }
    else if (v_fill_type == 25) {
        // rings
        float scale = 2.0;
        float cell_x = mod(wp.x, scale);
        float cell_y = mod(wp.y, scale);
        float dist = length(vec2(cell_x - scale * 0.5, cell_y - scale * 0.5));
        bool inside = dist < scale * 0.2 || dist > scale * 0.4;
        return inside ? v_fill_color_a : v_fill_color_b;
    }
    else if (v_fill_type == 26) {
        // stripes
        float t = step(0.5, fract(dist / v_fill_scale));
        return mix(v_fill_color_a, v_fill_color_b, t);
    }

    return v_fill_color_a;
}

void main() {
    vec2 p = rotate2d(v_local_pos, -v_rotation);
    float dist = eval_sdf(p);
    float aa = fwidth(dist) * 0.5;

    vec4 shadow_layer = vec4(0.0);
    if (v_shadow_color.a > 0.0 && v_shadow_radius > 0.0) {
        float outer_stroke = 0.0;
        if (v_stroke_type == 2) outer_stroke = v_stroke_width;
        else if (v_stroke_type == 3) outer_stroke = v_stroke_width * 0.5;

        vec2 shadow_p = rotate2d(v_local_pos - v_shadow_offset, -v_rotation);
        float shadow_dist = eval_sdf(shadow_p) - outer_stroke;
        float shadow_alpha = smoothstep(v_shadow_radius + aa, -v_shadow_radius - aa, shadow_dist);

        float outside_shape = step(0.0, dist - outer_stroke);
        shadow_alpha *= outside_shape;

        shadow_layer = vec4(v_shadow_color.rgb, v_shadow_color.a * shadow_alpha);
    }

    float fill_mask = smoothstep(aa, -aa, dist);

    vec4 fill_color = eval_fill(p, gl_FragCoord.xy, dist);

    vec4 stroke_layer = vec4(0.0);
    float stroke_mask = 0.0;
    float stroke_dist = 1e6;

    if (v_stroke_width > 0.0 && v_stroke_color.a > 0.0) {
        if (v_stroke_type == 1) {
            // inside
            stroke_dist = abs(dist + v_stroke_width * 0.5) - v_stroke_width * 0.5;
        } else if (v_stroke_type == 2) {
            // outside
            stroke_dist = abs(dist - v_stroke_width * 0.5) - v_stroke_width * 0.5;
        } else if (v_stroke_type == 3) {
            // centered
            stroke_dist = abs(dist) - v_stroke_width * 0.5;
        }

        stroke_mask = smoothstep(aa, -aa, stroke_dist);
    }

    vec3 base_rgb = fill_color.rgb;
    float base_alpha = fill_color.a * fill_mask;

    if (stroke_mask > 0.0) {
        vec3 stroke_rgb = v_stroke_color.rgb;

        float edge_mix = smoothstep(-aa, aa, stroke_dist);

        base_rgb = mix(stroke_rgb, fill_color.rgb, edge_mix);

        base_alpha = max(base_alpha, v_stroke_color.a * stroke_mask);
    }

    vec4 fill_final = vec4(base_rgb * base_alpha, base_alpha);

    vec4 stroke_final =
        vec4(v_stroke_color.rgb * v_stroke_color.a * stroke_mask,
            v_stroke_color.a * stroke_mask);
    vec4 shadow_final = vec4(v_shadow_color.rgb * shadow_layer.a, shadow_layer.a);

    vec4 final_color = vec4(0.0);
    final_color = alpha_blend(final_color, shadow_final);
    final_color = alpha_blend(final_color, fill_final);
    final_color = alpha_blend(final_color, stroke_final);
    frag_color = final_color;

    // frag_color = vec4(1.0, 0.0, 0.0, 1.0);
}
