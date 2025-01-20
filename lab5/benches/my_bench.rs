use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};
use lab5::{fibonacci, fibonacci_iteration};

pub fn fib_benchmark(c: &mut Criterion) {
    c.bench_function("fib 30", |b| b.iter(|| fibonacci(black_box(30))));
}

pub fn fib_iter_benchmark(c: &mut Criterion) {
    c.bench_function("fib_iter 30", |b| b.iter(|| fibonacci_iteration(black_box(30))));
}

// Configure the criterion 
// fn configure_criterion() -> Criterion {
//     Criterion::default()
//         .sample_size(10)
//         .measurement_time(std::time::Duration::from_secs(20))
// }

// criterion_group!{
//     name = benches;
//     config = configure_criterion();
//     targets = fib_benchmark, fib_iter_benchmark
// }
criterion_group!(benches, fib_benchmark, fib_iter_benchmark);
criterion_main!(benches);