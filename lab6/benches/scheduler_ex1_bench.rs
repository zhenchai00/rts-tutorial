use criterion::{
    black_box, criterion_group, criterion_main, Criterion
};

use lab6::{scheduler_ex1, scheduler_ex1_crossbeam};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("scheduler_ex1", |b| b.iter(|| black_box(scheduler_ex1())));
    c.bench_function("scheduler_ex1_crossbeam", |b| b.iter(|| black_box(scheduler_ex1_crossbeam())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
