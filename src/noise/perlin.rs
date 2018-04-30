use std::f64;

use cgmath::Vector2;
use cgmath::{InnerSpace, Vector3};

use grid::{Grid1d, Grid2d, Grid3d};
use interpolate::{self, InterpolationFunction, Lerp};
use noise::{Noise, Noise1d, Noise2d, Noise3d, Point1, Point2, Point3, TupleUtil, WithFrequency};
use gradient::{GradientFactory, GradientProvider};
use noise::octave::{build_geometric_fractal_noise, OctaveNoise};

pub type DefaultInterpolator = interpolate::Hermite5thOrderInterpolator;

macro_rules! size_tuple_to_frequency {
    ($name:ident, ($($idx:tt),+)) => (
        (
        $(
            f64::from($name.$idx)
        ),*
        )
    );
    ($name:ident) => (
        f64::from($name)
    );
}

#[derive(Clone, Debug)]
pub struct Perlin1d<G, P>
where
    G: GradientProvider<Point1<u32>>,
    P: InterpolationFunction,
{
    frequency: f64,
    gradients: G,
    interp: P,
}
#[derive(Clone, Debug)]
pub struct Perlin2d<G, P>
where
    G: GradientProvider<Point2<u32>>,
    P: InterpolationFunction,
{
    frequency: (f64, f64),
    gradients: G,
    interp: P,
}
#[derive(Debug, Clone)]
pub struct Perlin3d<G, P>
where
    G: GradientProvider<Point3<u32>>,
    P: InterpolationFunction,
{
    frequency: (f64, f64, f64),
    gradients: G,
    interp: P,
}

impl<G> Perlin1d<G, DefaultInterpolator>
where
    G: GradientProvider<Point1<u32>, DimType = u32, Output = f64>,
{
    pub fn new(frequency: f64, gradients: G) -> Perlin1d<G, DefaultInterpolator> {
        if let Some(dim) = gradients.max_dimensions() {
            assert!(frequency <= size_tuple_to_frequency!(dim),
                "The gradient provider has a smaller maximum size than the requested noise frequency.");
        }
        Perlin1d {
            frequency,
            gradients,
            interp: DefaultInterpolator::new(),
        }
    }

    pub fn build_geometric_octaves<F>(
        initial_frequency: <Self as Noise>::DimType,
        num_octaves: u32,
        frequency_scaling: <Self as Noise>::DimType,
        persistance: f64,
        gradient_factory: &mut F,
    ) -> OctaveNoise<Perlin1d<G, DefaultInterpolator>>
    where
        <Self as Noise>::DimType: TupleUtil<f64>,
        F: GradientFactory<f64, f64, Index = Point1<u32>, Output = G>,
    {
        build_geometric_fractal_noise(
            initial_frequency,
            num_octaves,
            frequency_scaling,
            persistance,
            &mut move |n, frequency, _| {
                let g = gradient_factory.build(n, frequency);
                Perlin1d::new(frequency, g)
            },
        )
    }
}

impl Perlin1d<Grid1d<f64>, DefaultInterpolator> {
    pub fn from_grid(grid: Grid1d<f64>) -> Perlin1d<Grid1d<f64>, DefaultInterpolator> {
        Perlin1d {
            frequency: grid.len() as f64,
            gradients: grid,
            interp: DefaultInterpolator::new(),
        }
    }
}

impl<G, P> WithFrequency for Perlin1d<G, P>
where
    G: GradientProvider<Point1<u32>, DimType = u32, Output = f64>,
    P: InterpolationFunction,
{
    fn with_frequency(self, frequency: Self::DimType) -> Self {
        if let Some(dim) = self.gradients.max_dimensions() {
            assert!(frequency <= size_tuple_to_frequency!(dim),
                "The gradient provider has a smaller maximum size than the requested noise frequency.");
        }
        Self {
            frequency: frequency,
            ..self
        }
    }
}

impl<G, P> Perlin1d<G, P>
where
    G: GradientProvider<Point1<u32>, DimType = u32, Output = f64>,
    P: InterpolationFunction,
{
    pub fn with_interpolator<P2>(self, interpolator: P2) -> Perlin1d<G, P2>
    where
        P2: InterpolationFunction,
    {
        Perlin1d {
            interp: interpolator,
            frequency: self.frequency,
            gradients: self.gradients,
        }
    }
}

impl<G, P> Noise for Perlin1d<G, P>
where
    G: GradientProvider<Point1<u32>, Output = f64>,
    P: InterpolationFunction,
{
    type IndexType = Point1<f64>;
    type DimType = f64;

    fn value_at(&self, pos: f64) -> f64 {
        let cell_pos = pos * self.width();
        let rel_pos = cell_pos - cell_pos.floor();

        let x_0 = cell_pos as u32;
        let x_1 = x_0 + 1;

        let gradients = [
            *self.gradients.get_gradient(x_0),
            *self.gradients.get_gradient(x_1),
        ];
        let rel_points = [0.0, 1.0];

        let distances = rel_points.iter().map(|x| rel_pos - x);

        let values_iter = distances.zip(gradients.iter()).map(|(d, &g)| d * g);

        let mut values = [0.0; 2];

        //Workaround for no way to `collect()` into an array.
        for (value, distance) in values.iter_mut().zip(values_iter) {
            *value = distance;
        }

        let interp_coeff = self.interp.interpolation_value(rel_pos);

        Lerp::lerp(values[0], values[1], interp_coeff) * 2.0
    }

    fn frequency(&self) -> f64 {
        self.frequency
    }
}

