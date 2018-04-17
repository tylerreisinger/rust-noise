use noise::{Noise, Noise1d, Noise2d};
use cgmath::{Vector2, Vector3};

#[derive(Debug, Clone)]
pub struct Extension2d<N>
where
    N: Noise1d,
{
    noise: N,
}

#[derive(Debug, Clone)]
pub struct Extension3d<N>
where
    N: Noise2d,
{
    noise: N,
}

impl<N> Extension2d<N>
where
    N: Noise1d,
{
    pub fn new(noise: N) -> Extension2d<N> {
        Extension2d { noise }
    }

    pub fn inner_noise(&self) -> &N {
        &self.noise
    }
}

impl<N> Noise for Extension2d<N>
where
    N: Noise1d,
{
    type IndexType = Vector2<f64>;
    type DimType = (u32, u32);

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        self.noise.value_at(pos.x)
    }
    fn dimensions(&self) -> Self::DimType {
        (self.noise.width(), 1)
    }
}

impl<N> Extension3d<N>
where
    N: Noise2d,
{
    pub fn new(noise: N) -> Extension3d<N> {
        Extension3d { noise }
    }

    pub fn inner_noise(&self) -> &N {
        &self.noise
    }
}

impl<N> Noise for Extension3d<N>
where
    N: Noise2d,
{
    type IndexType = Vector3<f64>;
    type DimType = (u32, u32, u32);

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        self.noise.value_at(Vector2::new(pos.x, pos.y))
    }
    fn dimensions(&self) -> Self::DimType {
        (self.noise.width(), self.noise.height(), 1)
    }
}
