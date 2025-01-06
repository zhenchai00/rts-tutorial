mod thread_pool_sample;
mod scheduled_thread_pool_sample;
mod stopwatch;
mod lab3b_example;
mod lab3b_stock;

// use stopwatch::stopwatch;
// use thread_pool_sample::thread_pool_fn;
// use scheduled_thread_pool_sample::schedule_thread_pool_fn;
// use lab3b_example::exercise4;
use lab3b_stock::stock_example;

fn main() {
    // thread_pool_fn();
    // schedule_thread_pool_fn();
    // stopwatch();
    // exercise4();
    stock_example();
    // loop{};
}
