use std::marker::PhantomData;
use noise::Noise;

#[derive(Clone, Debug)]
pub struct Constant<I, D> {
    value: f64,
    _phantom1: PhantomData<I>,
    _phantom2: PhantomData<D>,
}

impl<I, D> Constant<I, D>
where
    I: Clone,
{
    pub fn new(value: f64) -> Constant<I, D> {
        Constant {
            value,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

impl<I, D> Noise for Constant<I, D>
where
    I: Clone,
    D: Default,
{
    type IndexType = I;
    type DimType = D;

    fn value_at(&self, _: Self::IndexType) -> f64 {
        self.value
    }
    fn width(&self) -> u32 {
        1
    }
    fn height(&self) -> u32 {
        1
    }
    fn dimensions(&self) -> Self::DimType {
        Default::default()
    }
}
