use peak_alloc::PeakAlloc;
use std::thread;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

fn main() {
    fn fibonacci(n: u64) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }

    let initial_mem = PEAK_ALLOC.current_usage_as_kb();
    println!("Initial memory usage: {} KB of RAM.", initial_mem);

    for _ in 0..20 {
        thread::spawn(move || {
            let result = fibonacci(20);
            println!("Result: {}", result);
        }).join().unwrap();
    }

    let current_mem = PEAK_ALLOC.current_usage_as_kb();
    println!("This program currently uses {} KB of RAM.", current_mem);
    let peak_mem = PEAK_ALLOC.peak_usage_as_kb();
    println!("The max amount that was used {}", peak_mem);

}