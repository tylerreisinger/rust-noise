pub mod build;
pub mod permutation;
pub mod provider;
pub mod factory;

pub use self::build::{CubeGradientBuilder1d, CubeGradientBuilder2d, RandomGradientBuilder1d,
                      RandomGradientBuilder2d, RandomGradientBuilder3d};
pub use self::factory::{GridGradientFactory, PermutationGradientFactory,
                        RandomPermutationGradientFactory};
pub use self::permutation::PermutationTable;
pub use self::provider::{GradientTable, PermutedGradientTable};

use noise::Point1;

pub trait GradientFactory<O, F>
where
    O: Clone,
{
    type Index;
    type Output: GradientProvider<Self::Index, Output = O>;

    fn build(&mut self, octave: u32, frequency: F) -> Self::Output;
}

pub trait GradientBuilder {
    type Output;

    fn make_gradient(&mut self) -> Self::Output;
}

pub trait GradientProvider<I> {
    type Output: Clone;
    type DimType;
    fn get_gradient(&self, index: I) -> &Self::Output;

    fn dimensions(&self) -> Option<Self::DimType> {
        None
    }
}

impl<T> GradientProvider<Point1<u32>> for Vec<T>
where
    T: Clone,
{
    type Output = T;
    type DimType = u32;
    fn get_gradient(&self, index: Point1<u32>) -> &Self::Output {
        &self[index as usize]
    }

    fn dimensions(&self) -> Option<Self::DimType> {
        Some(self.len() as u32)
    }
}
