use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

const N: u128 = 20;

fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("no hotreload fibonacci", |b| {
        b.iter(|| benches::no_hotreload_fibonacci(black_box(N)))
    });

    c.bench_function("default hotreload fibonacci", |b| {
        b.iter(|| benches::default_hotreload_fibonacci(black_box(N)))
    });

    c.bench_function("runtime hotreload fibonacci", |b| {
        b.iter(|| benches::runtime_hotreload_fibonacci(black_box(N)))
    });
}

criterion_group!(benches, fibonacci_benchmark);
criterion_main!(benches);
