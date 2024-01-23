use bitonic_sort::bitonic_parallel;
use bitonic_sort::bitonic_serial;
use bitonic_sort::parallel_sort;
use criterion::{criterion_group, criterion_main, Criterion};
use rand;
use rand::Rng;

fn benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let data: Vec<f64> = (0..1_000_000)
        .map(|_| rng.gen_range(-1145141919.810..1145141919.810))
        .collect();

    c.bench_function("Parallel Sort", |b| {
        b.iter(|| {
            parallel_sort::parallel_sort(&mut data.clone(), 8);
        })
    });

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
            cloned_data.sort_unstable_by(|x, y| x.partial_cmp(y).expect("float error"));
        })
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
