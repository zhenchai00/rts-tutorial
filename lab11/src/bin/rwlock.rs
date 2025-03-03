use std::sync::RwLock;

use peak_alloc::PeakAlloc;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

fn main() {
    let initial_mem = PEAK_ALLOC.current_usage_as_kb();
    println!("Initial memory usage: {} KB of RAM.", initial_mem);

    let rwlock = RwLock::new(5);
    {
        let mut num = rwlock.write().unwrap();
        *num = 6;
    }
    println!("num = {:?}", rwlock);

    let current_mem = PEAK_ALLOC.current_usage_as_kb();
    println!("This program currently uses {} KB of RAM.", current_mem);
    let peak_mem = PEAK_ALLOC.peak_usage_as_kb();
    println!("The max amount that was used {}", peak_mem);
}