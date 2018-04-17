use noise::Noise;

#[derive(Clone, Debug)]
pub struct Scale<N>
where
    N: Noise,
{
    noise: N,
    amplitude: f64,
}
#[derive(Clone, Debug)]
pub struct WithRange<N>
where
    N: Noise,
{
    noise: N,
    min: f64,
    max: f64,
}

impl<N> Scale<N>
where
    N: Noise,
{
    pub fn new(noise: N, amplitude: f64) -> Scale<N> {
        Scale { noise, amplitude }
    }

    pub fn inner_noise(&self) -> &N {
        &self.noise
    }
    pub fn amplitude(&self) -> f64 {
        self.amplitude
    }
}

impl<N> WithRange<N>
where
    N: Noise,
{
    pub fn new(noise: N, min: f64, max: f64) -> WithRange<N> {
        assert!(min < max);
        WithRange { noise, min, max }
    }

    pub fn inner_noise(&self) -> &N {
        &self.noise
    }
    pub fn min(&self) -> f64 {
        self.min
    }
    pub fn max(&self) -> f64 {
        self.max
    }
}

impl<N> Noise for Scale<N>
where
    N: Noise,
{
    type IndexType = N::IndexType;
    type DimType = N::DimType;

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        self.noise.value_at(pos) * self.amplitude
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

impl<N> Noise for WithRange<N>
where
    N: Noise,
{
    type IndexType = N::IndexType;
    type DimType = N::DimType;

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        let normalized_val = 0.5 + 0.5 * self.noise.value_at(pos);

        self.min + normalized_val * (self.max - self.min)
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
