pub mod thread_pool;

use std::{
    fs::read_to_string,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream}, thread, time::Duration,
};

fn main() {
    let pool = thread_pool::ThreadPool::new(4);
    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(| | {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_head = http_request.get(0).unwrap();

    let (status_line, file_name) = match &request_head[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")

    };
       

        let contents = read_to_string(file_name).unwrap();
        let contents_len = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {contents_len}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();

        // println!("http request: {:?}", http_request);
    
}
