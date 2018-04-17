pub mod gradient;
pub mod perlin;
pub mod octave;

pub use noise::gradient::{CubeGradientBuilder2d, RandomGradientBuilder2d, RandomGradientBuilder3d};
pub use noise::perlin::{Perlin2d, Perlin3d};
pub use noise::octave::{Octave, OctaveNoise};

use adapter::{Extension2d, Extension3d, Slice1d, Slice2d};
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

pub trait Noise1d: Noise<IndexType = f64, DimType = (u32,)> + Sized {
    fn width(&self) -> u32 {
        self.dimensions().0
    }

    fn extend(self) -> Extension2d<Self> {
        Extension2d::new(self)
    }
}
pub trait Noise2d: Noise<IndexType = Vector2<f64>, DimType = (u32, u32)> + Sized {
    fn width(&self) -> u32 {
        self.dimensions().0
    }
    fn height(&self) -> u32 {
        self.dimensions().1
    }

    fn slice(self, height: f64) -> Slice1d<Self> {
        Slice1d::new(self, height)
    }
    fn extend(self) -> Extension3d<Self> {
        Extension3d::new(self)
    }
}
pub trait Noise3d
    : Noise<IndexType = Vector3<f64>, DimType = (u32, u32, u32)> + Sized {
    fn width(&self) -> u32 {
        self.dimensions().0
    }
    fn height(&self) -> u32 {
        self.dimensions().1
    }
    fn depth(&self) -> u32 {
        self.dimensions().2
    }

    fn slice(self, depth: f64) -> Slice2d<Self> {
        Slice2d::new(self, depth)
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
