use noise::Noise;
use cgmath::{Vector2, Vector3};

#[derive(Debug, Clone)]
pub struct Slice1d<N: Noise> {
    noise: N,
    height: f64,
}
#[derive(Debug, Clone)]
pub struct Slice2d<N: Noise> {
    noise: N,
    depth: f64,
}

impl<N> Slice1d<N>
where
    N: Noise,
{
    pub fn new(noise: N, height: f64) -> Slice1d<N> {
        Slice1d { noise, height }
    }
    pub fn noise(&self) -> &N {
        &self.noise
    }
    pub fn slice_height(&self) -> f64 {
        self.height
    }
}

impl<N> Noise for Slice1d<N>
where
    N: Noise<IndexType = Vector2<f64>, DimType = (u32, u32)>,
{
    type IndexType = f64;
    type DimType = (u32,);

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        self.noise.value_at(Vector2::new(pos, self.height))
    }

    fn dimensions(&self) -> Self::DimType {
        (self.noise.dimensions().0,)
    }
}

impl<N> Slice2d<N>
where
    N: Noise,
{
    pub fn new(noise: N, depth: f64) -> Slice2d<N> {
        Slice2d { noise, depth }
    }

    pub fn noise(&self) -> &N {
        &self.noise
    }

    pub fn slice_depth(&self) -> f64 {
        self.depth
    }
}

impl<N> Noise for Slice2d<N>
where
    N: Noise<IndexType = Vector3<f64>, DimType = (u32, u32, u32)>,
{
    type IndexType = Vector2<f64>;
    type DimType = (u32, u32);

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        self.noise.value_at(Vector3::new(pos.x, pos.y, self.depth))
    }

    fn dimensions(&self) -> Self::DimType {
        (self.noise.dimensions().0, self.noise.dimensions().1)
    }
}
