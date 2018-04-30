use noise::{Noise, Noise2d, Noise3d, Point1, Point2};

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
    N: Noise2d,
{
    type IndexType = Point1<f64>;
    type DimType = f64;

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        self.noise.value_at([pos, self.height])
    }

    fn frequency(&self) -> Self::DimType {
        self.noise.frequency().0
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
    N: Noise3d,
{
    type IndexType = Point2<f64>;
    type DimType = (f64, f64);

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        self.noise.value_at([pos[0], pos[1], self.depth])
    }

    fn frequency(&self) -> Self::DimType {
        (self.noise.frequency().0, self.noise.frequency().1)
    }
}
