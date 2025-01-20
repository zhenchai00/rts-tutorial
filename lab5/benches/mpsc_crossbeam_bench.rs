use criterion::{
    criterion_group,
    criterion_main,
    Criterion
};
use lab5::{mpsc_test,crossbeam_mpsc_test};

pub fn mpsc_benchmark (c: &mut Criterion) {
    c.bench_function("mpsc", |b| b.iter(|| mpsc_test()));
}

pub fn crossbeam_mpsc_benchmark (c: &mut Criterion) {
    c.bench_function("crossbeam_mpsc", |b| b.iter(|| crossbeam_mpsc_test()));
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
//     targets = mpsc_benchmark, crossbeam_mpsc_benchmark
// }

criterion_group!(benches, mpsc_benchmark, crossbeam_mpsc_benchmark);
criterion_main!(benches);