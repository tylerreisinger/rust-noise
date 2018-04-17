use noise::Noise;

#[derive(Copy, Clone, Debug)]
pub enum FilterKind {
    LowPass,
    HighPass,
}

#[derive(Debug, Clone)]
pub struct Clamp<N>
where
    N: Noise,
{
    noise: N,
    low: f64,
    high: f64,
}

#[derive(Debug, Clone)]
pub struct Filter<N, F>
where
    N: Noise,
    F: Fn(f64, f64, f64) -> f64,
{
    noise: N,
    start: f64,
    end: f64,
    kind: FilterKind,
    blend_fn: F,
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
    fn dimensions(&self) -> Self::DimType {
        self.noise.dimensions()
    }
}

impl<N, F> Filter<N, F>
where
    N: Noise,
    F: Fn(f64, f64, f64) -> f64,
{
    pub fn new(noise: N, start: f64, end: f64, kind: FilterKind, blend_fn: F) -> Filter<N, F> {
        assert!(start < end);
        Filter {
            noise,
            start,
            end,
            kind,
            blend_fn,
        }
    }

    pub fn inner_noise(&self) -> &N {
        &self.noise
    }

    pub fn kind(&self) -> FilterKind {
        self.kind
    }

    pub fn start(&self) -> f64 {
        self.start
    }
    pub fn end(&self) -> f64 {
        self.end
    }

    pub fn blend_fn(&self) -> &F {
        &self.blend_fn
    }
}

impl<N, F> Noise for Filter<N, F>
where
    N: Noise,
    F: Fn(f64, f64, f64) -> f64,
{
    type IndexType = N::IndexType;
    type DimType = N::DimType;

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        let val = self.noise.value_at(pos);

        let (x1, x2) = match self.kind {
            FilterKind::LowPass => (val, 0.0),
            FilterKind::HighPass => (0.0, val),
        };

        if val > self.start {
            if val < self.end {
                let t = (val - self.start) / (self.end - self.start);
                let f = &self.blend_fn;
                f(x1, x2, t)
            } else {
                x2
            }
        } else {
            x1
        }
    }
    fn dimensions(&self) -> Self::DimType {
        self.noise.dimensions()
    }
}
