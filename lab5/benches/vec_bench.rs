use criterion::{
    criterion_group,
    criterion_main,
    Criterion
};
use lab5::{vec_macro_test, vec_test};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("vec", |b| b.iter(|| vec_test()));
    c.bench_function("vec_macro", |b| b.iter(|| vec_macro_test()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);