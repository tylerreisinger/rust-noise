pub mod perlin;
pub mod perlin3d;
pub mod octave;

pub use noise::perlin::{CubeGradientBuilder2d, Perlin, RandomGradientBuilder2d};
pub use noise::perlin3d::{Perlin3d, RandomGradientBuilder3d};
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

impl<'a, N> Noise for &'a N
where
    N: Noise + 'a,
{
    type IndexType = N::IndexType;
    type DimType = N::DimType;

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        (*self).value_at(pos)
    }
    fn width(&self) -> u32 {
        (*self).width()
    }
    fn height(&self) -> u32 {
        (*self).height()
    }
    fn dimensions(&self) -> Self::DimType {
        (*self).dimensions()
    }
}

impl<'a, N> Noise for &'a mut N
where
    N: Noise + 'a,
{
    type IndexType = N::IndexType;
    type DimType = N::DimType;

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        (**self).value_at(pos)
    }
    fn width(&self) -> u32 {
        (**self).width()
    }
    fn height(&self) -> u32 {
        (**self).height()
    }
    fn dimensions(&self) -> Self::DimType {
        (**self).dimensions()
    }
}

impl<N> Noise for Box<N>
where
    N: Noise,
{
    type IndexType = N::IndexType;
    type DimType = N::DimType;

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        (**self).value_at(pos)
    }
    fn width(&self) -> u32 {
        (**self).width()
    }
    fn height(&self) -> u32 {
        (**self).height()
    }
    fn dimensions(&self) -> Self::DimType {
        (**self).dimensions()
    }
}
