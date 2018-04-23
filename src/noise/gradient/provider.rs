use std::ops::{Index, IndexMut};

use super::{GradientBuilder, GradientProvider, PermutationTable};
use noise::{Point1, Point2, Point3, Point4};

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

    pub fn len(&self) -> usize {
        self.table.len()
    }
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

    fn get_gradient(&self, index: &Point1<u32>) -> &G {
        &self.table[(*index as usize) % self.len()]
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
    fn index(&self, idx: u32) -> &G {
        self.get_gradient(&idx)
    }
}
impl<G> IndexMut<u32> for GradientTable<G>
where
    G: Clone,
{
    fn index_mut(&mut self, idx: u32) -> &mut G {
        let len = self.len();
        &mut self.table[(idx as usize) % len]
    }
}

impl<G> PermutedGradientTable<G>
where
    G: Clone,
{
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

    pub fn with_permutation_table_size<R>(self, rng: &mut R, size: u32) -> PermutedGradientTable<G>
    where
        R: Rng,
    {
        let permutations = PermutationTable::new(rng, size);

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

    pub fn hash_1d(&self, val: Point1<u32>) -> u32 {
        val
    }
    pub fn hash_2d(&self, val: Point2<u32>) -> u32 {
        val[0] ^ val[1]
    }
    pub fn hash_3d(&self, val: Point3<u32>) -> u32 {
        val[0] ^ val[1] ^ val[2]
    }
    pub fn hash_4d(&self, val: Point4<u32>) -> u32 {
        val[0] ^ val[1] ^ val[2] ^ val[3]
    }
}

impl<G> GradientProvider<Point1<u32>> for PermutedGradientTable<G>
where
    G: Clone,
{
    type Output = G;
    type DimType = (u32,);

    fn get_gradient(&self, index: &Point1<u32>) -> &G {
        self.table.get_gradient(&self.hash_1d(*index))
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

    fn get_gradient(&self, index: &Point2<u32>) -> &G {
        self.table.get_gradient(&self.hash_2d(*index))
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

    fn get_gradient(&self, index: &Point3<u32>) -> &G {
        self.table.get_gradient(&self.hash_3d(*index))
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

    fn get_gradient(&self, index: &Point4<u32>) -> &G {
        self.table.get_gradient(&self.hash_4d(*index))
    }
    fn dimensions(&self) -> Option<Self::DimType> {
        None
    }
}
