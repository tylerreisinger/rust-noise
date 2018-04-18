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
#[derive(Clone, Debug)]
pub struct ClampInput<N>
where
    N: Noise,
{
    noise: N,
    low: N::IndexType,
    high: N::IndexType,
}
#[derive(Clone, Debug)]
pub struct WrapInput<N>
where
    N: Noise,
{
    noise: N,
    low: N::IndexType,
    high: N::IndexType,
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

impl<N> ClampInput<N>
where
    N: Noise,
    N::IndexType: PointUtil<f64>,
{
    pub fn new(noise: N, low: N::IndexType, high: N::IndexType) -> ClampInput<N> {
        low.clone().apply(high.clone(), |l, h| {
            assert!(l < h);
            l
        });
        ClampInput { noise, low, high }
    }

    pub fn inner_noise(&self) -> &N {
        &self.noise
    }
}

impl<N> Noise for ClampInput<N>
where
    N: Noise,
    N::IndexType: PointUtil<f64>,
{
    type IndexType = N::IndexType;
    type DimType = N::DimType;

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        let clamped_pos = pos.apply(self.low.clone(), |x, y| if x < y { y } else { x })
            .apply(self.high.clone(), |x, y| if x > y { y } else { x });
        self.noise.value_at(clamped_pos)
    }

    fn dimensions(&self) -> N::DimType {
        self.noise.dimensions()
    }
}

impl<N> WrapInput<N>
where
    N: Noise,
    N::IndexType: PointUtil<f64>,
{
    pub fn new(noise: N, low: N::IndexType, high: N::IndexType) -> WrapInput<N> {
        low.clone().apply(high.clone(), |l, h| {
            assert!(l < h);
            l
        });
        WrapInput { noise, low, high }
    }

    pub fn inner_noise(&self) -> &N {
        &self.noise
    }
}

impl<N> Noise for WrapInput<N>
where
    N: Noise,
    N::IndexType: PointUtil<f64>,
{
    type IndexType = N::IndexType;
    type DimType = N::DimType;

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        let wrapped_pos = pos.apply_3(self.low.clone(), self.high.clone(), |x, low, high| {
            let range = high - low;
            let scaled = x - low;
            let mut wrapped = scaled % range;
            if wrapped < 0.0 {
                wrapped += range;
            }
            wrapped + low
        });
        self.noise.value_at(wrapped_pos)
    }

    fn dimensions(&self) -> N::DimType {
        self.noise.dimensions()
    }
}
