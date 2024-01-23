use std::sync::Arc;
use std::{slice, thread};
struct Wrap<T: ?Sized>(*mut T);
unsafe impl<T> Send for Wrap<T> {}
unsafe impl<T> Sync for Wrap<T> {}

pub fn parallel_sort<T>(nums: &mut Vec<T>, mut parallel: u8)
where
    T: PartialOrd + Send + Sync + Copy,
{
    if nums.is_empty() {
        return;
    }
    let origin_len = nums.len();
    if !origin_len.is_power_of_two() {
        let max = *nums.iter().fold(
            nums.first().unwrap(),
            |max, x| if max > x { max } else { x },
        );
        nums.resize(origin_len.next_power_of_two(), max);
    }
    let len = nums.len();
    parallel = if parallel < 1 {
        1
    } else {
        parallel.checked_next_power_of_two().unwrap_or(u8::MAX)
    };
    let mut size = len / parallel as usize;
    if size < 1 {
        size = 1;
        parallel = len as u8;
    }
    let shared_ptr = Arc::new(Wrap(nums.as_mut_ptr()));
    thread::scope(|s| {
        let mut handles = Vec::new();
        for i in 0..parallel as usize {
            let shared_ptr = shared_ptr.clone();
            handles.push(s.spawn(move || {
                let shared_slice = unsafe { slice::from_raw_parts_mut(shared_ptr.0, len) };
                shared_slice[i * size..(i + 1) * size]
                    .sort_unstable_by(|x, y| x.partial_cmp(y).expect("float error!"));
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
        while parallel > 1 {
            parallel /= 2;
            size *= 2;
            let mut handles = Vec::new();
            for i in 0..parallel as usize {
                let shared_ptr = shared_ptr.clone();
                handles.push(s.spawn(move || {
                    let shared_slice = unsafe {
                        slice::from_raw_parts_mut(shared_ptr.0, len)
                            .get_unchecked_mut(i * size..(i + 1) * size)
                    };
                    let mut tmp = Vec::with_capacity(size);
                    let (lb, rb) = (size / 2, size);
                    let (mut l, mut r) = (0, size / 2);
                    while l < lb && r < rb {
                        if shared_slice[l] <= shared_slice[r] {
                            tmp.push(shared_slice[l].clone());
                            l += 1;
                        } else {
                            tmp.push(shared_slice[r].clone());
                            r += 1;
                        }
                    }
                    tmp.extend_from_slice(&shared_slice[l..lb]);
                    tmp.extend_from_slice(&shared_slice[r..rb]);
                    shared_slice.copy_from_slice(&tmp[..]);
                }));
            }
            for handle in handles {
                handle.join().unwrap();
            }
        }
    });
    nums.truncate(origin_len);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_sort() {
        let mut nums = vec![4, 2, 7, 1, 5, 3, 6];
        let parallel = 2;
        parallel_sort(&mut nums, parallel);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_parallel_sort_empty() {
        let mut nums: Vec<i32> = vec![];
        let parallel = 2;
        parallel_sort(&mut nums, parallel);
        assert_eq!(nums, vec![]);
    }

    #[test]
    fn test_parallel_sort_single_element() {
        let mut nums = vec![42];
        let parallel = 2;
        parallel_sort(&mut nums, parallel);
        assert_eq!(nums, vec![42]);
    }

    #[test]
    fn test_parallel_sort_already_sorted() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let parallel = 2;
        parallel_sort(&mut nums, parallel);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_parallel_sort_reverse_sorted() {
        let mut nums = vec![7, 6, 5, 4, 3, 2, 1];
        let parallel = 2;
        parallel_sort(&mut nums, parallel);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_parallel_sort_duplicate_elements() {
        let mut nums = vec![4, 2, 7, 1, 5, 3, 6, 4, 2, 7, 1, 5, 3, 6];
        let parallel = 2;
        parallel_sort(&mut nums, parallel);
        assert_eq!(nums, vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7]);
    }

    #[test]
    fn test_parallel_sort_simple() {
        let mut nums = vec![4, 2, 7, 1, 5, 3, 6];
        let parallel = 2;
        parallel_sort(&mut nums, parallel);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7]);
    }
}