impl<G> Perlin2d<G, DefaultInterpolator>
where
    G: GradientProvider<Point2<u32>, DimType = (u32, u32), Output = Vector2<f64>>,
{
    pub fn new(frequency: (f64, f64), gradients: G) -> Perlin2d<G, DefaultInterpolator> {
        if let Some(dim) = gradients.max_dimensions() {
            assert!(frequency <= size_tuple_to_frequency!(dim, (0, 1)),
                "The gradient provider has a smaller maximum size than the requested noise frequency.");
        }
        Perlin2d {
            frequency,
            gradients,
            interp: DefaultInterpolator::new(),
        }
    }

    pub fn build_geometric_octaves<F>(
        initial_frequency: <Self as Noise>::DimType,
        num_octaves: u32,
        frequency_scaling: <Self as Noise>::DimType,
        persistance: f64,
        gradient_factory: &mut F,
    ) -> OctaveNoise<Perlin2d<G, DefaultInterpolator>>
    where
        <Self as Noise>::DimType: TupleUtil<f64>,
        F: GradientFactory<Vector2<f64>, (f64, f64), Index = Point2<u32>, Output = G>,
    {
        build_geometric_fractal_noise(
            initial_frequency,
            num_octaves,
            frequency_scaling,
            persistance,
            &mut move |n, frequency, _| {
                let g = gradient_factory.build(n, frequency);
                Perlin2d::new(frequency, g)
            },
        )
    }
}

impl Perlin2d<Grid2d<Vector2<f64>>, DefaultInterpolator> {
    pub fn from_grid(
        grid: Grid2d<Vector2<f64>>,
    ) -> Perlin2d<Grid2d<Vector2<f64>>, DefaultInterpolator> {
        Perlin2d {
            frequency: (grid.width() as f64, grid.height() as f64),
            gradients: grid,
            interp: DefaultInterpolator::new(),
        }
    }
}

impl<G, P> Perlin2d<G, P>
where
    G: GradientProvider<Point2<u32>, DimType = (u32, u32), Output = Vector2<f64>>,
    P: InterpolationFunction,
{
    pub fn with_interpolator<P2>(self, interpolator: P2) -> Perlin2d<G, P2>
    where
        P2: InterpolationFunction,
    {
        Perlin2d {
            interp: interpolator,
            frequency: self.frequency,
            gradients: self.gradients,
        }
    }
}

impl<G, P> WithFrequency for Perlin2d<G, P>
where
    G: GradientProvider<Point2<u32>, DimType = (u32, u32), Output = Vector2<f64>>,
    P: InterpolationFunction,
{
    fn with_frequency(self, frequency: Self::DimType) -> Self {
        if let Some(dim) = self.gradients.max_dimensions() {
            assert!(frequency <= size_tuple_to_frequency!(dim, (0, 1)),
                "The gradient provider has a smaller maximum size than the requested noise frequency.");
        }
        Self {
            frequency: frequency,
            ..self
        }
    }
}

