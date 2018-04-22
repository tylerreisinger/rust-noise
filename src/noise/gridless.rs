use std::u8;
use std::f64;

use cgmath::{InnerSpace, Vector2};
use noise::{Noise, Point2};
use interpolate::{Hermite5thOrderInterpolator, InterpolationFunction, Lerp};

use rand::{self, Rng, SeedableRng, XorShiftRng};

const TABLE_SIZE: usize = 256;

pub struct PermutationTable {
    table: Vec<u8>,
}

pub struct Perlin2d<P> {
    permutations: PermutationTable,
    interp: P,
}

impl PermutationTable {
    pub fn new<R>(rng: &mut R) -> PermutationTable
    where
        R: Rng,
    {
        let mut table: Vec<_> = (0..TABLE_SIZE)
            .map(|x| (x % u8::MAX as usize) as u8)
            .collect();
        rng.shuffle(&mut table);

        PermutationTable { table }
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }
}

impl Perlin2d<Hermite5thOrderInterpolator> {
    pub fn new() -> Perlin2d<Hermite5thOrderInterpolator> {
        Self::with_seed(rand::thread_rng().next_u64())
    }

    pub fn with_seed(seed: u64) -> Perlin2d<Hermite5thOrderInterpolator> {
        let low_part = (seed >> 32) as u32;
        let high_part = seed as u32;
        let mut rng = XorShiftRng::from_seed([1, low_part, high_part, low_part]);
        Perlin2d {
            permutations: PermutationTable::new(&mut rng),
            interp: Hermite5thOrderInterpolator::new(),
        }
    }
}

impl<P> Perlin2d<P>
where
    P: InterpolationFunction,
{
    pub fn set_interpolator<P2>(self, interpolator: P2) -> Perlin2d<P2>
    where
        P2: InterpolationFunction,
    {
        Perlin2d {
            interp: interpolator,
            permutations: self.permutations,
        }
    }
}

const INV_SQRT_2: f64 = 0.7071067811865475244008443621048490392848359376884740;

pub fn get_2d_gradient(hash: u8) -> Vector2<f64> {
    match hash % 12 {
        0 | 4 => Vector2::new(1.0, 0.0),
        1 | 5 => Vector2::new(0.0, 1.0),
        2 | 6 => Vector2::new(-1.0, 0.0),
        3 | 7 => Vector2::new(0.0, -1.0),
        8 => Vector2::new(INV_SQRT_2, INV_SQRT_2),
        9 => Vector2::new(-INV_SQRT_2, INV_SQRT_2),
        10 => Vector2::new(-INV_SQRT_2, -INV_SQRT_2),
        11 => Vector2::new(INV_SQRT_2, -INV_SQRT_2),
        _ => unreachable!(),
    }
}

pub fn hash_2d(x: isize, y: isize) -> usize {
    ((x & 0xFF) ^ (y & 0xFF)) as usize
}

impl<P> Noise for Perlin2d<P>
where
    P: InterpolationFunction,
{
    type IndexType = Point2<f64>;
    type DimType = (u32, u32);

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        let x_0 = pos[0] as isize;
        let x_1 = x_0 + 1;
        let y_0 = pos[1] as isize;
        let y_1 = y_0 + 1;

        let rel_x = pos[0] - (x_0 as f64);
        let rel_y = pos[1] - (y_0 as f64);
        let rel_pos = Vector2::new(rel_x, rel_y);

        let gradients = [
            get_2d_gradient(self.permutations.table[hash_2d(x_0, y_0)]),
            get_2d_gradient(self.permutations.table[hash_2d(x_1, y_0)]),
            get_2d_gradient(self.permutations.table[hash_2d(x_0, y_1)]),
            get_2d_gradient(self.permutations.table[hash_2d(x_1, y_1)]),
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
        (1, 1)
    }
}
