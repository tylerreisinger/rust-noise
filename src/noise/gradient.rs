use std::f64;

use rand;
use rand::distributions::{self, IndependentSample};
use cgmath::{Vector2, Vector3};

use noise::GradientBuilder;

#[derive(Debug, Clone)]
pub struct RandomGradientBuilder2d<R: rand::Rng> {
    rng: R,
    distribution: distributions::Range<f64>,
}

#[derive(Debug, Clone)]
pub struct CubeGradientBuilder2d<R: rand::Rng> {
    rng: R,
    distribution: distributions::Range<u8>,
}

#[derive(Debug, Clone)]
pub struct RandomGradientBuilder3d<R: rand::Rng> {
    rng: R,
    distribution: distributions::Range<f64>,
}

impl<R> RandomGradientBuilder3d<R>
where
    R: rand::Rng,
{
    pub fn new(rng: R) -> RandomGradientBuilder3d<R> {
        RandomGradientBuilder3d {
            rng,
            distribution: distributions::Range::new(0.0, 2.0 * f64::consts::PI),
        }
    }
}

impl<R> GradientBuilder for RandomGradientBuilder3d<R>
where
    R: rand::Rng,
{
    type Output = Vector3<f64>;

    fn make_gradient(&mut self) -> Vector3<f64> {
        let theta = self.distribution.ind_sample(&mut self.rng) / 2.0;
        let phi = self.distribution.ind_sample(&mut self.rng);

        let x = theta.sin() * phi.cos();
        let y = theta.sin() * phi.sin();
        let z = theta.cos();

        Vector3::new(x, y, z)
    }
}

impl<R> RandomGradientBuilder2d<R>
where
    R: rand::Rng,
{
    pub fn new(rng: R) -> RandomGradientBuilder2d<R> {
        RandomGradientBuilder2d {
            rng,
            distribution: distributions::Range::new(0.0, 2.0 * f64::consts::PI),
        }
    }
}

impl<R> CubeGradientBuilder2d<R>
where
    R: rand::Rng,
{
    pub fn new(rng: R) -> CubeGradientBuilder2d<R> {
        CubeGradientBuilder2d {
            rng,
            distribution: distributions::Range::new(0, 4),
        }
    }
}

impl<R> GradientBuilder for CubeGradientBuilder2d<R>
where
    R: rand::Rng,
{
    type Output = Vector2<f64>;

    fn make_gradient(&mut self) -> Vector2<f64> {
        let cube_gradients = [
            Vector2::new(-1.0, -1.0),
            Vector2::new(1.0, -1.0),
            Vector2::new(-1.0, 1.0),
            Vector2::new(1.0, 1.0),
        ];

        let idx = self.distribution.ind_sample(&mut self.rng);

        cube_gradients[idx as usize] / f64::consts::SQRT_2
    }
}

impl<R> GradientBuilder for RandomGradientBuilder2d<R>
where
    R: rand::Rng,
{
    type Output = Vector2<f64>;

    fn make_gradient(&mut self) -> Vector2<f64> {
        let angle = self.distribution.ind_sample(&mut self.rng);
        let (y, x) = angle.sin_cos();

        Vector2::new(x, y)
    }
}
