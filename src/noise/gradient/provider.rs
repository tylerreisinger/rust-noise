use std::f64;
use std::ops::{Index, IndexMut};

use super::{GradientBuilder, GradientProvider, PermutationTable};
use noise::{Point1, Point2, Point3, Point4};
use cgmath::Vector2;

use rand::Rng;

#[derive(Debug, Clone)]
pub struct GradientTable<G> {
    table: Vec<G>,
}
#[derive(Debug, Clone)]
pub struct PermutedGradientTable<G> {
    table: GradientTable<G>,
    permutations: PermutationTable,
}

const FRAC_SQRT_2: f64 = f64::consts::FRAC_1_SQRT_2;

pub fn cube_gradient_table_2d<R>(rng: &mut R) -> PermutedGradientTable<Vector2<f64>>
where
    R: Rng,
{
    PermutedGradientTable::from_values(
        rng,
        vec![
            Vector2::new(1.0, 0.0),
            Vector2::new(0.0, -1.0),
            Vector2::new(-1.0, 0.0),
            Vector2::new(0.0, 1.0),
            Vector2::new(1.0, 0.0),
            Vector2::new(0.0, -1.0),
            Vector2::new(-1.0, 0.0),
            Vector2::new(0.0, 1.0),
            Vector2::new(FRAC_SQRT_2, FRAC_SQRT_2),
            Vector2::new(FRAC_SQRT_2, -FRAC_SQRT_2),
            Vector2::new(-FRAC_SQRT_2, -FRAC_SQRT_2),
            Vector2::new(-FRAC_SQRT_2, FRAC_SQRT_2),
        ],
    )
}

impl<G> GradientTable<G>
where
    G: Clone,
{
    pub fn new<B>(builder: &mut B, size: usize) -> GradientTable<G>
    where
        B: GradientBuilder<Output = G>,
    {
        let vec = (0..size).map(|_| builder.make_gradient()).collect();

        GradientTable { table: vec }
    }

    pub fn from_gradients(table: Vec<G>) -> GradientTable<G> {
        GradientTable { table }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.table.len()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }
}

impl<G> GradientProvider<Point1<u32>> for GradientTable<G>
where
    G: Clone,
{
    type Output = G;
    type DimType = (u32,);

    #[inline]
    fn get_gradient(&self, index: Point1<u32>) -> &G {
        let idx = (index as usize) % self.len();
        unsafe { self.table.get_unchecked(idx) }
    }

    fn dimensions(&self) -> Option<Self::DimType> {
        None
    }
}

impl<G> Index<u32> for GradientTable<G>
where
    G: Clone,
{
    type Output = G;
    #[inline]
    fn index(&self, idx: u32) -> &G {
        self.get_gradient(idx)
    }
}
impl<G> IndexMut<u32> for GradientTable<G>
where
    G: Clone,
{
    #[inline]
    fn index_mut(&mut self, idx: u32) -> &mut G {
        let len = self.len();
        &mut self.table[(idx as usize) % len]
    }
}

impl<G> PermutedGradientTable<G>
where
    G: Clone,
{
    //This must always be a power of 2.
    const DEFAULT_PERMUTATION_SIZE: u32 = 256;
    pub fn new<B, R>(rng: &mut R, builder: &mut B, size: usize) -> PermutedGradientTable<G>
    where
        B: GradientBuilder<Output = G>,
        R: Rng,
    {
        let permutations = PermutationTable::new(rng, Self::DEFAULT_PERMUTATION_SIZE);
        let table = GradientTable::new(builder, size);

        PermutedGradientTable {
            permutations,
            table,
        }
    }

    pub fn from_values<R>(rng: &mut R, values: Vec<G>) -> PermutedGradientTable<G>
    where
        R: Rng,
    {
        let permutations = PermutationTable::new(rng, Self::DEFAULT_PERMUTATION_SIZE);
        let table = GradientTable::from_gradients(values);

        PermutedGradientTable {
            permutations,
            table,
        }
    }

    pub fn from_parts(
        permutations: PermutationTable,
        gradients: GradientTable<G>,
    ) -> PermutedGradientTable<G> {
        PermutedGradientTable {
            permutations,
            table: gradients,
        }
    }

    pub fn with_permutation_table_size<R>(self, rng: &mut R, size: u32) -> PermutedGradientTable<G>
    where
        R: Rng,
    {
        let permutations = PermutationTable::new(rng, Self::adjust_size(size));

        PermutedGradientTable {
            table: self.table,
            permutations,
        }
    }

    pub fn gradient_table(&self) -> &GradientTable<G> {
        &self.table
    }
    pub fn permutation_table(&self) -> &PermutationTable {
        &self.permutations
    }

    fn adjust_size(size: u32) -> u32 {
        let mut pow = 1;
        while pow < size {
            pow <<= 1;
        }
        pow
    }

    #[inline]
    fn wrap_val(&self, val: u32) -> u32 {
        //val % (self.permutations.len() as u32)
        val & ((self.permutations.len() - 1) as u32)
        //val
    }

    #[inline]
    pub fn index_1d(&self, val: Point1<u32>) -> u32 {
        let v = self.wrap_val(val);
        //Since we wrap the value, it is not possible to go out of bounds.
        unsafe { self.permutations.get_unchecked(v) }
    }
    #[inline]
    pub fn index_2d(&self, val: Point2<u32>) -> u32 {
        let v = self.wrap_val(val[1]);
        unsafe { self.permutations.get_unchecked(self.index_1d(val[0]) ^ v) }
    }
    #[inline]
    pub fn index_3d(&self, val: Point3<u32>) -> u32 {
        let v = self.wrap_val(val[2]);
        unsafe {
            self.permutations
                .get_unchecked(self.index_2d([val[0], val[1]]) ^ v)
        }
    }
    #[inline]
    pub fn index_4d(&self, val: Point4<u32>) -> u32 {
        let v = self.wrap_val(val[3]);
        unsafe {
            self.permutations
                .get_unchecked(self.index_3d([val[0], val[1], val[2]]) ^ v)
        }
    }
}

impl<G> GradientProvider<Point1<u32>> for PermutedGradientTable<G>
where
    G: Clone,
{
    type Output = G;
    type DimType = (u32,);

    #[inline]
    fn get_gradient(&self, index: Point1<u32>) -> &G {
        self.table.get_gradient(self.index_1d(index))
    }
    fn dimensions(&self) -> Option<Self::DimType> {
        None
    }
}
impl<G> GradientProvider<Point2<u32>> for PermutedGradientTable<G>
where
    G: Clone,
{
    type Output = G;
    type DimType = (u32, u32);

    #[inline]
    fn get_gradient(&self, index: Point2<u32>) -> &G {
        self.table.get_gradient(self.index_2d(index))
    }
    fn dimensions(&self) -> Option<Self::DimType> {
        None
    }
}
impl<G> GradientProvider<Point3<u32>> for PermutedGradientTable<G>
where
    G: Clone,
{
    type Output = G;
    type DimType = (u32, u32, u32);

    #[inline]
    fn get_gradient(&self, index: Point3<u32>) -> &G {
        self.table.get_gradient(self.index_3d(index))
    }
    fn dimensions(&self) -> Option<Self::DimType> {
        None
    }
}
impl<G> GradientProvider<Point4<u32>> for PermutedGradientTable<G>
where
    G: Clone,
{
    type Output = G;
    type DimType = (u32, u32, u32, u32);

    #[inline]
    fn get_gradient(&self, index: Point4<u32>) -> &G {
        self.table.get_gradient(self.index_4d(index))
    }
    fn dimensions(&self) -> Option<Self::DimType> {
        None
    }
}
