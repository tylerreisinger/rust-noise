use noise::Noise;
use cgmath::{Vector2, Vector3};

#[derive(Debug, Clone)]
pub struct Slice2d<'a, N: Noise + 'a> {
    noise: &'a N,
    depth: f64,
}

impl<'a, N> Slice2d<'a, N>
where
    N: Noise,
{
    pub fn new(noise: &'a N, depth: f64) -> Slice2d<'a, N> {
        Slice2d { noise, depth }
    }

    pub fn noise(&'a self) -> &'a N {
        self.noise
    }

    pub fn depth(&self) -> f64 {
        self.depth
    }
}

impl<'a, N> Noise for Slice2d<'a, N>
where
    N: Noise<IndexType = Vector3<f64>, DimType = (u32, u32, u32)>,
{
    type IndexType = Vector2<f64>;
    type DimType = (u32, u32);

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        self.noise.value_at(Vector3::new(pos.x, pos.y, self.depth))
    }

    fn width(&self) -> u32 {
        self.noise.width()
    }
    fn height(&self) -> u32 {
        self.noise.height()
    }
    fn dimensions(&self) -> Self::DimType {
        (self.noise.dimensions().0, self.noise.dimensions().1)
    }
}
