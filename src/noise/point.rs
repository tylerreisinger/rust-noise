pub type Point1<T> = T;
pub type Point2<T> = [T; 2];
pub type Point3<T> = [T; 3];
pub type Point4<T> = [T; 4];

pub trait PointUtil<T> {
    fn apply<F>(self, rhs: Self, f: F) -> Self
    where
        F: Fn(T, T) -> T;
    fn saturate(val: T) -> Self;
}

impl<T: Copy> PointUtil<T> for Point1<T> {
    fn apply<F>(self, rhs: Self, f: F) -> Self
    where
        F: Fn(T, T) -> T,
    {
        f(self, rhs)
    }
    fn saturate(val: T) -> Self {
        val
    }
}
impl<T: Copy> PointUtil<T> for Point2<T> {
    fn apply<F>(self, rhs: Self, f: F) -> Self
    where
        F: Fn(T, T) -> T,
    {
        [f(self[0], rhs[0]), f(self[1], rhs[1])]
    }
    fn saturate(val: T) -> Self {
        [val, val]
    }
}
impl<T: Copy> PointUtil<T> for Point3<T> {
    fn apply<F>(self, rhs: Self, f: F) -> Self
    where
        F: Fn(T, T) -> T,
    {
        [f(self[0], rhs[0]), f(self[1], rhs[1]), f(self[2], rhs[2])]
    }
    fn saturate(val: T) -> Self {
        [val, val, val]
    }
}
impl<T: Copy> PointUtil<T> for Point4<T> {
    fn apply<F>(self, rhs: Self, f: F) -> Self
    where
        F: Fn(T, T) -> T,
    {
        [
            f(self[0], rhs[0]),
            f(self[1], rhs[1]),
            f(self[2], rhs[2]),
            f(self[3], rhs[3]),
        ]
    }
    fn saturate(val: T) -> Self {
        [val, val, val, val]
    }
}
