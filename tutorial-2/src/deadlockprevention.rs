use std::sync::Mutex;

pub fn deadlockprevention() {
    let mutex1 = Mutex::new(1);
    let mutex2 = Mutex::new(2);

    let result = mutex1.lock().and_then(|val1| {
        mutex2.lock().map(|val2| *val1 + *val2)
    });

    match result {
        Ok(sum) => println!("Sum: {}", sum),
        Err(_) => println!("Deadlock detected"),
    }
}