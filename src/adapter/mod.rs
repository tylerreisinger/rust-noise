pub mod scale;
pub mod transform;
pub mod combine;
pub mod filter;
pub mod generate;

pub use self::scale::{Scale, WithRange};
pub use self::transform::{Negate, Transform};
pub use self::combine::{Add, Combine, Multiply, Select};
pub use self::filter::Clamp;
pub use self::generate::{Constant, FunctionValue};

use super::noise::Noise;

pub trait NoiseExt: super::noise::Noise + Sized {
    fn scale(self, amplitude: f64) -> Scale<Self> {
        Scale::new(self, amplitude)
    }

    fn with_range(self, min: f64, max: f64) -> WithRange<Self> {
        WithRange::new(self, min, max)
    }

    fn transform<F>(self, f: F) -> Transform<Self, F>
    where
        F: Fn(&Self::IndexType, f64) -> f64,
    {
        Transform::new(self, f)
    }

    fn combine<N2, F>(self, right_noise: N2, combiner: F) -> Combine<Self, N2, F>
    where
        Self::DimType: TupleUtil<u32>,
        N2: Noise<IndexType = Self::IndexType, DimType = Self::DimType>,
        F: Fn(f64, f64) -> f64,
    {
        Combine::new(self, right_noise, combiner)
    }

    fn select<N2, N3>(self, right_noise: N2, criteria: N3, threshold: f64) -> Select<Self, N2, N3>
    where
        Self::DimType: TupleUtil<u32>,
        N2: Noise<IndexType = Self::IndexType, DimType = Self::DimType>,
        N3: Noise<IndexType = Self::IndexType, DimType = Self::DimType>,
    {
        Select::new(self, right_noise, criteria, threshold)
    }

    fn add<N2>(self, right_noise: N2) -> Add<Self, N2>
    where
        Self::DimType: TupleUtil<u32>,
        N2: Noise<IndexType = Self::IndexType, DimType = Self::DimType>,
    {
        Add::new(self, right_noise)
    }

    fn multiply<N2>(self, right_noise: N2) -> Multiply<Self, N2>
    where
        Self::DimType: TupleUtil<u32>,
        N2: Noise<IndexType = Self::IndexType, DimType = Self::DimType>,
    {
        Multiply::new(self, right_noise)
    }

    fn negate(self) -> Negate<Self> {
        Negate::new(self)
    }

    fn clamp(self, low: f64, high: f64) -> Clamp<Self> {
        Clamp::new(self, low, high)
    }
}

impl<N> NoiseExt for N
where
    N: Noise,
{
}

pub trait TupleUtil<T> {
    fn max(&self, other: &Self) -> Self;
    fn saturate(val: u32) -> Self;
}

impl TupleUtil<u32> for (u32,) {
    fn max(&self, rhs: &(u32,)) -> (u32,) {
        (self.0.max(rhs.0),)
    }
    fn saturate(val: u32) -> (u32,) {
        (val,)
    }
}
impl TupleUtil<u32> for (u32, u32) {
    fn max(&self, rhs: &(u32, u32)) -> (u32, u32) {
        (self.0.max(rhs.0), self.1.max(rhs.1))
    }
    fn saturate(val: u32) -> (u32, u32) {
        (val, val)
    }
}
impl TupleUtil<u32> for (u32, u32, u32) {
    fn max(&self, rhs: &(u32, u32, u32)) -> (u32, u32, u32) {
        (self.0.max(rhs.0), self.1.max(rhs.1), self.2.max(rhs.2))
    }
    fn saturate(val: u32) -> (u32, u32, u32) {
        (val, val, val)
    }
}
impl TupleUtil<u32> for (u32, u32, u32, u32) {
    fn max(&self, rhs: &(u32, u32, u32, u32)) -> (u32, u32, u32, u32) {
        (
            self.0.max(rhs.0),
            self.1.max(rhs.1),
            self.2.max(rhs.2),
            self.3.max(rhs.3),
        )
    }
    fn saturate(val: u32) -> (u32, u32, u32, u32) {
        (val, val, val, val)
    }
}
