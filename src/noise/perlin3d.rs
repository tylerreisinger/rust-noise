use interpolate::{InterpolationFunction, Lerp};
use noise::slice;
use grid::Grid3d;
use noise::GradientBuilder;
use noise::Noise;
use noise::octave::{Octave, OctaveNoise};
use rand;
use rand::distributions::{self, IndependentSample};
use std::f64;

use cgmath::{InnerSpace, Vector3};

#[derive(Debug, Clone)]
pub struct Perlin3d<P: InterpolationFunction> {
    grid: Grid3d<Vector3<f64>>,
    interp: P,
}

impl<P> Perlin3d<P>
where
    P: InterpolationFunction,
{
    pub fn new<T>(dimensions: (u32, u32, u32), builder: &mut T, interp: P) -> Perlin3d<P>
    where
        T: GradientBuilder<Output = Vector3<f64>>,
    {
        let (width, height, depth) = dimensions;
        let size = ((width + 1) as usize) * ((height + 1) as usize) * ((depth + 1) as usize);

        let data = (0..size).map(|_| builder.make_gradient()).collect();

        Perlin3d {
            grid: Grid3d::with_data(width + 1, height + 1, depth + 1, data),
            interp: interp,
        }
    }

    pub fn depth(&self) -> u32 {
        self.grid.depth() - 1
    }

    pub fn slice_2d(&self, depth: f64) -> slice::Slice2d<Perlin3d<P>> {
        slice::Slice2d::new(self, depth)
    }
}

impl<P> Noise for Perlin3d<P>
where
    P: InterpolationFunction,
{
    type IndexType = Vector3<f64>;
    type DimType = (u32, u32, u32);

    fn value_at(&self, pos: Vector3<f64>) -> f64 {
        let cell_pos = Vector3::new(
            pos.x * f64::from(self.width()),
            pos.y * f64::from(self.height()),
            pos.z * f64::from(self.depth()),
        );

        let x_0 = cell_pos.x as usize;
        let x_1 = cell_pos.x.ceil() as usize;
        let y_0 = cell_pos.y as usize;
        let y_1 = cell_pos.y.ceil() as usize;
        let z_0 = cell_pos.z as usize;
        let z_1 = cell_pos.z.ceil() as usize;

        let rel_x = cell_pos.x - cell_pos.x.floor();
        let rel_y = cell_pos.y - cell_pos.y.floor();
        let rel_z = cell_pos.z - cell_pos.z.floor();
        let rel_pos = Vector3::new(rel_x, rel_y, rel_z);

        let gradients = [
            self.grid[(x_0, y_0, z_0)],
            self.grid[(x_1, y_0, z_0)],
            self.grid[(x_0, y_1, z_0)],
            self.grid[(x_1, y_1, z_0)],
            self.grid[(x_0, y_0, z_1)],
            self.grid[(x_1, y_0, z_1)],
            self.grid[(x_0, y_1, z_1)],
            self.grid[(x_1, y_1, z_1)],
        ];
        let rel_points = [
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(1.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(1.0, 0.0, 1.0),
            Vector3::new(0.0, 1.0, 1.0),
            Vector3::new(1.0, 1.0, 1.0),
        ];

        let distances = rel_points.iter().map(|x| rel_pos - x);
        let values_iter = distances.zip(gradients.iter()).map(|(d, &g)| d.dot(g));
        let mut values = [0.0; 8];

        for (value, distance) in values.iter_mut().zip(values_iter) {
            *value = distance;
        }

        let interp_x = self.interp.interpolation_value(rel_x);
        let interp_y = self.interp.interpolation_value(rel_y);
        let interp_z = self.interp.interpolation_value(rel_z);

        let p1 = Lerp::lerp(values[0], values[1], interp_x);
        let p2 = Lerp::lerp(values[2], values[3], interp_x);
        let p3 = Lerp::lerp(values[4], values[5], interp_x);
        let p4 = Lerp::lerp(values[6], values[7], interp_x);

        let front_p = Lerp::lerp(p1, p2, interp_y);
        let back_p = Lerp::lerp(p3, p4, interp_y);

        Lerp::lerp(front_p, back_p, interp_z) * f64::consts::SQRT_2
    }

    fn width(&self) -> u32 {
        self.grid.width() - 1
    }
    fn height(&self) -> u32 {
        self.grid.height() - 1
    }
    fn dimensions(&self) -> (u32, u32, u32) {
        (self.width(), self.height(), self.depth())
    }
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

pub fn build_geometric_octaves<P, G>(
    start_dimensions: (u32, u32, u32),
    num_octaves: u32,
    denominator: f64,
    gradient_builder: &mut G,
    interpolator: &P,
) -> OctaveNoise<Perlin3d<P>>
where
    G: GradientBuilder<Output = Vector3<f64>>,
    P: InterpolationFunction + Clone,
{
    let mut octaves = Vec::with_capacity(num_octaves as usize);
    let amplitude_multiplier: f64 = 1.0
        / (0..num_octaves)
            .map(|x| 1.0 / (denominator.powi(x as i32 + 1)))
            .sum::<f64>();

    for i in 0..num_octaves {
        let amplitude = (1.0 / (denominator.powi(i as i32 + 1))) * amplitude_multiplier;
        let octave = Octave::new(
            Perlin3d::new(
                (
                    start_dimensions.0 << i,
                    start_dimensions.1 << i,
                    start_dimensions.2,
                ),
                gradient_builder,
                interpolator.clone(),
            ),
            amplitude,
        );
        octaves.push(octave);
    }

    OctaveNoise::from_octaves(octaves)
}
