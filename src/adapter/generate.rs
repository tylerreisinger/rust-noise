use std::marker::PhantomData;
use noise::Noise;
use noise::TupleUtil;

#[derive(Clone, Debug)]
pub struct Constant<I, D> {
    value: f64,
    _phantom1: PhantomData<I>,
    _phantom2: PhantomData<D>,
}

#[derive(Clone, Debug)]
pub struct FunctionValue<I, D, F>
where
    I: Clone,
    D: Default + TupleUtil<f64>,
    F: Fn(&I) -> f64,
{
    function: F,
    _phantom1: PhantomData<I>,
    _phantom2: PhantomData<D>,
}

impl<I, D> Constant<I, D>
where
    I: Clone,
    D: Default + TupleUtil<f64>,
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
    D: Default + TupleUtil<f64>,
{
    type IndexType = I;
    type DimType = D;

    fn value_at(&self, _: Self::IndexType) -> f64 {
        self.value
    }
    fn frequency(&self) -> Self::DimType {
        Self::DimType::saturate(1.0)
    }
}

impl<I, D, F> FunctionValue<I, D, F>
where
    I: Clone,
    D: Default + TupleUtil<f64>,
    F: Fn(&I) -> f64,
{
    pub fn new(function: F) -> FunctionValue<I, D, F> {
        FunctionValue {
            function,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }

    pub fn function(&self) -> &F {
        &self.function
    }
}

impl<I, D, F> Noise for FunctionValue<I, D, F>
where
    I: Clone,
    D: Default + TupleUtil<f64>,
    F: Fn(&I) -> f64,
{
    type IndexType = I;
    type DimType = D;

    fn value_at(&self, index: Self::IndexType) -> f64 {
        let f = &self.function;
        f(&index)
    }
    fn frequency(&self) -> Self::DimType {
        Self::DimType::saturate(1.0)
    }
}
