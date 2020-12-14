use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ffl_utils::aggregates::{async_min, async_max, async_n_window_sma, async_price_diff};

fn min_benchmark(c: &mut Criterion) {
    let _vec = vec![4.1, 5.1, 6.1, 7.1, 8.1, 9.1, -2.1];
    c.bench_function("async min", |b| b.iter(|| async_min(black_box(&_vec))));
}

fn max_benchmark(c: &mut Criterion) {
    let _vec = vec![4.1, 5.1, 6.1, 7.1, 8.1, 9.1, -2.1];
    c.bench_function("async max", |b| b.iter(|| async_max(black_box(&_vec))));
}

fn change_benchmark(c: &mut Criterion) {
    let _vec: Vec<f64> = vec![1.3, 2.3, 3.3, 4.3, 5.3, 6.3, 7.3, 8.3, 9.3, 10.3];
    c.bench_function("async change", |b| b.iter(|| async_price_diff(black_box(&_vec))));
}

fn n_window_sma_benchmark(c: &mut Criterion) {
    let _vec: Vec<f64> = vec![1.3, 2.3, 3.3, 4.3, 5.3, 6.3, 7.3, 8.3, 9.3, 10.3];
    c.bench_function("async n_window_sma", |b| b.iter(|| async_n_window_sma(5, black_box(&_vec))));
}

criterion_group!(benches, min_benchmark, max_benchmark, change_benchmark, n_window_sma_benchmark);
criterion_main!(benches);
