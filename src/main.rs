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
mod irc;

const ADDR: &str = "localhost:3030";

// struct server {
//     pub channels: HashMap::<String, irc::channel::Channel>,
//     pub domain: &String,
// }

fn main() {
    // bind to address ADDR 
    let listener = match TcpListener::bind(ADDR) {
        Ok(listener) => listener,
        Err(e) => {panic!("Error binding to TCP socket: {}", e);}
    };

    let mut channel = irc::channel::Channel {
        users: HashMap::<String, &irc::User>::new(),
        priv_users: HashMap::<String, &irc::User>::new(),
        flag: irc::channel::Flags::new(),
        name: String::from("channel"),
        topic: String::from(""),
        key: String::from("passwd")
    };

    let user = irc::User {name: String::from("name")};
    channel.add_user(&user);
    

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

    // search for first null character in array
    let len = buffer.iter().position(|&p| p == 0).unwrap();
    
    // convert the input to uppercase
    // slice index only to the length of the string
    let client_in= String::from_utf8_lossy(&buffer[0..len]);
    let mut response: String = String::from("");

    //let final_response = format!("{} {} {}", response, channel_message, users);
    let host = String::from("localhost");
    let username = String::from("jacob");
    let message = String::from("Welcome to IRCrust");
    let channel = String::from("channel");
    let users = String::from("jacob man");

    // parse the client input text
    println!("client in: {}", client_in);
    if client_in.contains("JOIN") {
        response.push_str(":jacob!jacob@localhost JOIN #channel\n");
        response.push_str(format!(":{} {} {} = #{} :{} \n", 
                                  &localhost, irc::Response::RplUsersstart, &channel, &users));
        response.push_str(":localhost 366 jacob #channel :End of NAMES list\n"); 
    } else if client_in.contains("CAP") { 
        // welcome message
        response = irc::commandf::server_client(&host, irc::Response::RplWelcome, &username, &message);
    } else {
        response = "".to_string();
    }

    
    println!("{}", response);
    
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

