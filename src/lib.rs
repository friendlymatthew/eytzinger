#![feature(core_intrinsics)]

use core::intrinsics::prefetch_read_data;
use std::mem::MaybeUninit;

#[repr(align(64))]
struct AlignedVec<T>(Vec<MaybeUninit<T>>);

pub struct EytzingerVec<T: Ord + Copy>(AlignedVec<T>);

impl<T: Ord + Copy> EytzingerVec<T> {
    pub fn new() -> EytzingerVec<T> {
        Self(AlignedVec(Vec::new()))
    }

    pub fn from_slice(a: &[T]) -> Self {
        let capacity = a.len() + 1;
        let mut b = Vec::with_capacity(capacity);
        unsafe {
            b.set_len(capacity);
        }
        Self::construct(a, &mut b, 0, 1);
        Self(AlignedVec(b))
    }

    fn construct(a: &[T], b: &mut [MaybeUninit<T>], i: usize, k: usize) -> usize {
        match k < b.len() {
            true => {
                let mut idx = Self::construct(a, b, i, 2 * k);
                b[k].write(a[idx]); // man i hate this copy
                idx += 1;
                idx = Self::construct(a, b, idx, 2 * k + 1);
                idx
            }
            false => i,
        }
    }

    pub fn search(&self, x: &T) -> usize {
        let mut k = 1;
        let n = self.0 .0.len();

        while k <= n {
            unsafe {
                let prefetch_ptr = self.0 .0.as_ptr().wrapping_offset(2 * k as isize);
                prefetch_read_data(prefetch_ptr, 3);
            }

            k = 2 * k + (unsafe { self.0 .0[k].assume_init() } < *x) as usize;
        }

        k >>= k.trailing_ones() + 1;
        let current = unsafe { self.0 .0[k].assume_init() };
        k * (current == *x) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search() {
        let dat = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let eyt = EytzingerVec::from_slice(&dat);

        let res = eyt.search(&8);
        assert_eq!(res, 1);

        let res = eyt.search(&2);
        assert_eq!(res, 4);

        let res = eyt.search(&69);
        assert_eq!(res, 0);
    }
}
