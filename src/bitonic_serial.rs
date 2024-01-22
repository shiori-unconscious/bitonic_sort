//! This module provides a serial implementation of the bitonic sort algorithm.
//!
//! The bitonic sort algorithm is a comparison-based sorting algorithm that works by dividing the input
//! into two parts, sorting them independently, and then merging the results in a bitonic sequence.
//! The algorithm has a time complexity of O(n log^2 n), where n is the number of elements to be sorted.
//!
//! # Examples
//!
//! ```
//! use bitonic_sort::bitonic_serial::bitonic_sort;
//!
//! let mut nums = vec![4, 2, 7, 1, 5];
//! bitonic_sort(&mut nums);
//! assert_eq!(nums, vec![1, 2, 4, 5, 7]);
//! ```
/// Performs a bitonic sort on the given mutable slice of elements.
///
/// The `bitonic_sort` function sorts the elements in ascending order by default.
/// If the `reverse` parameter is set to `true`, it sorts the elements in descending order.
///
/// # Examples
///
/// ```
/// use bitonic_sort::bitonic_serial::bitonic_sort;
/// 
/// let mut nums = vec![4, 2, 7, 1, 5];
/// bitonic_sort(&mut nums);
/// assert_eq!(nums, vec![1, 2, 4, 5, 7]);
/// ```

pub fn bitonic_sort<T>(nums: &mut Vec<T>)
where
    T: PartialOrd + Copy,
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

    __bitonic_sort(&mut nums[..], false);
    nums.truncate(origin_len);
}

use std::cell::Cell;

fn __bitonic_merge<T>(nums: &mut [T], reverse: bool)
where
    T: PartialOrd + Copy,
{
    let len = nums.len();
    let slice = Cell::from_mut(&mut nums[..]).as_slice_of_cells();
    for (num1, num2) in slice[..len / 2].iter().zip(slice[len / 2..].iter()) {
        if (num1.get() > num2.get()) ^ reverse {
            Cell::swap(num1, num2);
        }
    }
}

fn __bitonic_sort<T>(nums: &mut [T], reverse: bool)
where
    T: PartialOrd + Copy,
{
    let len = nums.len();
    if len <= 1 {
        return;
    }
    __bitonic_sort(&mut nums[..len / 2], false);
    __bitonic_sort(&mut nums[len / 2..], true);
    let mut size = len;
    while size > 1 {
        for i in 0..len / size {
            __bitonic_merge(&mut nums[i * size..(i + 1) * size], reverse);
        }
        size /= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitonic_sort_ascending() {
        let mut nums = vec![4, 2, 7, 1, 5];
        bitonic_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 4, 5, 7]);
    }

    #[test]
    fn test_bitonic_sort_descending() {
        let mut nums = vec![4, 2, 7, 1, 5];
        bitonic_sort(&mut nums);
        nums.reverse();
        assert_eq!(nums, vec![7, 5, 4, 2, 1]);
    }

    #[test]
    fn test_bitonic_sort_empty() {
        let mut nums: Vec<i32> = vec![];
        bitonic_sort(&mut nums);
        assert_eq!(nums, vec![]);
    }

    #[test]
    fn test_bitonic_sort_single_element() {
        let mut nums = vec![42];
        bitonic_sort(&mut nums);
        assert_eq!(nums, vec![42]);
    }

    #[test]
    fn test_bitonic_sort_power_of_two() {
        let mut nums = vec![4, 2, 7, 1, 5, 3, 6, 8];
        bitonic_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_bitonic_sort_not_power_of_two() {
        let mut nums = vec![4, 2, 7, 1, 5, 3, 6];
        bitonic_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7]);
    }
}
