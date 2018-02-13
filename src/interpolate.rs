use std::f64;

pub trait InterpolationFunction {
    fn interpolation_value(&self, t: f64) -> f64;
}

pub trait Lerp {
    fn lerp(x1: Self, x2: Self, pos: f64) -> Self;
}

#[derive(Default, Clone, Debug)]
pub struct ImprovedPerlinInterpolator;
#[derive(Default, Clone, Debug)]
pub struct BasicPerlinInterpolator;
#[derive(Default, Clone, Debug)]
pub struct LinearInterpolator;

impl ImprovedPerlinInterpolator {
    pub fn new() -> ImprovedPerlinInterpolator {
        ImprovedPerlinInterpolator {}
    }
}

impl BasicPerlinInterpolator {
    pub fn new() -> BasicPerlinInterpolator {
        BasicPerlinInterpolator {}
    }
}

impl LinearInterpolator {
    pub fn new() -> LinearInterpolator {
        LinearInterpolator {}
    }
}

impl Lerp for f64 {
    fn lerp(x1: f64, x2: f64, pos: f64) -> f64 {
        (1.0 - pos) * x1 + x2 * pos
    }
}

impl InterpolationFunction for ImprovedPerlinInterpolator {
    fn interpolation_value(&self, t: f64) -> f64 {
        t * t * t * (10.0 + (t * (-15.0 + 6.0 * t)))
    }
}

impl InterpolationFunction for BasicPerlinInterpolator {
    fn interpolation_value(&self, t: f64) -> f64 {
        t * t * (3.0 - 2.0 * t)
    }
}

impl InterpolationFunction for LinearInterpolator {
    fn interpolation_value(&self, t: f64) -> f64 {
        t
    }
}
