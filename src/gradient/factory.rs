use super::{GradientBuilder, GradientFactory};
use grid::{GradientGrid, Grid1d, Grid2d, Grid3d};
use cgmath::{Vector2, Vector3};
use noise::{Point1, Point2, Point3};
use super::PermutedGradientTable;

use rand::Rng;

#[derive(Debug)]
pub struct GridGradientFactory<'a, B, R>
where
    B: GradientBuilder + 'a,
    R: Rng + 'a,
{
    builder: &'a mut B,
    rng: R,
}

#[derive(Debug)]
pub struct RandomPermutationGradientFactory<'a, B, R>
where
    B: GradientBuilder + 'a,
    R: Rng + 'a,
{
    builder: &'a mut B,
    rng: R,
    grid_scaling: f64,
    grid_size: u32,
}

#[derive(Debug)]
pub struct PermutationGradientFactory<G>
where
    G: Clone,
{
    table: PermutedGradientTable<G>,
}

impl<'a, B, R> GridGradientFactory<'a, B, R>
where
    B: GradientBuilder + 'a,
    R: Rng,
{
    pub fn new(builder: &'a mut B, rng: R) -> GridGradientFactory<'a, B, R> {
        GridGradientFactory { builder, rng }
    }

    pub fn rng(&mut self) -> &mut R {
        &mut self.rng
    }
}

impl<'a, B, R> GradientFactory<f64, f64> for GridGradientFactory<'a, B, R>
where
    B: GradientBuilder<Output = f64> + 'a,
    R: Rng + 'a,
{
    type Index = Point1<u32>;
    type Output = Grid1d<f64>;

    fn build(&mut self, _: u32, frequency: f64) -> Grid1d<f64> {
        let dims = (frequency.ceil() + 1.0) as u32;
        Grid1d::build_grid(dims, self.builder)
    }
}
impl<'a, B, R> GradientFactory<Vector2<f64>, (f64, f64)> for GridGradientFactory<'a, B, R>
where
    B: GradientBuilder<Output = Vector2<f64>> + 'a,
    R: Rng + 'a,
{
    type Index = Point2<u32>;
    type Output = Grid2d<Vector2<f64>>;

    fn build(&mut self, _: u32, frequency: (f64, f64)) -> Grid2d<Vector2<f64>> {
        let dims = (
            (frequency.0.ceil() + 1.0) as u32,
            (frequency.1.ceil() + 1.0) as u32,
        );
        Grid2d::build_grid(dims, self.builder)
    }
}
impl<'a, B, R> GradientFactory<Vector3<f64>, (f64, f64, f64)> for GridGradientFactory<'a, B, R>
where
    B: GradientBuilder<Output = Vector3<f64>> + 'a,
    R: Rng + 'a,
{
    type Index = Point3<u32>;
    type Output = Grid3d<Vector3<f64>>;

    fn build(&mut self, _: u32, frequency: (f64, f64, f64)) -> Grid3d<Vector3<f64>> {
        let dims = (
            (frequency.0.ceil() + 1.0) as u32,
            (frequency.1.ceil() + 1.0) as u32,
            (frequency.2.ceil() + 1.0) as u32,
        );
        Grid3d::build_grid(dims, self.builder)
    }
}

impl<'a, B, R> RandomPermutationGradientFactory<'a, B, R>
where
    B: GradientBuilder + 'a,
    R: Rng + 'a,
{
    pub fn new(
        builder: &'a mut B,
        rng: R,
        grid_size: u32,
        grid_scaling: f64,
    ) -> RandomPermutationGradientFactory<'a, B, R> {
        RandomPermutationGradientFactory {
            builder,
            rng,
            grid_size,
            grid_scaling,
        }
    }

    pub fn rng(&mut self) -> &mut R {
        &mut self.rng
    }

    pub fn grid_size(&self) -> u32 {
        self.grid_size
    }
    pub fn grid_scaling(&self) -> f64 {
        self.grid_scaling
    }
}

impl<'a, B, R> GradientFactory<f64, f64> for RandomPermutationGradientFactory<'a, B, R>
where
    B: GradientBuilder<Output = f64> + 'a,
    R: Rng + 'a,
{
    type Index = Point1<u32>;
    type Output = PermutedGradientTable<f64>;

    fn build(&mut self, octave: u32, _: f64) -> PermutedGradientTable<f64> {
        PermutedGradientTable::new(
            &mut self.rng,
            self.builder,
            self.grid_size * (self.grid_scaling.powi(octave as i32) as u32),
        )
    }
}

impl<'a, B, R> GradientFactory<Vector2<f64>, (f64, f64)>
    for RandomPermutationGradientFactory<'a, B, R>
where
    B: GradientBuilder<Output = Vector2<f64>> + 'a,
    R: Rng + 'a,
{
    type Index = Point2<u32>;
    type Output = PermutedGradientTable<Vector2<f64>>;

    fn build(&mut self, octave: u32, _: (f64, f64)) -> PermutedGradientTable<Vector2<f64>> {
        PermutedGradientTable::new(
            &mut self.rng,
            self.builder,
            self.grid_size * (self.grid_scaling.powi(octave as i32) as u32),
        )
    }
}

impl<'a, B, R> GradientFactory<Vector3<f64>, (f64, f64, f64)>
    for RandomPermutationGradientFactory<'a, B, R>
where
    B: GradientBuilder<Output = Vector3<f64>> + 'a,
    R: Rng + 'a,
{
    type Index = Point3<u32>;
    type Output = PermutedGradientTable<Vector3<f64>>;

    fn build(&mut self, octave: u32, _: (f64, f64, f64)) -> PermutedGradientTable<Vector3<f64>> {
        PermutedGradientTable::new(
            &mut self.rng,
            self.builder,
            self.grid_size * (self.grid_scaling.powi(octave as i32) as u32),
        )
    }
}

impl<G> PermutationGradientFactory<G>
where
    G: Clone,
{
    pub fn new(table: PermutedGradientTable<G>) -> PermutationGradientFactory<G> {
        PermutationGradientFactory { table }
    }
}

impl GradientFactory<f64, f64> for PermutationGradientFactory<f64> {
    type Index = Point1<u32>;
    type Output = PermutedGradientTable<f64>;

    fn build(&mut self, _: u32, _: f64) -> PermutedGradientTable<f64> {
        self.table.clone()
    }
}

impl GradientFactory<Vector2<f64>, (f64, f64)> for PermutationGradientFactory<Vector2<f64>> {
    type Index = Point2<u32>;
    type Output = PermutedGradientTable<Vector2<f64>>;

    fn build(&mut self, _: u32, _: (f64, f64)) -> PermutedGradientTable<Vector2<f64>> {
        self.table.clone()
    }
}

impl GradientFactory<Vector3<f64>, (f64, f64, f64)> for PermutationGradientFactory<Vector3<f64>> {
    type Index = Point3<u32>;
    type Output = PermutedGradientTable<Vector3<f64>>;

    fn build(&mut self, _: u32, _: (f64, f64, f64)) -> PermutedGradientTable<Vector3<f64>> {
        self.table.clone()
    }
}
