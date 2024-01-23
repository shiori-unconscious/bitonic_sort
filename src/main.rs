use bitonic_sort::{bitonic_parallel, bitonic_serial, parallel_sort};

fn main() {
    let mut nums = vec![3.1, 8.2, 3.4, 2.22, 4.44];
    bitonic_serial::bitonic_sort(&mut nums);
    println!("{:?}", nums);
    let mut nums = vec![3.1, 8.2, 3.4, 2.22, 4.44];
    bitonic_parallel::bitonic_sort(&mut nums, 2);
    println!("{:?}", nums);
    let mut nums = vec![3.1, 8.2, 3.4, 2.22, 4.44];
    parallel_sort::parallel_sort(&mut nums, 8);
    println!("{:?}", nums);
}
