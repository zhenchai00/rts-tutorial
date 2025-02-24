use std::sync::mpsc::channel;
use peak_alloc::PeakAlloc;

#[derive(Debug)]
enum Type {
    Cake,
    Bun,
    Spoilt,
}

#[derive(Debug)]
struct  Item {
    item_type: Type,
}

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

fn main() {
    let (tx, rx) = channel();
    let cake = Type::Cake;
    tx.send(Item {item_type: cake}).unwrap();

    let bun = Type::Bun;
    let current_mem = PEAK_ALLOC.current_usage_as_mb();
    println!("This program currently uses {} MB of RAM.", current_mem);
    tx.send(Item {item_type: bun}).unwrap();

    let mut first_item = rx.recv().unwrap();
    first_item.item_type = Type::Spoilt;

    let current_mem = PEAK_ALLOC.current_usage_as_mb();
    println!("This program currently uses {} MB of RAM.", current_mem);

    match first_item.item_type {
        Type::Cake => {}
        Type::Bun => {}
        Type::Spoilt => {}
    }
    println!("item: {:?}", first_item);

    let current_mem = PEAK_ALLOC.current_usage_as_kb();
    println!("This program currently uses {} KB of RAM.", current_mem);
    let peak_mem = PEAK_ALLOC.peak_usage_as_kb();
    println!("The max amount that was used {}", peak_mem);
}