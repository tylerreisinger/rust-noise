use noise::Noise;

#[derive(Clone, Debug)]
pub struct Transform<N, F>
where
    N: Noise,
    F: Fn(&N::IndexType, f64) -> f64,
{
    noise: N,
    transform: F,
}

#[derive(Clone, Debug)]
pub struct Negate<N>
where
    N: Noise,
{
    noise: N,
}

impl<N, F> Transform<N, F>
where
    N: Noise,
    F: Fn(&N::IndexType, f64) -> f64,
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
    F: Fn(&N::IndexType, f64) -> f64,
{
    type IndexType = N::IndexType;
    type DimType = N::DimType;

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        let f = &self.transform;
        f(&pos, self.noise.value_at(pos.clone()))
    }
    fn frequency(&self) -> Self::DimType {
        self.noise.frequency()
    }
}

impl<N> Negate<N>
where
    N: Noise,
{
    pub fn new(noise: N) -> Negate<N> {
        Negate { noise }
    }

    pub fn inner_noise(&self) -> &N {
        &self.noise
    }
}

impl<N> Noise for Negate<N>
where
    N: Noise,
{
    type IndexType = N::IndexType;
    type DimType = N::DimType;

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        -self.noise.value_at(pos)
    }
    fn frequency(&self) -> Self::DimType {
        self.noise.frequency()
    }
}
