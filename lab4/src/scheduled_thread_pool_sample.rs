use std::time::Duration;
use scheduled_thread_pool::ScheduledThreadPool;

pub fn schedule_thread_pool_fn () {
    let pool = ScheduledThreadPool::new(2); 
    pool.execute_at_fixed_rate(
        Duration::from_secs(5),     // initial delay
        Duration::from_secs(2),             // period 
        || {
        println!("hello from scheduled thread pool");
    });
}