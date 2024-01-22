use bitonic_sort::bitonic_parallel;
use bitonic_sort::bitonic_serial;
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion) {
    let data: Vec<u32> = (0..1_000).rev().collect();

    c.bench_function("Serial Bitonic Sort", |b| {
        b.iter(|| {
            bitonic_serial::bitonic_sort(&mut data.clone());
        })
    });
    
    c.bench_function("Parallel Bitonic Sort", |b| {
        b.iter(|| {
            bitonic_parallel::bitonic_sort(&mut data.clone(), 16);
        })
    });

    c.bench_function("Standard Library Sort", |b| {
        b.iter(|| {
            let mut cloned_data = data.clone();
            cloned_data.sort_unstable();
        })
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
