use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("localhost:3030").unwrap();

    for stream in listener.incoming() {
        let num = handle_connection(stream.unwrap());
        println!("Write {} bytes", num);
    }
}

fn handle_connection(mut stream: TcpStream) -> usize {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let response = String::from_utf8_lossy(&buffer[..]).to_uppercase();
    
    let num = response.len();

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    
    return num;
}
