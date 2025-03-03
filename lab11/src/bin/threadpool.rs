use peak_alloc::PeakAlloc;
use threadpool::ThreadPool;
use std::sync::mpsc;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn compare_memory_usage(thread_count: usize) {
    // Measure memory usage before starting the threads
    let initial_mem = PEAK_ALLOC.current_usage_as_kb();
    println!("Starting with {} threads, initial memory usage: {} KB", thread_count, initial_mem);

    // Create a thread pool with the specified number of threads
    let pool = ThreadPool::new(thread_count);

    // Create a channel to receive results
    let (tx, rx) = mpsc::channel();

    // Spawn threads in the pool to calculate Fibonacci(20)
    for _ in 0..thread_count {
        let tx = tx.clone();
        pool.execute(move || {
            let result = fibonacci(20);
            tx.send(result).unwrap();
        });
    }

    // Collect all results (not used in this case, but just for completeness)
    let mut results = Vec::new();
    for _ in 0..thread_count {
        results.push(rx.recv().unwrap());
    }

    // Measure memory usage after thread execution
    let current_mem = PEAK_ALLOC.current_usage_as_kb();
    println!("After {} threads, current memory usage: {} KB", thread_count, current_mem);

    // Peak memory usage
    let peak_mem = PEAK_ALLOC.peak_usage_as_kb();
    println!("Peak memory usage with {} threads: {} KB", thread_count, peak_mem);

    println!("\n {} threads, used total memory of {} \n", thread_count, current_mem - initial_mem);
}

fn main() {
    // Compare memory usage with different thread counts (5, 10, 15, and 20)
    for thread_count in [5, 10, 15, 20].iter() {
        compare_memory_usage(*thread_count);
    }
}
