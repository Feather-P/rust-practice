use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    example_1();
    println!();
    example_2();
}

fn example_1() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(String::from("Hi~")).unwrap();
    });

    let recv = rx.recv().unwrap();
    println!("Main thread got the transmission: {}", recv)
}

fn example_2() {
    let (tx, rx) = mpsc::channel();
    let tx_cloned = tx.clone();
    let message_list = vec![
        String::from("Hello"),
        String::from("from"),
        String::from("thread"),
        String::from("2")
    ];
    let message_list_1 = vec![
        String::from("Ciallo~"),
        String::from("frooooom"),
        String::from("threaaaaaaad"),
        String::from("2222222")
    ];

    let handle = thread::spawn(move || {
        for message in message_list {
            tx.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let handle1 = thread::spawn(move | | {
        for message in message_list_1{
            tx_cloned.send(message).unwrap(); //使用克隆的channel作为第二个发送端发送消息
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx{
        println!("{}", received);
    }
    handle.join().unwrap();
    handle1.join().unwrap();
    
}