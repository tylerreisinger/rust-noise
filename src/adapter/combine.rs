use noise::Noise;
use super::TupleUtil;

#[derive(Debug, Clone)]
pub struct Add<N1, N2>
where
    N1: Noise,
    N2: Noise<IndexType = N1::IndexType, DimType = N1::DimType>,
{
    left_noise: N1,
    right_noise: N2,
}

#[derive(Debug, Clone)]
pub struct Combine<N1, N2, F>
where
    N1: Noise,
    N2: Noise<IndexType = N1::IndexType, DimType = N1::DimType>,
    F: Fn(f64, f64) -> f64,
{
    left_noise: N1,
    right_noise: N2,
    combiner: F,
}

impl<N1, N2, F> Combine<N1, N2, F>
where
    N1: Noise,
    N2: Noise<IndexType = N1::IndexType, DimType = N1::DimType>,
    F: Fn(f64, f64) -> f64,
{
    pub fn new(left_noise: N1, right_noise: N2, combiner: F) -> Combine<N1, N2, F> {
        Combine {
            left_noise,
            right_noise,
            combiner,
        }
    }

    pub fn left_noise(&self) -> &N1 {
        &self.left_noise
    }
    pub fn right_noise(&self) -> &N2 {
        &self.right_noise
    }
    pub fn combiner_fn(&self) -> &F {
        &self.combiner
    }
}

impl<N1, N2, F> Noise for Combine<N1, N2, F>
where
    N1: Noise,
    N1::DimType: TupleUtil<u32>,
    N2: Noise<IndexType = N1::IndexType, DimType = N1::DimType>,
    F: Fn(f64, f64) -> f64,
{
    type IndexType = N1::IndexType;
    type DimType = N1::DimType;

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        let f = &self.combiner;
        f(
            self.left_noise.value_at(pos.clone()),
            self.right_noise.value_at(pos),
        )
    }
    fn width(&self) -> u32 {
        u32::max(self.left_noise.width(), self.right_noise.width())
    }
    fn height(&self) -> u32 {
        u32::max(self.left_noise.height(), self.right_noise.height())
    }
    fn dimensions(&self) -> Self::DimType {
        self.left_noise
            .dimensions()
            .max(&self.right_noise.dimensions())
    }
}

impl<N1, N2> Add<N1, N2>
where
    N1: Noise,
    N1::DimType: TupleUtil<u32>,
    N2: Noise<IndexType = N1::IndexType, DimType = N1::DimType>,
{
    pub fn new(left_noise: N1, right_noise: N2) -> Add<N1, N2> {
        Add {
            left_noise,
            right_noise,
        }
    }

    pub fn left_noise(&self) -> &N1 {
        &self.left_noise
    }
    pub fn right_noise(&self) -> &N2 {
        &self.right_noise
    }
}

impl<N1, N2> Noise for Add<N1, N2>
where
    N1: Noise,
    N1::DimType: TupleUtil<u32>,
    N2: Noise<IndexType = N1::IndexType, DimType = N1::DimType>,
{
    type IndexType = N1::IndexType;
    type DimType = N1::DimType;

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        self.left_noise.value_at(pos.clone()) + self.right_noise.value_at(pos)
    }
    fn width(&self) -> u32 {
        u32::max(self.left_noise.width(), self.right_noise.width())
    }
    fn height(&self) -> u32 {
        u32::max(self.left_noise.height(), self.right_noise.height())
    }
    fn dimensions(&self) -> Self::DimType {
        self.left_noise
            .dimensions()
            .max(&self.right_noise.dimensions())
    }
}
