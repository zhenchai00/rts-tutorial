use tokio::sync::Mutex;
use tokio::sync::mpsc::{self, Sender};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;


#[derive(Debug, Clone)]
enum Product {
    Souffle,
    Cake,
    Bread,
}

#[derive(Debug, PartialEq)]
enum Priority {
    High,
    Medium,
    Low
}

struct Baker {
    name: String,
    product: Product,
    priority: Priority,
}

#[derive(Debug)]
struct Oven {
    is_busy: Mutex<bool>,
}

impl Oven {
    async fn use_oven(&self, baker: &Baker) {
        let mut is_busy = self.is_busy.lock().await;
        if !*is_busy {
            println!("{} is using the oven to bake {:?}", baker.name, baker.product);
            *is_busy = true;
            tokio::time::sleep(Duration::from_secs(2)).await;
            *is_busy = false;
            println!("{} has finished baking {:?}", baker.name, baker.product);
        }
    }
}

async fn baker_task(
    baker: Baker,
    oven: Arc<Oven>,
    sender: Sender<Product>,
) {
    loop {
        if baker.priority == Priority::High {
            oven.use_oven(&baker).await;
            sender.send(baker.product.clone()).await.unwrap();
            break;
        } else { 
            let is_busy = oven.is_busy.lock().await;
            if !*is_busy {
                oven.use_oven(&baker).await;
                sender.send(baker.product.clone()).await.unwrap();
                break;
            }
        }
        sleep(Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() {
    // Create the oven and wrap it in Arc and Mutex for shared access
    let oven = Arc::new(Oven {
        is_busy: Mutex::new(false),
    });

    // Create the communication channels for the workers
    let (tx, mut rx) = mpsc::channel(32);

    // Create bakers with different priorities
    let bakers = vec![
        Baker {
            name: "Souffle Baker".to_string(),
            product: Product::Souffle,
            priority: Priority::High,
        },
        Baker {
            name: "Cake Baker".to_string(),
            product: Product::Cake,
            priority: Priority::Medium,
        },
        Baker {
            name: "Bread Baker".to_string(),
            product: Product::Bread,
            priority: Priority::Low,
        },
    ];

    // Spawn tasks for each baker
    let mut tasks = vec![];
    for baker in bakers {
        let oven_clone = Arc::clone(&oven);
        let sender_clone = tx.clone();
        let task = tokio::spawn(baker_task(baker, oven_clone, sender_clone));
        tasks.push(task);
    }

    // Wait for all tasks to complete
    for task in tasks {
        task.await.unwrap();
    }

    // Collect and print the products sent by the bakers
    while let Some(product) = rx.recv().await {
        println!("Worker received: {:?}", product);
    }
}