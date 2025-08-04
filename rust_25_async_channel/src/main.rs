use std::time::Duration;

use trpl::{self, sleep};

fn main() {
    println!("Running example1: ");
    example1();
    println!("Running example2:");
    example2();
}

fn example1() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let greet_string = String::from("Ciallo~");
        tx.send(greet_string).unwrap();

        let received = rx.recv().await.unwrap();
        println!("received: {}", received);
    });
}

fn example2() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone();

        let send = async move {
            for i in 0..5 {
                tx.send(i).unwrap();
                sleep(Duration::from_millis(300)).await;
            }
        };

        let receive = async {
            while let Some(message) = rx.recv().await {
                println!("Got value: {}", message);
            }
        };

        let send2 = async move {
            for i in 0..15 {
                tx1.send(i).unwrap();
                sleep(Duration::from_millis(100)).await;
            }
        };

        trpl::join3(send, receive, send2).await;
    });
}
