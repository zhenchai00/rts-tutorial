use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::sync::mpsc::{channel, Receiver, Sender};
use tokio::task;

struct BakedItem {
    name: String,
}

#[derive(Eq, PartialEq)]
struct PrioritizedTask {
    priority: u8,
    name: &'static str,
}
impl PrioritizedTask {
    fn bake(&self, sender: Sender<BakedItem>) {
        println!("{} baker is using the oven, sending 12 {}s to the Worker ",self.name, self.name);
        for _ in 1..12 {
            sender.send(BakedItem { name: self.name.to_string() }).unwrap();
        }
    }

    fn work(&self, receiver: Receiver<BakedItem>) {
        loop {
            let item = receiver.recv().unwrap();
            println!("Worker: received a {}, adding to the shelf", item.name);
        }
    }
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
    task_queue.push(PrioritizedTask { priority: 1, name: "Croissant"});
    task_queue.push(PrioritizedTask { priority: 4, name: "Souffle"});
    task_queue.push(PrioritizedTask { priority: 3, name: "Cake"});
    task_queue.push(PrioritizedTask { priority: 2, name: "Worker"});

    // process tasks in order of priority
    let (tx, rx) = channel();
    while let Some(task) = task_queue.pop() {
        let tx = tx.clone();
        task::spawn(async move {
            task.bake(tx);
        }).await.unwrap();
    }
    let worker = PrioritizedTask { priority: 1, name: "Worker"};
    worker.work(rx);
}

async fn worker(receiver: Receiver<BakedItem>) {
    loop {
        let item = receiver.recv().unwrap();
        println!("Worker: received a {}, adding to the shelf", item.name);
    }
}
