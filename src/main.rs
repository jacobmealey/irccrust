// IRCrust: main.rs
// Author: Jacob Mealey <jacob.mealey@maine.edu>
// Main code for IRCrust (pronounced I-R-crust)
// a "simple" IRC server written in rust because
// I don't like being happy?

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // bind to localhost:3030 how can we make the address and port
    // constants or "predefine macros" like in C
    let listener = match TcpListener::bind("localhost:3030") {
        Ok(listener) => listener,
        Err(e) => {panic!("Error binding to TCP socket: {}", e);}
    };

    // currently we are only listening to a single connection at 
    // a time, we /should/ open a new thread everytime we get a 
    // connection
    for stream in listener.incoming() {
        loop {
            match stream {
                Ok(ref stream) => {
                    let num = handle_connection(&stream);
                    println!("Wrote {} bytes", num);
                    // if zero, no bytes written connection is closed
                    // (do we know that for sure?)
                    // break out of 'loop' and scan for new connections
                    if num == 0 {
                        break;
                    }
                }
                // Not sure what type of errors there could be,
                // so we'll find out if things break
                Err(ref _e) => {println!("Error in stream :(");}
            }
        }
        
    }
    
}

// Handle connection takes a TcpStream and returns the
// amount of bytes written to the stream. It reads a 
// 1024 bytes at a time from the TcpStream
//
// Ideally it should return a Result<> and have the err
// handled properly
fn handle_connection(mut stream: &TcpStream) -> usize {
    // set buffer to size of 1024 and read from TcpStream 
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    println!("Request from: {}", String::from_utf8_lossy(&buffer[..]));

    // convert the input to uppercase
    let response = String::from_utf8_lossy(&buffer[..]).to_uppercase();
    
    // set num (the return value)
    let num = response.len();

    // need to match the wrte() to see if the error connection is still
    // alive, not sure why we don't need to do it on the read (we probs should)
    match stream.write(response.as_bytes()) {
        Ok(_) => {
            stream.flush().unwrap();
            return num;
        }
        // We should probably be checking what the error is and handling 
        // instead of assuming the connection is dead.
        Err(_e) => {
            println!("Connection Closed");
            return 0;
        }
    }
    
}
