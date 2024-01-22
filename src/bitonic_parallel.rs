//! This module contains the implementation of a parallel bitonic sort algorithm.
//!
//! The `bitonic_sort` function sorts a given vector in ascending order using the bitonic sort algorithm.
//! It supports parallel execution by dividing the sorting process into multiple threads.
//!
//! # Examples
//!
//! ```
//! use bitonic_sort::bitonic_parallel::bitonic_sort;
//!
//! let mut nums = vec![4, 2, 7, 1, 5, 3, 6];
//! let parallel = 2;
//! bitonic_sort(&mut nums, parallel);
//! assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7]);
//! ```
/// This module contains the implementation of a parallel bitonic sort algorithm.
///
/// The `bitonic_sort` function sorts a given vector in ascending order using the bitonic sort algorithm.
/// It supports parallel execution by dividing the sorting process into multiple threads.
///
/// # Examples
///
/// ```
/// use bitonic_sort::bitonic_parallel::bitonic_sort;
///
/// let mut nums = vec![4, 2, 7, 1, 5, 3, 6];
/// let parallel = 2;
/// bitonic_sort(&mut nums, parallel);
/// assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7]);
/// ```

pub fn bitonic_sort<T>(nums: &mut Vec<T>, parallel: u8)
where
    T: PartialOrd + Copy + Send + Sync,
{
    if nums.is_empty() {
        return;
    }
    let origin_len = nums.len();
    if !origin_len.is_power_of_two() {
        let max = *nums.iter().fold(
            nums.first().unwrap(),
            |max, x| if max.ge(x) { max } else { x },
        );
        nums.resize(origin_len.next_power_of_two(), max);
    }
    __bitonic_sort(&mut nums[..], false, parallel);
    nums.truncate(origin_len);
}

use std::sync::Arc;
use std::{mem, slice, thread};

struct SliceWrapper<T: ?Sized>(*mut T);
unsafe impl<T> Send for SliceWrapper<T> {}
unsafe impl<T> Sync for SliceWrapper<T> {}
impl<T> Clone for SliceWrapper<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<T> Copy for SliceWrapper<T> {}
fn __bitonic_merge<T>(nums: &mut [T], reverse: bool, mut parallel: u8)
where
    T: PartialOrd + Copy + Send + Sync,
{
    let len = nums.len();
    let size = len / (2 * parallel as usize);
    if size == 0 {
        parallel = (len / 2) as u8;
    }
    let shared_nums = Arc::new(SliceWrapper(nums.as_mut_ptr()));
    thread::scope(|s| {
        for i in 0..parallel as usize {
            let nums = Arc::clone(&shared_nums);
            s.spawn(move || {
                let slice1 = unsafe {
                    slice::from_raw_parts_mut(nums.0, len)
                        .get_unchecked_mut(i * size..(i + 1) * size)
                };
                let slice2 = unsafe {
                    slice::from_raw_parts_mut(nums.0, len)
                        .get_unchecked_mut(len / 2 + i * size..len / 2 + (i + 1) * size)
                };
                for (num1, num2) in slice1.iter_mut().zip(slice2.iter_mut()) {
                    if (num1 > num2) ^ reverse {
                        mem::swap(num1, num2);
                    }
                }
            });
        }
    })
}

fn __bitonic_sort<T>(nums: &mut [T], reverse: bool, parallel: u8)
where
    T: PartialOrd + Copy + Send + Sync,
{
    let len = nums.len();
    if len <= 1 {
        return;
    }
    __bitonic_sort(&mut nums[..len / 2], false, parallel);
    __bitonic_sort(&mut nums[len / 2..], true, parallel);
    let mut size = len;
    while size > 1 {
        for i in 0..len / size {
            __bitonic_merge(&mut nums[i * size..(i + 1) * size], reverse, parallel);
        }
        size /= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitonic_sort() {
        let mut nums = vec![4, 2, 7, 1, 5, 3, 6];
        let parallel = 2;
        bitonic_sort(&mut nums, parallel);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_bitonic_sort_empty() {
        let mut nums: Vec<i32> = vec![];
        let parallel = 2;
        bitonic_sort(&mut nums, parallel);
        assert_eq!(nums, vec![]);
    }

    #[test]
    fn test_bitonic_sort_single_element() {
        let mut nums = vec![42];
        let parallel = 2;
        bitonic_sort(&mut nums, parallel);
        assert_eq!(nums, vec![42]);
    }

    #[test]
    fn test_bitonic_sort_already_sorted() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let parallel = 2;
        bitonic_sort(&mut nums, parallel);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_bitonic_sort_reverse_sorted() {
        let mut nums = vec![7, 6, 5, 4, 3, 2, 1];
        let parallel = 2;
        bitonic_sort(&mut nums, parallel);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_bitonic_sort_duplicate_elements() {
        let mut nums = vec![4, 2, 7, 1, 5, 3, 6, 4, 2, 7, 1, 5, 3, 6];
        let parallel = 2;
        bitonic_sort(&mut nums, parallel);
        assert_eq!(nums, vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7]);
    }

    #[test]
    fn test_bitonic_sort_simple() {
        let mut nums = vec![4, 2, 7, 1, 5, 3, 6];
        let parallel = 2;
        bitonic_sort(&mut nums, parallel);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7]);
    }
}
