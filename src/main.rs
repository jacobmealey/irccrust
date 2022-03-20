// IRCrust: main.rs
// Author: Jacob Mealey <jacob.mealey@maine.edu>
// Main code for IRCrust (pronounced I-R-crust)
// a "simple" IRC server written in rust because
// I don't like being happy?

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use crate::irc::channel;
mod irc;

const ADDR: &str = "localhost:3030";

fn main() {
    // bind to address ADDR 
    let listener = match TcpListener::bind(ADDR) {
        Ok(listener) => listener,
        Err(e) => {panic!("Error binding to TCP socket: {}", e);}
    };

    let channel = irc::channel::Channel {
        users: Vec::<irc::User>::new(),
        priv_users: Vec::<irc::User>::new(),
        flag: Vec::<irc::channel::Flags>::new(),
        name: String::from("channel"),
        topic: String::from(""),
        key: String::from("passwd")
    };
    

    // currently we are only listening to a single connection at 
    // a time, we /should/ open a new thread everytime we get a 
    // connection
    for stream in listener.incoming() {
        // create a thread for each no connection. I don't really
        // konw how to handle this properly but it doesn't seem
        // terribel?
        let thread = thread::spawn(|| {
            // we loop to ensure that stream stays in scope and 
            // is not dropped (thus killing the connection)
            loop {
                let stream = match stream {
                    Ok(ref stream) => stream,
                    Err(_e) => {panic!("Error in stream :(");}
                };
                let num = handle_connection(&stream);
                println!("Wrote {} bytes", num);
                // if zero, no bytes written connection is closed
                // (do we know that for sure?)
                // break out of 'loop' and scan for new connections
                if num == 0 {
                    break;
                }
            }

        });
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
