use peak_alloc::PeakAlloc;

struct Item (i32);

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

fn main() {
    let initial_mem = PEAK_ALLOC.current_usage_as_kb();
    println!("Initial memory usage: {} KB of RAM.", initial_mem);

    let mut items = Vec::new();
    items.push(Item(1));
    let current_mem = PEAK_ALLOC.current_usage_as_kb();
    println!("This program currently uses {} KB of RAM.", current_mem);
    items.push(Item(2));
    let current_mem = PEAK_ALLOC.current_usage_as_kb();
    println!("This program currently uses {} KB of RAM.", current_mem);
    items.push(Item(3));
    let current_mem = PEAK_ALLOC.current_usage_as_kb();
    println!("This program currently uses {} KB of RAM.", current_mem);
    items.push(Item(4));
    let current_mem = PEAK_ALLOC.current_usage_as_kb();
    println!("This program currently uses {} KB of RAM.", current_mem);
    items.push(Item(5));
    let current_mem = PEAK_ALLOC.current_usage_as_kb();
    println!("This program currently uses {} KB of RAM.", current_mem);


    for item in &items {
        println!("Item value: {}", item.0);
    }

    let current_mem = PEAK_ALLOC.current_usage_as_kb();
    println!("This program currently uses {} KB of RAM.", current_mem);
    let peak_mem = PEAK_ALLOC.peak_usage_as_kb();
    println!("The max amount that was used {}", peak_mem);
}