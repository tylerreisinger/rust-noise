pub trait Noise {
    type IndexType: Clone;
    type DimType;

    fn value_at(&self, pos: Self::IndexType) -> f64;

    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn dimensions(&self) -> Self::DimType;
}
