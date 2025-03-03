use std::{sync::{mpsc::channel, Arc, Mutex}, thread};

use crossbeam::channel::unbounded;
use peak_alloc::PeakAlloc;
use threadpool::ThreadPool;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

#[derive(Debug)]
struct Job {
    job_id: u32,
    description: String,
}

pub fn scheduler_ex1() {
    let initial_mem = PEAK_ALLOC.current_usage_as_kb();
    println!("Initial memory usage: {} KB of RAM.", initial_mem);

    let jobs = vec![
        Job {
            job_id: 1,
            description: "Job 1".to_string(),
        },
        Job {
            job_id: 2,
            description: "Job 2".to_string(),
        },
        Job {
            job_id: 3,
            description: "Job 3".to_string(),
        },
        Job {
            job_id: 4,
            description: "Job 4".to_string(),
        },
        Job {
            job_id: 5,
            description: "Job 5".to_string(),
        },
        Job {
            job_id: 6,
            description: "Job 6".to_string(),
        },
        Job {
            job_id: 7,
            description: "Job 7".to_string(),
        },
        Job {
            job_id: 8,
            description: "Job 8".to_string(),
        },
        Job {
            job_id: 9,
            description: "Job 9".to_string(),
        },
        Job {
            job_id: 10,
            description: "Job 10".to_string(),
        },
        Job {
            job_id: 11,
            description: "Job 11".to_string(),
        },
        Job {
            job_id: 12,
            description: "Job 12".to_string(),
        },
        Job {
            job_id: 13,
            description: "Job 13".to_string(),
        },
        Job {
            job_id: 14,
            description: "Job 14".to_string(),
        },
        Job {
            job_id: 15,
            description: "Job 15".to_string(),
        },
        Job {
            job_id: 16,
            description: "Job 16".to_string(),
        },
        Job {
            job_id: 17,
            description: "Job 17".to_string(),
        },
        Job {
            job_id: 18,
            description: "Job 18".to_string(),
        },
        Job {
            job_id: 19,
            description: "Job 19".to_string(),
        },
        Job {
            job_id: 20,
            description: "Job 20".to_string(),
        },
        Job {
            job_id: 21,
            description: "Job 21".to_string(),
        },
        Job {
            job_id: 22,
            description: "Job 22".to_string(),
        },
        Job {
            job_id: 23,
            description: "Job 23".to_string(),
        },
        Job {
            job_id: 24,
            description: "Job 24".to_string(),
        },
        Job {
            job_id: 25,
            description: "Job 25".to_string(),
        },
        Job {
            job_id: 26,
            description: "Job 26".to_string(),
        },
        Job {
            job_id: 27,
            description: "Job 27".to_string(),
        },
        Job {
            job_id: 28,
            description: "Job 28".to_string(),
        },
        Job {
            job_id: 29,
            description: "Job 29".to_string(),
        },
        Job {
            job_id: 30,
            description: "Job 30".to_string(),
        },
    ];

    let job_queue = Arc::new(Mutex::new(jobs));
    let (tx, rx) = channel();
    let mut handles = vec![];

    for i in 0..4 {
        let job_queue = Arc::clone(&job_queue);
        let sender = tx.clone();


        let handle = thread::spawn(move || {
            let job = job_queue.lock().unwrap().pop();
            match job {
                Some(job) => {
                    println!("Thread {} processing job {:?}", i, job);
                    thread::sleep(std::time::Duration::from_millis(200));
                    let msg = format!("Thread {} completed Job {:?}", i, job);
                    sender.send(msg).unwrap();
                }
                None => {
                    println!("Thread {} didn't get any job", i);
                }
            }
        });

        handles.push(handle);

        let current_mem = PEAK_ALLOC.current_usage_as_kb();
        println!("Thread {} is currently uses {} KB of RAM.",i, current_mem);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let msg = rx.recv().unwrap();
    println!("{}", msg);

    println!("All job completed!");
    let current_mem = PEAK_ALLOC.current_usage_as_kb();
    println!("This program currently uses {} KB of RAM.", current_mem);
    let peak_mem = PEAK_ALLOC.peak_usage_as_kb();
    println!("The max amount that was used {}", peak_mem);

    println!("Total memory used: {} KB", current_mem - initial_mem);
}

pub fn scheduler_ex1_crossbeam() {
    let initial_mem = PEAK_ALLOC.current_usage_as_kb();
    println!("Initial memory usage: {} KB of RAM.", initial_mem);

    let jobs = (1..=30).map(|i| Job {
        job_id: i,
        description: format!("Job {}", i),
    }).collect::<Vec<_>>();


    let job_queue = Arc::new(Mutex::new(jobs));
    let (tx, rx) = unbounded();
    let pool = ThreadPool::new(4);

    for i in 0..4 {
        let job_queue = Arc::clone(&job_queue);
        let sender = tx.clone();

        pool.execute(move || {
            let job = job_queue.lock().unwrap().pop();
            match job {
                Some(job) => {
                    println!("Thread {} processing job {:?}", i, job);
                    thread::sleep(std::time::Duration::from_millis(200));
                    let msg = format!("Thread {} completed Job {:?}", i, job);
                    sender.send(msg).unwrap();
                }
                None => {
                    println!("Thread {} didn't get any job", i);
                }
            }
        });
        let current_mem = PEAK_ALLOC.current_usage_as_kb();
        println!("Thread {} is currently uses {} KB of RAM.",i, current_mem);
    }

    let msg = rx.recv().unwrap();
    println!("{}", msg);

    println!("All job completed!");
    let current_mem = PEAK_ALLOC.current_usage_as_kb();
    println!("This program currently uses {} KB of RAM.", current_mem);
    let peak_mem = PEAK_ALLOC.peak_usage_as_kb();
    println!("The max amount that was used {}", peak_mem);

    println!("Total memory used: {} KB", current_mem - initial_mem);
}

fn main() {
    scheduler_ex1();
    PEAK_ALLOC.reset_peak_usage();
    scheduler_ex1_crossbeam();
}