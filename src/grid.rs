use std::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
pub struct Grid<T> {
    data: Vec<T>,
    width: u32,
    height: u32,
}

impl<T> Grid<T>
where
    T: Clone + Default,
{
    pub fn new(width: u32, height: u32) -> Grid<T> {
        let size = (width * height) as usize;
        let mut data = Vec::with_capacity(size);
        data.resize(size, T::default());
        Grid {
            data,
            width,
            height,
        }
    }
}

impl<T> Grid<T> {
    pub fn with_data(width: u32, height: u32, data: Vec<T>) -> Grid<T> {
        assert_eq!(data.len(), (width * height) as usize);

        Grid {
            data,
            width,
            height,
        }
    }
    pub fn index_from_coords(&self, coords: (usize, usize)) -> usize {
        let (x, y) = coords;
        x + y * (self.width as usize)
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn size(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }
    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let idx = self.index_from_coords(index);
        &self.data[idx]
    }
}
impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let idx = self.index_from_coords(index);
        &mut self.data[idx]
    }
}
