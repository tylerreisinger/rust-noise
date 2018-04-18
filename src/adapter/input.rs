use noise::{Noise, PointUtil};

#[derive(Clone, Debug)]
pub struct ScaleInput<N>
where
    N: Noise,
{
    noise: N,
    scale: N::IndexType,
}
#[derive(Clone, Debug)]
pub struct ShiftInput<N>
where
    N: Noise,
{
    noise: N,
    shift: N::IndexType,
}

impl<N> ScaleInput<N>
where
    N: Noise,
{
    pub fn new(noise: N, scale: N::IndexType) -> ScaleInput<N> {
        ScaleInput { noise, scale }
    }

    pub fn inner_noise(&self) -> &N {
        &self.noise
    }
}

impl<N> Noise for ScaleInput<N>
where
    N: Noise,
    N::IndexType: PointUtil<f64>,
{
    type IndexType = N::IndexType;
    type DimType = N::DimType;

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        let scaled_pos = pos.apply(self.scale.clone(), |x, y| x * y);
        self.noise.value_at(scaled_pos)
    }

    fn dimensions(&self) -> N::DimType {
        self.noise.dimensions()
    }
}

impl<N> ShiftInput<N>
where
    N: Noise,
{
    pub fn new(noise: N, shift: N::IndexType) -> ShiftInput<N> {
        ShiftInput { noise, shift }
    }

    pub fn inner_noise(&self) -> &N {
        &self.noise
    }
}

impl<N> Noise for ShiftInput<N>
where
    N: Noise,
    N::IndexType: PointUtil<f64>,
{
    type IndexType = N::IndexType;
    type DimType = N::DimType;

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        let shifted_pos = pos.apply(self.shift.clone(), |x, y| x + y);
        self.noise.value_at(shifted_pos)
    }

    fn dimensions(&self) -> N::DimType {
        self.noise.dimensions()
    }
}