impl<G, P> Noise for Perlin2d<G, P>
where
    G: GradientProvider<Point2<u32>, Output = Vector2<f64>>,
    P: InterpolationFunction,
{
    type IndexType = Point2<f64>;
    type DimType = (f64, f64);

    fn value_at(&self, pos: Point2<f64>) -> f64 {
        let cell_pos = Vector2::new(pos[0] * self.width(), pos[1] * self.height());
        let rel_x = cell_pos.x - f64::floor(cell_pos.x);
        let rel_y = cell_pos.y - f64::floor(cell_pos.y);
        let rel_pos = Vector2::new(rel_x, rel_y);

        let x_0 = cell_pos.x as u32;
        let x_1 = x_0 + 1;
        let y_0 = cell_pos.y as u32;
        let y_1 = y_0 + 1;

        let gradients = [
            *self.gradients.get_gradient([x_0, y_0]),
            *self.gradients.get_gradient([x_1, y_0]),
            *self.gradients.get_gradient([x_0, y_1]),
            *self.gradients.get_gradient([x_1, y_1]),
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

        Lerp::lerp(p1, p2, interp_y) * f64::consts::SQRT_2
    }

    fn frequency(&self) -> (f64, f64) {
        self.frequency
    }
}

impl<G> Perlin3d<G, DefaultInterpolator>
where
    G: GradientProvider<Point3<u32>, DimType = (u32, u32, u32), Output = Vector3<f64>>,
{
    pub fn new(frequency: (f64, f64, f64), gradients: G) -> Perlin3d<G, DefaultInterpolator> {
        if let Some(dim) = gradients.max_dimensions() {
            assert!(frequency <= size_tuple_to_frequency!(dim, (0, 1, 2)),
                "The gradient provider has a smaller maximum size than the requested noise frequency.");
        }
        Perlin3d {
            frequency,
            gradients,
            interp: DefaultInterpolator::new(),
        }
    }

    pub fn build_geometric_octaves<F>(
        initial_frequency: <Self as Noise>::DimType,
        num_octaves: u32,
        frequency_scaling: <Self as Noise>::DimType,
        persistance: f64,
        gradient_factory: &mut F,
    ) -> OctaveNoise<Perlin3d<G, DefaultInterpolator>>
    where
        <Self as Noise>::DimType: TupleUtil<f64>,
        F: GradientFactory<Vector3<f64>, (f64, f64, f64), Index = Point3<u32>, Output = G>,
    {
        build_geometric_fractal_noise(
            initial_frequency,
            num_octaves,
            frequency_scaling,
            persistance,
            &mut move |n, frequency, _| {
                let g = gradient_factory.build(n, frequency);
                Perlin3d::new(frequency, g)
            },
        )
    }
}

impl Perlin3d<Grid3d<Vector3<f64>>, DefaultInterpolator> {
    pub fn from_grid(
        grid: Grid3d<Vector3<f64>>,
    ) -> Perlin3d<Grid3d<Vector3<f64>>, DefaultInterpolator> {
        Perlin3d {
            frequency: (
                grid.width() as f64,
                grid.height() as f64,
                grid.depth() as f64,
            ),
            gradients: grid,
            interp: DefaultInterpolator::new(),
        }
    }
}

impl<G, P> Perlin3d<G, P>
where
    G: GradientProvider<Point3<u32>, DimType = (u32, u32, u32), Output = Vector3<f64>>,
    P: InterpolationFunction,
{
    pub fn with_interpolator<P2>(self, interpolator: P2) -> Perlin3d<G, P2>
    where
        P2: InterpolationFunction,
    {
        Perlin3d {
            interp: interpolator,
            frequency: self.frequency,
            gradients: self.gradients,
        }
    }
}

impl<G, P> WithFrequency for Perlin3d<G, P>
where
    G: GradientProvider<Point3<u32>, DimType = (u32, u32, u32), Output = Vector3<f64>>,
    P: InterpolationFunction,
{
    fn with_frequency(self, frequency: Self::DimType) -> Self {
        if let Some(dim) = self.gradients.max_dimensions() {
            assert!(frequency <= size_tuple_to_frequency!(dim, (0, 1, 2)),
                "The gradient provider has a smaller maximum size than the requested noise frequency.");
        }
        Self {
            frequency: frequency,
            ..self
        }
    }
}

impl<G, P> Noise for Perlin3d<G, P>
where
    G: GradientProvider<Point3<u32>, Output = Vector3<f64>>,
    P: InterpolationFunction,
{
    type IndexType = Point3<f64>;
    type DimType = (f64, f64, f64);

    fn value_at(&self, pos: Point3<f64>) -> f64 {
        let cell_pos = Vector3::new(
            pos[0] * self.width(),
            pos[1] * self.height(),
            pos[2] * self.depth(),
        );

        let rel_x = cell_pos.x - cell_pos.x.floor();
        let rel_y = cell_pos.y - cell_pos.y.floor();
        let rel_z = cell_pos.z - cell_pos.z.floor();
        let rel_pos = Vector3::new(rel_x, rel_y, rel_z);

        let x_0 = cell_pos.x as u32;
        let x_1 = x_0 + 1;
        let y_0 = cell_pos.y as u32;
        let y_1 = y_0 + 1;
        let z_0 = cell_pos.z as u32;
        let z_1 = z_0 + 1;

        let gradients = [
            *self.gradients.get_gradient([x_0, y_0, z_0]),
            *self.gradients.get_gradient([x_1, y_0, z_0]),
            *self.gradients.get_gradient([x_0, y_1, z_0]),
            *self.gradients.get_gradient([x_1, y_1, z_0]),
            *self.gradients.get_gradient([x_0, y_0, z_1]),
            *self.gradients.get_gradient([x_1, y_0, z_1]),
            *self.gradients.get_gradient([x_0, y_1, z_1]),
            *self.gradients.get_gradient([x_1, y_1, z_1]),
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

    fn frequency(&self) -> (f64, f64, f64) {
        self.frequency
    }
}

#[cfg(test)]
mod tests {
    use std::f64;

    use rand;
    use test;
    use super::Perlin2d;
    use grid::{self, GradientGrid};
    use interpolate;
    use noise::{gradient, Noise};

    #[bench]
    fn bench_perlin_2d_grid(b: &mut test::Bencher) {
        let (width, height) = (1000u32, 1000);
        let dx = 1.0 / f64::from(width);
        let dy = 1.0 / f64::from(height);

        let perlin = Perlin2d::new(
            (10, 10),
            grid::Grid2d::build_grid(
                (11, 11),
                &mut gradient::RandomGradientBuilder2d::new(rand::thread_rng()),
            ),
            //gradient::provider::cube_gradient_table_2d(&mut rand::thread_rng()),
        );

        b.iter(|| perlin.value_at([0.333, 0.754]));
    }
}
