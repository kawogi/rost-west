use std::{array, ops::Index};

use crate::lanczos_doubler::LanczosSampler;

pub struct DataSeries<T, const BITS: u32> {
    data: Box<[T]>,
}

impl<T, const SIZE_BITS: u32> DataSeries<T, SIZE_BITS> {
    /// length of the 1D-array
    pub const SIZE: u32 = 1_u32 << SIZE_BITS;

    /// bit-mask to wrap x and y so that they are guaranteed to be in bounds
    pub const SIZE_MASK: u16 = Self::SIZE.wrapping_sub(1) as u16;

    /// total number of data points within the array
    pub const LEN: usize = 1_usize << SIZE_BITS;

    /// number of right shifts to perform in order to map a world coordinate onto this data square
    pub const WORLD_SHIFT: u32 = u16::BITS - SIZE_BITS;

    /// create a new empty list
    pub fn new(fill: T) -> Self
    where
        T: Clone,
    {
        assert!(SIZE_BITS <= u16::BITS);
        Self {
            data: (vec![fill; Self::LEN]).into_boxed_slice(),
        }
    }

    /// This is a convenience function to get the internal data length of an instance without having to
    /// type out the entire type name.
    #[inline]
    #[expect(clippy::len_without_is_empty, reason = "it's never empty")]
    pub const fn len(&self) -> usize {
        Self::LEN
    }

    #[inline]
    pub fn offset(x: u16) -> usize {
        usize::from(Self::wrap(x))
    }

    #[inline]
    pub fn wrap(x: u16) -> u16 {
        x & Self::SIZE_MASK
    }

    #[inline]
    pub fn set(&mut self, x: u16, value: T) {
        self.data[Self::offset(x)] = value;
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.data.iter_mut()
    }

    pub fn clone_with(&self, map: impl Fn(&T) -> T) -> Self {
        Self {
            data: self.data.iter().map(map).collect(),
        }
    }
    // }

    // impl<T, const BITS: u32> GetWrapping for DataSeries<T, BITS>
    // where
    //     T: Copy,
    // {
    //     type Item = T;

    #[inline]
    pub fn get(&self, x: u16) -> T
    where
        T: Copy,
    {
        self.data[Self::offset(x)]
    }

    pub fn get_2(&self, x: u16) -> [T; 2]
    where
        T: Copy,
    {
        let left = Self::offset(x);
        let right = Self::offset(x.wrapping_add(1));
        [self.data[left], self.data[right]]
    }

    pub fn get_3(&self, x: u16) -> [T; 3]
    where
        T: Copy,
    {
        let left = Self::offset(x.wrapping_sub(1));
        let center = Self::offset(x);
        let right = Self::offset(x.wrapping_add(1));
        [self.data[left], self.data[center], self.data[right]]
    }
}

impl<T, const BITS: u32> AsRef<[T]> for DataSeries<T, BITS> {
    fn as_ref(&self) -> &[T] {
        &self.data
    }
}

impl<T, const BITS: u32> AsMut<[T]> for DataSeries<T, BITS> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}

impl<T, const BITS: u32> Default for DataSeries<T, BITS>
where
    T: Default + Clone,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T, const SIZE_BITS: u32> Index<usize> for DataSeries<T, SIZE_BITS> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const SIZE_BITS: u32> TryFrom<Vec<T>> for DataSeries<T, SIZE_BITS> {
    type Error = String;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if value.len() != Self::LEN {
            return Err(format!(
                "Expected a vector containing {expected} but got one with {actual} entries.",
                expected = Self::LEN,
                actual = value.len()
            ));
        }
        Ok(Self {
            data: value.into_boxed_slice(),
        })
    }
}

// pub trait GetWrapping {
//     type Item: Copy;

//     fn get(&self, xy: [u16; 2]) -> Self::Item;
//     fn get_2x2(&self, xy: [u16; 2]) -> [Self::Item; 4];
//     fn get_3x3(&self, xy: [u16; 2]) -> [Self::Item; 9];
// }

pub struct Sampler1d<'sampler, 'data, const SIZE_BITS: u32, const WINDOW_SIZE: usize> {
    sampler: &'sampler LanczosSampler<WINDOW_SIZE>,
    data: &'data DataSeries<f32, SIZE_BITS>,
}

impl<'sampler, 'data, const SIZE_BITS: u32, const WINDOW_SIZE: usize>
    Sampler1d<'sampler, 'data, SIZE_BITS, WINDOW_SIZE>
{
    const WINDOW_OFFSET: u16 = LanczosSampler::<WINDOW_SIZE>::WINDOW_OFFSET as u16;
    const WORLD_SHIFT: u32 = DataSeries::<f32, SIZE_BITS>::WORLD_SHIFT;

    pub fn new(
        data: &'data DataSeries<f32, SIZE_BITS>,
        sampler: &'sampler LanczosSampler<WINDOW_SIZE>,
    ) -> Self {
        assert_eq!(sampler.resolution_bits(), Self::WORLD_SHIFT, "the samplers resolution and the map's size (in bits) need to add up to the width of the world coordinates (u16)");
        Self { data, sampler }
    }

    pub fn sample(&self, world_x: u16) -> f32 {
        let x = world_x >> Self::WORLD_SHIFT;

        let row_values = array::from_fn(|col_index| {
            let offset = DataSeries::<f32, SIZE_BITS>::offset(
                x.wrapping_add(col_index as u16)
                    .wrapping_sub(Self::WINDOW_OFFSET),
            );
            self.data[offset]
        });
        self.sampler.get(world_x, &row_values)
    }
}
