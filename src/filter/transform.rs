use noise::Noise;

#[derive(Clone, Debug)]
pub struct Transform<N, F>
where
    N: Noise,
    F: Fn(N::IndexType, f64) -> f64,
{
    noise: N,
    transform: F,
}

impl<N, F> Transform<N, F>
where
    N: Noise,
    F: Fn(N::IndexType, f64) -> f64,
{
    pub fn new(noise: N, transform: F) -> Transform<N, F> {
        Transform { noise, transform }
    }

    pub fn inner_noise(&self) -> &N {
        &self.noise
    }
    pub fn transform_fn(&self) -> &F {
        &self.transform
    }
}

impl<N, F> Noise for Transform<N, F>
where
    N: Noise,
    F: Fn(N::IndexType, f64) -> f64,
{
    type IndexType = N::IndexType;
    type DimType = N::DimType;

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        let f = &self.transform;
        f(pos.clone(), self.noise.value_at(pos))
    }
    fn width(&self) -> u32 {
        self.noise.width()
    }
    fn height(&self) -> u32 {
        self.noise.height()
    }
    fn dimensions(&self) -> Self::DimType {
        self.noise.dimensions()
    }
}
