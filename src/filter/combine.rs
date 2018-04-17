use noise::Noise;

pub trait TupleMax<T> {
    fn max(&self, other: &Self) -> Self;
}

impl TupleMax<u32> for (u32,) {
    fn max(&self, rhs: &(u32,)) -> (u32,) {
        (self.0.max(rhs.0),)
    }
}
impl TupleMax<u32> for (u32, u32) {
    fn max(&self, rhs: &(u32, u32)) -> (u32, u32) {
        (self.0.max(rhs.0), self.1.max(rhs.1))
    }
}
impl TupleMax<u32> for (u32, u32, u32) {
    fn max(&self, rhs: &(u32, u32, u32)) -> (u32, u32, u32) {
        (self.0.max(rhs.0), self.1.max(rhs.1), self.2.max(rhs.2))
    }
}
impl TupleMax<u32> for (u32, u32, u32, u32) {
    fn max(&self, rhs: &(u32, u32, u32, u32)) -> (u32, u32, u32, u32) {
        (
            self.0.max(rhs.0),
            self.1.max(rhs.1),
            self.2.max(rhs.2),
            self.3.max(rhs.3),
        )
    }
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
    N1::DimType: TupleMax<u32>,
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
