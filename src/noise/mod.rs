pub mod fbm;
pub mod perlin;
pub mod point;
pub mod octave;

pub use noise::perlin::{Perlin1d, Perlin2d, Perlin3d};
pub use noise::octave::{Octave, OctaveNoise};
pub use noise::point::{Point1, Point2, Point3, Point4, PointUtil};

use adapter::{Extension2d, Extension3d, Slice1d, Slice2d};

pub trait WithFrequency: Noise {
    fn with_frequency(self, frequency: Self::DimType) -> Self;
}

pub trait Noise {
    type IndexType: Clone;
    type DimType;

    fn value_at(&self, pos: Self::IndexType) -> f64;

    fn frequency(&self) -> Self::DimType;
}

pub trait Noise1d: Noise<IndexType = Point1<f64>, DimType = f64> + Sized {
    fn width(&self) -> f64 {
        self.frequency()
    }

    fn extend(self) -> Extension2d<Self> {
        Extension2d::new(self)
    }
}
pub trait Noise2d: Noise<IndexType = Point2<f64>, DimType = (f64, f64)> + Sized {
    fn width(&self) -> f64 {
        self.frequency().0
    }
    fn height(&self) -> f64 {
        self.frequency().1
    }

    fn slice(self, height: f64) -> Slice1d<Self> {
        Slice1d::new(self, height)
    }
    fn extend(self) -> Extension3d<Self> {
        Extension3d::new(self)
    }
}
pub trait Noise3d
    : Noise<IndexType = Point3<f64>, DimType = (f64, f64, f64)> + Sized {
    fn width(&self) -> f64 {
        self.frequency().0
    }
    fn height(&self) -> f64 {
        self.frequency().1
    }
    fn depth(&self) -> f64 {
        self.frequency().2
    }

    fn slice(self, depth: f64) -> Slice2d<Self> {
        Slice2d::new(self, depth)
    }
}

impl<'a, N> Noise for &'a N
where
    N: Noise + 'a,
{
    type IndexType = N::IndexType;
    type DimType = N::DimType;

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        (*self).value_at(pos)
    }
    fn frequency(&self) -> Self::DimType {
        (*self).frequency()
    }
}

impl<'a, N> Noise for &'a mut N
where
    N: Noise + 'a,
{
    type IndexType = N::IndexType;
    type DimType = N::DimType;

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        (**self).value_at(pos)
    }
    fn frequency(&self) -> Self::DimType {
        (**self).frequency()
    }
}

impl<N> Noise for Box<N>
where
    N: Noise,
{
    type IndexType = N::IndexType;
    type DimType = N::DimType;

    fn value_at(&self, pos: Self::IndexType) -> f64 {
        (**self).value_at(pos)
    }
    fn frequency(&self) -> Self::DimType {
        (**self).frequency()
    }
}

impl<N> Noise1d for N
where
    N: Noise<IndexType = Point1<f64>, DimType = f64>,
{
}
impl<N> Noise2d for N
where
    N: Noise<IndexType = Point2<f64>, DimType = (f64, f64)>,
{
}
impl<N> Noise3d for N
where
    N: Noise<IndexType = Point3<f64>, DimType = (f64, f64, f64)>,
{
}

pub trait ToTuple<T = Self> {
    type Output;
    type Unwrap;

    fn to_tuple(self) -> Self::Output;
    fn unwrap(self) -> Self::Unwrap;
}

