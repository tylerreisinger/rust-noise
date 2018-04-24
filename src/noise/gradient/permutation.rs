use std::u8;
use std::ops::Index;

use rand::Rng;
use cgmath::Vector2;

const INV_SQRT_2: f64 = 0.7071067811865475244008443621048490392848359376884740;

pub fn get_2d_gradient(hash: u8) -> Vector2<f64> {
    match hash % 12 {
        0 | 4 => Vector2::new(1.0, 0.0),
        1 | 5 => Vector2::new(0.0, 1.0),
        2 | 6 => Vector2::new(-1.0, 0.0),
        3 | 7 => Vector2::new(0.0, -1.0),
        8 => Vector2::new(INV_SQRT_2, INV_SQRT_2),
        9 => Vector2::new(-INV_SQRT_2, INV_SQRT_2),
        10 => Vector2::new(-INV_SQRT_2, -INV_SQRT_2),
        11 => Vector2::new(INV_SQRT_2, -INV_SQRT_2),
        _ => unreachable!(),
    }
}

#[derive(Clone, Debug)]
pub struct PermutationTable {
    table: Vec<u32>,
}

impl PermutationTable {
    pub fn new<R>(rng: &mut R, size: u32) -> PermutationTable
    where
        R: Rng,
    {
        assert!(size > 0);
        let mut table: Vec<_> = (0..size).map(|x| x % size).collect();
        rng.shuffle(&mut table);

        PermutationTable { table }
    }

    #[inline]
    pub fn values(&self) -> &[u32] {
        &self.table
    }

    #[inline]
    pub unsafe fn get_unchecked(&self, index: u32) -> u32 {
        *self.table.get_unchecked(index as usize)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.table.len()
    }
}

impl Index<u32> for PermutationTable {
    type Output = u32;

    #[inline]
    fn index(&self, index: u32) -> &u32 {
        &self.table[index as usize]
    }
}
