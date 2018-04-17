pub mod perlin;
pub mod perlin3d;
pub mod slice;
pub mod octave;

pub use noise::perlin::{RandomGradientBuilder2d, CubeGradientBuilder2d, Perlin};
pub use noise::perlin3d::{Perlin3d, RandomGradientBuilder3d};
pub use noise::slice::Slice2d;
pub use noise::octave::{Octave, OctaveNoise};

pub trait GradientBuilder {
    type Output;

    fn make_gradient(&mut self) -> Self::Output;
}

pub trait Noise {
    type IndexType: Clone;
    type DimType;

    fn value_at(&self, pos: Self::IndexType) -> f64;

    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn dimensions(&self) -> Self::DimType;
}
