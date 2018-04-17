use std::f64;

pub trait InterpolationFunction {
    fn interpolation_value(&self, t: f64) -> f64;
}

pub trait Lerp {
    fn lerp(x1: Self, x2: Self, pos: f64) -> Self;
}

#[derive(Default, Clone, Debug)]
pub struct Hermite5thOrderInterpolator;
#[derive(Default, Clone, Debug)]
pub struct Hermite3rdOrderInterpolator;
#[derive(Default, Clone, Debug)]
pub struct LinearInterpolator;

impl Hermite5thOrderInterpolator {
    pub fn new() -> Hermite5thOrderInterpolator {
        Hermite5thOrderInterpolator {}
    }
}

impl Hermite3rdOrderInterpolator {
    pub fn new() -> Hermite3rdOrderInterpolator {
        Hermite3rdOrderInterpolator {}
    }
}

impl LinearInterpolator {
    pub fn new() -> LinearInterpolator {
        LinearInterpolator {}
    }
}

impl Lerp for f64 {
    #[inline]
    fn lerp(x1: f64, x2: f64, pos: f64) -> f64 {
        (1.0 - pos) * x1 + x2 * pos
    }
}

impl InterpolationFunction for Hermite5thOrderInterpolator {
    #[inline]
    fn interpolation_value(&self, t: f64) -> f64 {
        t * t * t * (10.0 + (t * (-15.0 + 6.0 * t)))
    }
}

impl InterpolationFunction for Hermite3rdOrderInterpolator {
    #[inline]
    fn interpolation_value(&self, t: f64) -> f64 {
        t * t * (3.0 - 2.0 * t)
    }
}

impl InterpolationFunction for LinearInterpolator {
    #[inline]
    fn interpolation_value(&self, t: f64) -> f64 {
        t
    }
}
