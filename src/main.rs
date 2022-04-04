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
mod irc;

const ADDR: &str = "localhost:3030";

struct Server<'a> {
    pub channels: HashMap::<String, irc::channel::Channel<'a>>,
    pub users: HashMap::<String, irc::User>,
    pub domain: String,
}

impl Server<'_> {
    pub fn new<'a>(host: String) -> Server<'a> {
        return Server {
            channels: HashMap::<String, irc::channel::Channel<'a>>::new(),
            users: HashMap::<String, irc::User>::new(),
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

    let mut threads = vec![];
    

    // currently we are only listening to a single connection at 
    // a time, we /should/ open a new thread everytime we get a 
    // connection
    for stream in listener.incoming() {
        // create a thread for each no connection. I don't really
        // konw how to handle this properly but it doesn't seem
        // terribel?
        let server_lock_clone = Arc::clone(&server_lock);
        threads.push(thread::spawn(move || {
            // we loop to ensure that stream stays in scope and 
            // is not dropped (thus killing the connection)
            let mut user: irc::User = irc::User {name: String::from("")};
            loop {
                let stream = match stream {
                    Ok(ref stream) => stream,
                    Err(_e) => {panic!("Error in stream :(");}
                };
                let num = handle_connection(&stream, &server_lock_clone, &mut user);
                println!("Wrote {} bytes", num);
                // if zero, no bytes written connection is closed
                // (do we know that for sure?)
                // break out of 'loop' and scan for new connections
                if num == 0 {
                    break;
                }
            }

        }));
    }

    for thread in threads {
        let _ = thread.join();
    }
    
}

// Handle connection takes a TcpStream and returns the
// amount of bytes written to the stream. It reads a 
// 1024 bytes at a time from the TcpStream
//
// Ideally it should return a Result<> and have the err
// handled properly
fn handle_connection(mut stream: &TcpStream, lock: &Arc<Mutex<Server>>, user: &mut irc::User) -> usize {
    // set buffer to size of 1024 and read from TcpStream 
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();


    //let server = Arc::clone(&lock);
    let mut server = lock.lock().unwrap();

    // search for first null character in array
    let len = buffer.iter().position(|&p| p == 0).unwrap();
    
    // convert the input to uppercase
    // slice index only to the length of the string
    let client_in = String::from_utf8_lossy(&buffer[0..len]);
    println!("client in: {}", client_in);

    let mut response: String = String::from("");
    let msgs = irc::commandf::message_decode(client_in.to_string());

    let host = String::from("localhost");
    let mut username = String::from("jacob");
    let message = String::from("Welcome to IRCrust");
    let mut channel = String::from("channel");
    

    // parse the client input text
    for msg in msgs{
        println!("User: {}", username);
        match msg.msg_type {
            irc::commandf::IRCMessageType::JOIN => {
                channel = msg.component[0].clone();
                let channel_obj = server.channels.entry(channel).or_insert(irc::channel::Channel::new("channel"));
                response = irc::commandf::client_join(&user.name, &channel_obj.name, &host);
            }
            irc::commandf::IRCMessageType::NICK => {
                user.name = msg.component[0].clone();
                println!("NICK messages");
                response = irc::commandf::server_client(&host, irc::Response::RplWelcome, &user.name, &message)
            }
            irc::commandf::IRCMessageType::KILL => {
                // kill thread? 
            }
            _ => {
            }
        }
    }
    
    
    
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

