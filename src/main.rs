use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use rserv::thread_pool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:13099").unwrap();

    let tp = rserv::thread_pool::ThreadPool::new(16);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        tp.execute(move || {
            handle(stream);
        });
    }
}

fn handle(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("{}", String::from_utf8_lossy(&buffer));

    let content = { "HTTP/1.1 200 OK\r\n\r\n" };
    stream.write(&content.as_bytes()).unwrap();

    for i in 0..50 {
        let content = format!(" {} ", i);

        stream.write(&content.as_bytes()).unwrap();
    }

    stream.flush().unwrap();
}