impl ToTuple for f64 {
    type Output = (f64,);
    type Unwrap = Self;
    fn to_tuple(self) -> (f64,) {
        (self,)
    }
    fn unwrap(self) -> Self {
        self
    }
}
impl ToTuple for u32 {
    type Output = (u32,);
    type Unwrap = Self;
    fn to_tuple(self) -> (u32,) {
        (self,)
    }
    fn unwrap(self) -> Self {
        self
    }
}
impl<T> ToTuple for (T,) {
    type Output = Self;
    type Unwrap = T;
    fn to_tuple(self) -> Self {
        self
    }
    fn unwrap(self) -> T {
        self.0
    }
}
impl<T> ToTuple for (T, T) {
    type Output = Self;
    type Unwrap = Self;
    fn to_tuple(self) -> Self {
        self
    }
    fn unwrap(self) -> Self {
        self
    }
}
impl<T> ToTuple for (T, T, T) {
    type Output = Self;
    type Unwrap = Self;
    fn to_tuple(self) -> Self {
        self
    }
    fn unwrap(self) -> Self {
        self
    }
}
impl<T> ToTuple for (T, T, T, T) {
    type Output = Self;
    type Unwrap = Self;
    fn to_tuple(self) -> Self {
        self
    }
    fn unwrap(self) -> Self {
        self
    }
}

pub trait TupleUtil<T> {
    fn max(&self, other: &Self) -> Self;
    fn saturate(val: f64) -> Self;
    fn apply<F>(self, rhs: Self, f: F) -> Self
    where
        F: Fn(T, T) -> T;
}

pub trait TupleMap<T, U>: TupleUtil<T> {
    type Output;

    fn map<F>(self, f: F) -> Self::Output
    where
        F: Fn(T) -> U;
}

impl TupleUtil<f64> for f64 {
    fn max(&self, rhs: &f64) -> f64 {
        f64::max(*self, *rhs)
    }
    fn saturate(val: f64) -> f64 {
        val
    }
    fn apply<F>(self, rhs: Self, f: F) -> Self
    where
        F: Fn(f64, f64) -> f64,
    {
        f(self, rhs)
    }
}

impl TupleUtil<f64> for (f64,) {
    fn max(&self, rhs: &(f64,)) -> (f64,) {
        (self.0.max(rhs.0),)
    }
    fn saturate(val: f64) -> (f64,) {
        (val,)
    }
    fn apply<F>(self, rhs: Self, f: F) -> Self
    where
        F: Fn(f64, f64) -> f64,
    {
        (f(self.0, rhs.0),)
    }
}

impl TupleUtil<f64> for (f64, f64) {
    fn max(&self, rhs: &(f64, f64)) -> (f64, f64) {
        (self.0.max(rhs.0), self.1.max(rhs.1))
    }
    fn saturate(val: f64) -> (f64, f64) {
        (val, val)
    }
    fn apply<F>(self, rhs: Self, f: F) -> Self
    where
        F: Fn(f64, f64) -> f64,
    {
        (f(self.0, rhs.0), f(self.1, rhs.1))
    }
}
impl TupleUtil<f64> for (f64, f64, f64) {
    fn max(&self, rhs: &(f64, f64, f64)) -> (f64, f64, f64) {
        (self.0.max(rhs.0), self.1.max(rhs.1), self.2.max(rhs.2))
    }
    fn saturate(val: f64) -> (f64, f64, f64) {
        (val, val, val)
    }
    fn apply<F>(self, rhs: Self, f: F) -> Self
    where
        F: Fn(f64, f64) -> f64,
    {
        (f(self.0, rhs.0), f(self.1, rhs.1), f(self.2, rhs.2))
    }
}
impl TupleUtil<f64> for (f64, f64, f64, f64) {
    fn max(&self, rhs: &(f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
        (
            self.0.max(rhs.0),
            self.1.max(rhs.1),
            self.2.max(rhs.2),
            self.3.max(rhs.3),
        )
    }
    fn saturate(val: f64) -> (f64, f64, f64, f64) {
        (val, val, val, val)
    }
    fn apply<F>(self, rhs: Self, f: F) -> Self
    where
        F: Fn(f64, f64) -> f64,
    {
        (
            f(self.0, rhs.0),
            f(self.1, rhs.1),
            f(self.2, rhs.2),
            f(self.3, rhs.3),
        )
    }
}
