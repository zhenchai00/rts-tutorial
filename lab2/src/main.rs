mod benchmarking;
mod channel_example;

use benchmarking::{mutex_test, rwlock_test};
use channel_example::{channel_ex, crossbeam_channel_ex};
use bma_benchmark::benchmark;
use std::hint::black_box;

fn main() {
    // for the use of the functions in the benchmarking module without shorthand
    // benchmarking::mutex_test();
    // benchmarking::rwlock_test();

    mutex_test();
    rwlock_test();

    benchmark!(1000000,{
        bench_vec();
    });
    benchmark!(1000000,{
        bench_vec2();
    });
    benchmark!(1000000,{
        bench_vec3();
    });

    channel_ex();
    crossbeam_channel_ex();
}

struct Student {
    name: String,
    id: i32
}

pub fn bench_vec() {
    let s = Student{
        name: "John".to_string(),
        id: 101
    };
    let mut vec = Vec::new();
    vec.push(s);
}

pub fn bench_vec2() {
    let mut vec = Vec::new();
    vec.push(Student{
        name: "John".to_string(),
        id: 100
    });
}

pub fn bench_vec3() {
    let _vec = vec![
        Student{
            name: "John".to_string(),
            id: 100
        }
    ];
}
