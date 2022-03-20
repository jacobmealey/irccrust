use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("localhost:3030").unwrap();

    for stream in listener.incoming() {
        loop {
            match stream {
                Ok(ref stream) => {
                    let num = handle_connection(&stream);
                    println!("Wrote {} bytes", num);
                    if num == 0 {
                        break;
                    }
                }
                Err(ref _e) => {println!("Error in stream :(");}
            }
        }
        
    }
    
}

// Handle connection takes a TcpStream and returns the
// amount of bytes written to the stream. It reads a 
// 1024 bytes at a time from the TcpStream
fn handle_connection(mut stream: &TcpStream) -> usize {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    println!("Request from: {}", String::from_utf8_lossy(&buffer[..]));

    let response = String::from_utf8_lossy(&buffer[..]).to_uppercase();
    
    let num = response.len();

    match stream.write(response.as_bytes()) {
        Ok(_) => {
            stream.flush().unwrap();
            return num;
        }
        Err(_e) => {
            //stream.shutdown(Shutdown::Both).expect("Shutdown call failed");
            println!("Connection Closed");
            return 0;
        }
    }
    
}
