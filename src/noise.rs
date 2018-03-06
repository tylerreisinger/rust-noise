use cgmath::Vector2;

pub trait Noise {
    fn value_at(&self, pos: Vector2<f64>) -> f64;

    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn dimensions(&self) -> (u32, u32) {
        (self.width(), self.height())
    }
}
