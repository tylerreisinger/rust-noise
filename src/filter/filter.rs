use noise::Noise;

#[derive(Debug, Clone)]
pub struct Clamp<N>
where
    N: Noise,
{
    noise: N,
    low: f64,
    high: f64,
}

impl<N> Clamp<N>
where
    N: Noise,
{
    pub fn new(noise: N, low: f64, high: f64) -> Clamp<N> {
        assert!(low < high);
        Clamp { noise, low, high }
    }

    pub fn inner_noise(&self) -> &N {
        &self.noise
    }
}

impl<N> Noise for Clamp<N>
where
    N: Noise,
{
    type IndexType = N::IndexType;
    type DimType = N::DimType;

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        let val = self.noise.value_at(pos);
        if val < self.low {
            self.low
        } else if val > self.high {
            self.high
        } else {
            val
        }
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
