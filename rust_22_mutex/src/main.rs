use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    println!("EXAMPLE1:");
    single_thread_mutex_example();
    println!("EXAMPLE2:");
    multi_thread_mutex_example();
}

fn single_thread_mutex_example() {
    let m = Mutex::new(5);
    let mut n = m.lock().unwrap();
    *n = 6;
    println!("{:?}", *n)
}

fn multi_thread_mutex_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut threads = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let thread = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("{}", *counter.lock().unwrap())
}
