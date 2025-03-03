use peak_alloc::PeakAlloc;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

fn main() {
    // Measure memory before allocation
    let initial_mem = PEAK_ALLOC.current_usage_as_kb();
    println!("Initial memory usage: {} KB", initial_mem);

    // Method 1: Allocate the array [i32; 5]
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    
    // Measure memory usage after allocating the array
    let array_mem = PEAK_ALLOC.current_usage_as_kb();
    println!("Memory usage after allocating the array: {} KB", array_mem);
    let peak_mem_array = PEAK_ALLOC.peak_usage_as_kb();
    println!("Peak memory usage after array allocation: {} KB", peak_mem_array);

    // Reset peak memory usage for the next measurement
    PEAK_ALLOC.reset_peak_usage();

    // Method 2: Allocate the vector Vec<i32>
    let vec = vec![1, 2, 3, 4, 5];
    
    // Measure memory usage after allocating the vector
    let vec_mem = PEAK_ALLOC.current_usage_as_kb();
    println!("Memory usage after allocating the vector: {} KB", vec_mem);
    let peak_mem_vec = PEAK_ALLOC.peak_usage_as_kb();
    println!("Peak memory usage after vector allocation: {} KB", peak_mem_vec);

    // Final memory usage comparison
    println!("Total memory used by array (current): {} KB", array_mem - initial_mem);
    println!("Total memory used by vector (current): {} KB", vec_mem - initial_mem);
}
