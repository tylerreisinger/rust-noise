pub mod scale;
pub mod transform;
pub mod combine;
pub mod filter;

pub use filter::scale::{Scale, WithRange};
pub use filter::transform::Transform;
pub use filter::combine::Combine;
pub use filter::filter::Clamp;

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
        N2: super::noise::Noise<IndexType = Self::IndexType, DimType = Self::DimType>,
        F: Fn(f64, f64) -> f64,
    {
        Combine::new(self, right_noise, combiner)
    }

    fn clamp(self, low: f64, high: f64) -> Clamp<Self> {
        Clamp::new(self, low, high)
    }
}

impl<N> NoiseExt for N
where
    N: super::noise::Noise,
{
}
