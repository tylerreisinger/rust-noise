use std::f64;

use cgmath::Vector2;
use cgmath::{InnerSpace, Vector3};

use grid::{Grid, Grid3d};
use interpolate::{InterpolationFunction, Lerp};
use noise::{GradientBuilder, Noise, Noise1d, Noise2d, Noise3d, Point1, Point2, Point3, TupleUtil};
use noise::octave::{build_geometric_fractal_noise, OctaveNoise};

#[derive(Clone, Debug)]
pub struct Perlin1d<P: InterpolationFunction> {
    grid: Vec<f64>,
    interp: P,
}
#[derive(Clone, Debug)]
pub struct Perlin2d<P: InterpolationFunction> {
    grid: Grid<Vector2<f64>>,
    interp: P,
}
#[derive(Debug, Clone)]
pub struct Perlin3d<P: InterpolationFunction> {
    grid: Grid3d<Vector3<f64>>,
    interp: P,
}

impl<P> Perlin1d<P>
where
    P: InterpolationFunction + Clone,
{
    pub fn new<T>(size: u32, builder: &mut T, interp: P) -> Perlin1d<P>
    where
        T: GradientBuilder<Output = f64>,
    {
        let data = (0..size).map(|_| builder.make_gradient()).collect();

        Perlin1d {
            grid: data,
            interp: interp,
        }
    }

    pub fn build_geometric_octaves<G>(
        initial_frequency: u32,
        num_octaves: u32,
        octave_scaling: u32,
        persistance: f64,
        gradient_builder: &mut G,
        interpolator: P,
    ) -> OctaveNoise<Self>
    where
        G: GradientBuilder<Output = f64>,
        Self: Noise<DimType = (u32,)>,
    {
        build_geometric_fractal_noise::<Self, _>(
            (initial_frequency,),
            num_octaves,
            (octave_scaling,),
            persistance,
            &mut move |_, frequency, _| {
                Perlin1d::new(frequency.0, gradient_builder, interpolator.clone())
            },
        )
    }
}

impl<P> Noise for Perlin1d<P>
where
    P: InterpolationFunction,
{
    type IndexType = Point1<f64>;
    type DimType = (u32,);

    fn value_at(&self, pos: f64) -> f64 {
        let cell_pos = pos * f64::from(self.width());
        let x_0 = cell_pos as usize;
        let x_1 = x_0 + 1;

        let rel_pos = cell_pos - cell_pos.floor();

        let gradients = [self.grid[x_0], self.grid[x_1]];
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

    fn dimensions(&self) -> (u32,) {
        (self.grid.len() as u32 - 1,)
    }
}

impl<P> Perlin2d<P>
where
    P: InterpolationFunction + Clone,
{
    pub fn new<T>(dimensions: (u32, u32), builder: &mut T, interp: P) -> Perlin2d<P>
    where
        T: GradientBuilder<Output = Vector2<f64>>,
    {
        let (width, height) = dimensions;
        let size = ((width + 1) as usize) * ((height + 1) as usize);

        let data = (0..size).map(|_| builder.make_gradient()).collect();

        Perlin2d {
            grid: Grid::with_data(width + 1, height + 1, data),
            interp: interp,
        }
    }
    pub fn build_geometric_octaves<G>(
        initial_frequency: <Self as Noise>::DimType,
        num_octaves: u32,
        octave_scaling: <Self as Noise>::DimType,
        persistance: f64,
        gradient_builder: &mut G,
        interpolator: P,
    ) -> OctaveNoise<Self>
    where
        G: GradientBuilder<Output = Vector2<f64>>,
        <Self as Noise>::DimType: TupleUtil<u32>,
    {
        build_geometric_fractal_noise(
            initial_frequency,
            num_octaves,
            octave_scaling,
            persistance,
            &mut move |_, frequency, _| {
                Perlin2d::new(frequency, gradient_builder, interpolator.clone())
            },
        )
    }
}

impl<P> Noise for Perlin2d<P>
where
    P: InterpolationFunction,
{
    type IndexType = Point2<f64>;
    type DimType = (u32, u32);

    fn value_at(&self, pos: Point2<f64>) -> f64 {
        let cell_pos = Vector2::new(
            pos[0] * f64::from(self.width()),
            pos[1] * f64::from(self.height()),
        );
        let x_0 = cell_pos.x as usize;
        let x_1 = x_0 + 1;
        let y_0 = cell_pos.y as usize;
        let y_1 = y_0 + 1;

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

        Lerp::lerp(p1, p2, interp_y) * f64::consts::SQRT_2
    }

    fn dimensions(&self) -> (u32, u32) {
        (self.grid.width() - 1, self.grid.height() - 1)
    }
}

impl<P> Perlin3d<P>
where
    P: InterpolationFunction + Clone,
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

    pub fn build_geometric_octaves<G>(
        initial_frequency: <Self as Noise>::DimType,
        num_octaves: u32,
        octave_scaling: <Self as Noise>::DimType,
        persistance: f64,
        gradient_builder: &mut G,
        interpolator: P,
    ) -> OctaveNoise<Self>
    where
        G: GradientBuilder<Output = Vector3<f64>>,
        <Self as Noise>::DimType: TupleUtil<u32>,
    {
        build_geometric_fractal_noise(
            initial_frequency,
            num_octaves,
            octave_scaling,
            persistance,
            &mut move |_, frequency, _| {
                Perlin3d::new(frequency, gradient_builder, interpolator.clone())
            },
        )
    }
}

impl<P> Noise for Perlin3d<P>
where
    P: InterpolationFunction,
{
    type IndexType = Point3<f64>;
    type DimType = (u32, u32, u32);

    fn value_at(&self, pos: Point3<f64>) -> f64 {
        let cell_pos = Vector3::new(
            pos[0] * f64::from(self.width()),
            pos[1] * f64::from(self.height()),
            pos[2] * f64::from(self.depth()),
        );

        let x_0 = cell_pos.x as usize;
        let x_1 = x_0 + 1;
        let y_0 = cell_pos.y as usize;
        let y_1 = y_0 + 1;
        let z_0 = cell_pos.z as usize;
        let z_1 = z_0 + 1;

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

    fn dimensions(&self) -> (u32, u32, u32) {
        (
            self.grid.width() - 1,
            self.grid.height() - 1,
            self.grid.depth() - 1,
        )
    }
}
