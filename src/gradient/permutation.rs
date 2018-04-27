use std::ops::Index;

use rand::Rng;

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
