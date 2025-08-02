use std::thread::{self, sleep};
use std::time::Duration;

fn main() {
    example_1();
    example_2();
}

fn example_1() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("Message from the new thread: {}", i);
            sleep(Duration::from_millis(1));
        }
    });

    for i in 0..5 {
        println!("Message from the main thread: {}", i);
        sleep(Duration::from_millis(1));
    }

    if let Err(e) = handle.join() {
        eprintln!("Thread paniced: {:?}", e);
    }
}

fn example_2() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn( move | | {
        println!("{:?}", v);
    });
    // 新线程的闭包取得了所有权（使用move声明而非让rust推断）
    handle.join().unwrap();
}