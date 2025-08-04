use std::time::Duration;

use trpl::{self, join, sleep};

fn main() {
    trpl::run(async {
        let handle1 = trpl::spawn_task(async {
            for i in 0..10 {
                println!("Thread 1 counting the number: {}", i);
                sleep(Duration::from_millis(500)).await;
            }
        });
        let handle2 = trpl::spawn_task(async {
            for i in 0..8 {
                println!("Thread 2 counting the number {}", i);
                sleep(Duration::from_millis(500)).await;
            }
        });
        for i in 0..5 {
            println!("Thread main counting the number: {}", i);
            trpl::sleep(Duration::from_millis(500)).await;
        }
        let (result1, result2) = join(handle1, handle2).await;

        if let Err(e) = result1 {
            eprintln!("Task1 failed: {:?}", e);
        }
        if let Err(e) = result2 {
            eprintln!("Task2 failed: {:?}", e)
        }
    });
}
