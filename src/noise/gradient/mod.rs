pub mod build;
pub mod permutation;
pub mod provider;

pub use self::build::{CubeGradientBuilder1d, CubeGradientBuilder2d, RandomGradientBuilder1d,
                      RandomGradientBuilder2d, RandomGradientBuilder3d};
pub use self::permutation::PermutationTable;
pub use self::provider::GradientTable;

pub trait GradientBuilder {
    type Output;

    fn make_gradient(&mut self) -> Self::Output;
}

pub trait GradientProvider<I> {
    type Output: Clone;
    fn get_gradient(&self, index: &I) -> &Self::Output;
}
