// IRCrust: main.rs
// Author: Jacob Mealey <jacob.mealey@maine.edu>
// Main code for IRCrust (pronounced I-R-crust)
// a "simple" IRC server written in rust because
// I don't like being happy?

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
mod irc;

const ADDR: &str = "localhost:3030";

#[derive(Clone)]
struct Server<'a> {
    pub channels: Box<HashMap::<String, irc::channel::Channel<'a>>>,
    pub users: Box<HashMap::<String, irc::User>>,
    pub domain: String,
}

impl Server<'_> {
    pub fn new<'a>(host: String) -> Server<'a> {
        return Server {
            channels: Box::new(HashMap::<String, irc::channel::Channel<'a>>::new()),
            users: Box::new(HashMap::<String, irc::User>::new()),
            domain: host,
        }
    }
}

fn main() {
    // bind to address ADDR 
    let listener = match TcpListener::bind(ADDR) {
        Ok(listener) => listener,
        Err(e) => {panic!("Error binding to TCP socket: {}", e);}
    };


    let server_lock = Arc::new(Mutex::new(Server::new(String::from("localhost")))); 
    let (tx, rx): (Sender<Server>, Receiver<Server>) = mpsc::channel();

    let mut threads = vec![];
    

    // currently we are only listening to a single connection at 
    // a time, we /should/ open a new thread everytime we get a 
    // connection
    for stream in listener.incoming() {
        // create a thread for each no connection. I don't really
        // konw how to handle this properly but it doesn't seem
        // terribel?
        let server = Arc::clone(&server_lock);
        let thread_tx = tx.clone();

        let thread = thread::spawn(move || {
            // we loop to ensure that stream stays in scope and 
            // is not dropped (thus killing the connection)
            let mut user: irc::User = irc::User {name: String::from("")};
            loop {
                let mut stream = match stream {
                    Ok(ref stream) => stream,
                    Err(_e) => {panic!("Error in stream :(");}
                };
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();


                let server = server.lock().unwrap();
                let mut new_server = server.clone();
                // search for first null character in array
                let len = buffer.iter().position(|&p| p == 0).unwrap();

                // slice index only to the length of the string
                let client_in = String::from_utf8_lossy(&buffer[0..len]);
                println!("client in: {}", client_in);
                // decode client in 
                let msgs = irc::commandf::message_decode(client_in.to_string());

                let mut response: String = String::from("");
                thread_tx.send(new_server);

                println!("Response: \n{}", response);

                // need to match the wrte() to see if the error connection is still
                // alive, not sure why we don't need to do it on the read (we probs should)
                match stream.write(response.as_bytes()) {
                    Ok(_) => {
                        stream.flush().unwrap();
                        return len;
                    }
                    // We should probably be checking what the error is and handling 
                    // instead of assuming the connection is dead.
                    Err(_e) => {
                        println!("Connection Closed");
                        return 0;
                    }
                }
           }

        });

        threads.push(thread);
    }
        
    loop {
        for msg in rx.recv() {
            todo!();
        }
    }
    
}

// Handle connection takes a TcpStream and returns the
// amount of bytes written to the stream. It reads a 
// 1024 bytes at a time from the TcpStream
//
// Ideally it should return a Result<> and have the err
// handled properly
fn _handle_connection(mut stream: &TcpStream, tx: &Sender<Server>, lock: &Arc<Mutex<Server>>, user: &mut irc::User) -> usize {
    // set buffer to size of 1024 and read from TcpStream 
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();


    //let server = Arc::clone(&lock);
    let server = lock.lock().unwrap();

    // search for first null character in array
    let len = buffer.iter().position(|&p| p == 0).unwrap();

    // slice index only to the length of the string
    let client_in = String::from_utf8_lossy(&buffer[0..len]);
    println!("client in: {}", client_in);
    // decode client in 
    let msgs = irc::commandf::message_decode(client_in.to_string());
     
    let mut response: String = String::from("");

    
    println!("Response: \n{}", response);
    
    // need to match the wrte() to see if the error connection is still
    // alive, not sure why we don't need to do it on the read (we probs should)
    match stream.write(response.as_bytes()) {
        Ok(_) => {
            stream.flush().unwrap();
            return len;
        }
        // We should probably be checking what the error is and handling 
        // instead of assuming the connection is dead.
        Err(_e) => {
            println!("Connection Closed");
            return 0;
        }
    }
    
}

