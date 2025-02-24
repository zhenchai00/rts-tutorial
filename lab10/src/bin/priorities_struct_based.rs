use std::collections::BinaryHeap;
use std::cmp::Ordering;
use tokio::task;

#[derive(Eq, PartialEq)]
struct PrioritizedTask {
    priority: u8,
    name: &'static str,
}

// custom ordering so the hightest priority is processed first
impl Ord for PrioritizedTask {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for PrioritizedTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[tokio::main]
async fn main() {
    let mut task_queue = BinaryHeap::new();

    // add tasks with different priorities
    task_queue.push(PrioritizedTask { priority: 1, name: "Low Priority Task"});
    task_queue.push(PrioritizedTask { priority: 3, name: "High Priority Task"});
    task_queue.push(PrioritizedTask { priority: 2, name: "Medium Priority Task"});

    // process tasks in order of priority
    while let Some(task) = task_queue.pop() {
        task::spawn(async move {
            println!("Executing: {}", task.name);
        }).await.unwrap();
    }
}
