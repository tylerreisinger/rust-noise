fn clamp_to_normal(x: f64) -> f64 {
    if x > 1.0 {
        1.0
    } else if x < 0.0 {
        0.0
    } else {
        x
    }
}

#[inline]
pub fn linear_blend(x1: f64, x2: f64, t: f64) -> f64 {
    let x1_n = x1;
    let x2_n = x2;
    let t_n = clamp_to_normal(t);

    x1_n * (1.0 - t_n) + x2_n * t_n
}

#[inline]
pub fn hermite_3rd_order_blend(x1: f64, x2: f64, t: f64) -> f64 {
    let factor = t * t * (3.0 - 2.0 * t);

    linear_blend(x1, x2, factor)
}

#[inline]
pub fn hermite_5th_order_blend(x1: f64, x2: f64, t: f64) -> f64 {
    let factor = t * t * t * (10.0 + (t * (-15.0 + 6.0 * t)));

    linear_blend(x1, x2, factor)
}
