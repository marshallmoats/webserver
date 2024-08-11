use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use std::{thread, time};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();


        for _ in 0..100 {
            stream.write_all(b"yo!").unwrap();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");

}