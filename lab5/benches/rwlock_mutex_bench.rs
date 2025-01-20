use criterion::{
    criterion_group,
    criterion_main,
    Criterion
};
use lab5::{mutex_test, rw_lock_test};

pub fn rw_lock_benchmark (c: &mut Criterion) {
    c.bench_function("rw_lock", |b| b.iter(|| rw_lock_test()));
}

pub fn mutex_benchmark (c: &mut Criterion) {
    c.bench_function("mutex", |b| b.iter(|| mutex_test()));
}

criterion_group!(benches, rw_lock_benchmark, mutex_benchmark);
criterion_main!(benches);