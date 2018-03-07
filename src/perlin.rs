use std::f64;

use cgmath::Vector2;
use rand;
use rand::distributions::{self, IndependentSample};

use grid::Grid;
use interpolate::{InterpolationFunction, Lerp};
use noise::Noise;
use octave::{Octave, OctaveNoise};

pub trait GradientBuilder {
    type Output;

    fn make_gradient(&mut self) -> Self::Output;
}

#[derive(Clone, Debug)]
pub struct Perlin<P: InterpolationFunction> {
    grid: Grid<Vector2<f64>>,
    interp: P,
}

#[derive(Debug, Clone)]
pub struct RandomGradientBuilder2d<R: rand::Rng> {
    rng: R,
    distribution: distributions::Range<f64>,
}

impl<P> Perlin<P>
where
    P: InterpolationFunction,
{
    pub fn new<T>(dimensions: (u32, u32), builder: &mut T, interp: P) -> Perlin<P>
    where
        T: GradientBuilder<Output = Vector2<f64>>,
    {
        let (width, height) = dimensions;
        let size = ((width + 1) as usize) * ((height + 1) as usize);

        let data = (0..size).map(|_| builder.make_gradient()).collect();

        Perlin {
            grid: Grid::with_data(width + 1, height + 1, data),
            interp: interp,
        }
    }
}

impl<P> Noise for Perlin<P>
where
    P: InterpolationFunction,
{
    fn value_at(&self, pos: Vector2<f64>) -> f64 {
        let cell_pos = Vector2::new(
            pos.x * f64::from(self.width()),
            pos.y * f64::from(self.height()),
        );
        let x_0 = cell_pos.x as usize;
        let x_1 = cell_pos.x.ceil() as usize;
        let y_0 = cell_pos.y as usize;
        let y_1 = cell_pos.y.ceil() as usize;

        let rel_x = cell_pos.x - cell_pos.x.floor();
        let rel_y = cell_pos.y - cell_pos.y.floor();
        let rel_pos = Vector2::new(rel_x, rel_y);

        let gradients = [
            self.grid[(x_0, y_0)],
            self.grid[(x_1, y_0)],
            self.grid[(x_0, y_1)],
            self.grid[(x_1, y_1)],
        ];
        let rel_points = [
            Vector2::new(0.0, 0.0),
            Vector2::new(1.0, 0.0),
            Vector2::new(0.0, 1.0),
            Vector2::new(1.0, 1.0),
        ];

        let distances = rel_points.iter().map(|x| rel_pos - x);

        let values_iter = distances.zip(gradients.iter()).map(|(d, &g)| d.perp_dot(g));

        let mut values = [0.0; 4];

        //Workaround for no way to `collect()` into an array.
        for (value, distance) in values.iter_mut().zip(values_iter) {
            *value = distance;
        }

        let interp_x = self.interp.interpolation_value(rel_x);
        let interp_y = self.interp.interpolation_value(rel_y);

        let p1 = Lerp::lerp(values[0], values[1], interp_x);
        let p2 = Lerp::lerp(values[2], values[3], interp_x);

        Lerp::lerp(p1, p2, interp_y) / f64::consts::SQRT_2
    }

    fn width(&self) -> u32 {
        self.grid.width() - 1
    }
    fn height(&self) -> u32 {
        self.grid.height() - 1
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

pub fn build_geometric_octaves<P, G>(
    start_dimensions: (u32, u32),
    num_octaves: u32,
    denominator: f64,
    gradient_builder: &mut G,
    interpolator: &P,
) -> OctaveNoise<Perlin<P>>
where
    G: GradientBuilder<Output = Vector2<f64>>,
    P: InterpolationFunction + Clone,
{
    let mut octaves = Vec::with_capacity(num_octaves as usize);
    let amplitude_multiplier: f64 = 1.0
        / (0..num_octaves)
            .map(|x| 1.0 / (denominator * f64::from(x + 1)))
            .sum::<f64>();

    for i in 0..num_octaves {
        let amplitude = (1.0 / (denominator * f64::from(i + 1))) * amplitude_multiplier;
        let octave = Octave::new(
            Perlin::new(
                (start_dimensions.0 * (i + 1), start_dimensions.1 * (i + 1)),
                gradient_builder,
                interpolator.clone(),
            ),
            amplitude,
        );
        octaves.push(octave);
    }

    OctaveNoise::from_octaves(octaves)
}
