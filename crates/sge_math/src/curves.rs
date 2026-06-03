use sge_vectors::{FloatPow, Vec2};

// thanks to: https://pomax.github.io/bezierinfo/

pub fn point_on_linear_bezier(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    a * (1.0 - t) + b * t
}

pub fn point_on_quadratic_bezier(a: Vec2, b: Vec2, c: Vec2, t: f32) -> Vec2 {
    let m = 1.0 - t;
    let m2 = m.squared();
    let t2 = t.squared();

    let term1 = a.x * m2;
    let term2 = b.x * m * t * 2.0;
    let term3 = c.x * t2;
    let x = term1 + term2 + term3;

    let term1 = a.y * m2;
    let term2 = b.y * m * t * 2.0;
    let term3 = c.y * t2;
    let y = term1 + term2 + term3;

    Vec2::new(x, y)
}

// https://pomax.github.io/bezierinfo/images/chapters/control/be73034ac382b54863c7a18c2932cbbc.svg
pub fn point_on_cubic_bezier(a: Vec2, b: Vec2, c: Vec2, d: Vec2, t: f32) -> Vec2 {
    let m = 1.0 - t;
    let m2 = m.squared();
    let m3 = m.cubed();
    let t2 = t.squared();
    let t3 = t.cubed();

    let term1 = a.x * m3;
    let term2 = b.x * 3.0 * m2 * t;
    let term3 = c.x * 3.0 * m * t2;
    let term4 = d.x * t3;
    let x = term1 + term2 + term3 + term4;

    let term1 = a.y * m3;
    let term2 = b.y * 3.0 * m2 * t;
    let term3 = c.y * 3.0 * m * t2;
    let term4 = d.y * t3;
    let y = term1 + term2 + term3 + term4;

    Vec2::new(x, y)
}

pub fn derivative_of_cubic_bezier(a: Vec2, b: Vec2, c: Vec2, d: Vec2, t: f32) -> Vec2 {
    let a2 = 3.0 * (b - a);
    let b2 = 3.0 * (c - b);
    let c2 = 3.0 * (d - c);
    point_on_quadratic_bezier(a2, b2, c2, t)
}

pub fn derivative_of_quadratic_bezier(a: Vec2, b: Vec2, c: Vec2, t: f32) -> Vec2 {
    let a2 = 2.0 * (b - a);
    let b2 = 2.0 * (c - b);
    point_on_linear_bezier(a2, b2, t)
}

pub fn tangent_to_cubic_bezier(a: Vec2, b: Vec2, c: Vec2, d: Vec2, t: f32) -> Vec2 {
    derivative_of_cubic_bezier(a, b, c, d, t).normalize()
}

pub fn tangent_to_quadratic_bezier(a: Vec2, b: Vec2, c: Vec2, t: f32) -> Vec2 {
    derivative_of_quadratic_bezier(a, b, c, t).normalize()
}

pub fn normal_to_cubic_bezier(a: Vec2, b: Vec2, c: Vec2, d: Vec2, t: f32) -> Vec2 {
    tangent_to_cubic_bezier(a, b, c, d, t).perp()
}

pub fn normal_to_quadratic_bezier(a: Vec2, b: Vec2, c: Vec2, t: f32) -> Vec2 {
    tangent_to_quadratic_bezier(a, b, c, t).perp()
}
