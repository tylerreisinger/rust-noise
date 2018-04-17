pub mod perlin;
pub mod perlin3d;
pub mod octave;

pub use noise::perlin::{CubeGradientBuilder2d, Perlin, RandomGradientBuilder2d};
pub use noise::perlin3d::{Perlin3d, RandomGradientBuilder3d};
pub use noise::octave::{Octave, OctaveNoise};

use cgmath::{Vector2, Vector3};

pub trait GradientBuilder {
    type Output;

    fn make_gradient(&mut self) -> Self::Output;
}

pub trait Noise {
    type IndexType: Clone;
    type DimType;

    fn value_at(&self, pos: Self::IndexType) -> f64;

    fn dimensions(&self) -> Self::DimType;
}

pub trait Noise1d: Noise<IndexType = f64, DimType = (u32,)> {
    fn width(&self) -> u32 {
        self.dimensions().0
    }
}
pub trait Noise2d: Noise<IndexType = Vector2<f64>, DimType = (u32, u32)> {
    fn width(&self) -> u32 {
        self.dimensions().0
    }
    fn height(&self) -> u32 {
        self.dimensions().1
    }
}
pub trait Noise3d: Noise<IndexType = Vector3<f64>, DimType = (u32, u32, u32)> {
    fn width(&self) -> u32 {
        self.dimensions().0
    }
    fn height(&self) -> u32 {
        self.dimensions().1
    }
    fn depth(&self) -> u32 {
        self.dimensions().2
    }
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
    fn dimensions(&self) -> Self::DimType {
        (**self).dimensions()
    }
}

impl<N> Noise1d for N
where
    N: Noise<IndexType = f64, DimType = (u32,)>,
{
}
impl<N> Noise2d for N
where
    N: Noise<IndexType = Vector2<f64>, DimType = (u32, u32)>,
{
}
impl<N> Noise3d for N
where
    N: Noise<IndexType = Vector3<f64>, DimType = (u32, u32, u32)>,
{
}
