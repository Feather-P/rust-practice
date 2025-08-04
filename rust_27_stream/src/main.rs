use std::time::Duration;
use std::pin::pin;
use trpl::{self, channel, stream_from_iter, ReceiverStream, Stream, StreamExt};

fn main() {
    example2();
}

fn example1() {
    trpl::run(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|x| x * 2);
        let mut stream = stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The Value is: {}", value);
        }
    });
}

fn example2() {
    trpl::run(async {
        let mut messages = pin!(get_message().timeout(Duration::from_millis(1)));
        while let Some(value) = messages.next().await {
            match value {
                Ok(value) => println!("Message: {}", value),
                Err(e) => eprintln!("Something goes wrong: {:?}", e)
            }
        }
    })
}

fn get_message() -> impl Stream<Item = String> {
    let (tx, rx) = channel();
    let one_to_ten = [1,2,3,4,5,6,7,8,9,10];
    for number in one_to_ten {
        tx.send(number.to_string()).unwrap();
    }
    ReceiverStream::new(rx)
}