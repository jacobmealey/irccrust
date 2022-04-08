// IRCrust: main.rs
// Author: Jacob Mealey <jacob.mealey@maine.edu>
// Main code for IRCrust (pronounced I-R-crust)
// a "simple" IRC server written in rust because
// I don't like being happy?

use std::io::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use tokio::{io::{AsyncWriteExt, BufReader, AsyncBufReadExt}, net::TcpListener, sync::broadcast};
mod irc;

const ADDR: &str = "localhost:3030";

#[derive(Clone)]
struct Server {
    pub channels: Box<HashMap::<String, irc::channel::Channel>>,
    pub users: Box<HashMap::<String, SocketAddr>>,
    pub domain: String,
}

impl Server {
    pub fn new(host: String) -> Server {
        return Server {
            channels: Box::new(HashMap::<String, irc::channel::Channel>::new()),
            users: Box::new(HashMap::<String, SocketAddr>::new()),
            domain: host,
        }
    }
}

#[tokio::main]
async fn main() {
    // bind to address ADDR 
    let listener = match TcpListener::bind(ADDR).await {
        Ok(listener) => listener,
        Err(e) => {panic!("Error binding to TCP socket: {}", e);}
    };


    let server_lock = Arc::new(Mutex::new(Server::new(String::from("localhost")))); 
    
    let (tx, _rx) = broadcast::channel(10);

    // This loops creates a new thread and keeps them alive as long as their is a 
    // connection everytime a new connection is made it spawns a new thread
    loop {
        // get socket and adress from the listener
        let (mut socket, addr) = listener.accept().await.unwrap();

        // create clones of channels 
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        let mut line = String::new();

        let server = Arc::clone(&server_lock);
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut user: irc::User = irc::User{name: "".to_string()};
            loop {
                tokio::select! {

                    result = reader.read_line(&mut line) => {
                        let mut msg_type: irc::commandf::IRCMessageType = irc::commandf::IRCMessageType::UNKNOWN;
                        let mut response = String::from("");
                        let messages = irc::commandf::message_decode(line.clone());

                        {
                            let mut server = server.lock().unwrap();
                            if result.unwrap() == 0 {
                                break;
                            }


                            for msg in messages {
                                msg_type = msg.msg_type;
                                match msg_type {
                                    irc::commandf::IRCMessageType::NICK => {
                                        user.name= msg.component[0].clone();
                                        response = irc::commandf::server_client(&server.domain, irc::Response::RplWelcome, &user.name, &"Goodday!".to_string());
                                    }
                                    irc::commandf::IRCMessageType::JOIN => {
                                        let channel = server.channels.entry(msg.component[0].clone()).or_insert(irc::channel::Channel::new("channel"));
                                        channel.users.insert(user.name.clone(), addr);
                                        response = irc::commandf::client_join(&user.name, &msg.component[0], &server.domain.clone());
                                    }
                                    irc::commandf::IRCMessageType::PRIVMSG  => {
                                        response = line.clone();
                                        println!("{}", line);
                                    }                            
                                    _ => {
                                        response = "".to_string();
                                        println!("{}", line);
                                    }
                                }
                            }
                        }

                        tx.send((msg_type, response.clone(), addr)).unwrap();
                    } result = rx.recv() => {
                        // this part should NEVER mutate the server -- this is for updating 
                        // updating all clients with current state of this biddy
                        let server_ : Server; 
                        {
                            server_ = server.lock().unwrap().clone();
                        }
                        let (mtype, msg, other_addr) = result.unwrap();
                        let messages = irc::commandf::message_decode(msg.clone());
                        match mtype {
                            irc::commandf::IRCMessageType::NICK => {
                                if addr == other_addr {
                                    writer.write_all(&msg.as_bytes()).await.unwrap();
                                } 
                            }
                            irc::commandf::IRCMessageType::JOIN => {
                                let message = &messages[0];
                                let channel = &message.component[1];
                                println!("{}", channel);
                                if addr == other_addr {
                                    writer.write_all(&msg.as_bytes()).await.unwrap();
                                } 
                            }
                            irc::commandf::IRCMessageType::PRIVMSG => {
                                println!("{}", msg);
                            }
                            _ => {}
                        }
                    }
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
// fn _handle_connection(mut stream: &TcpStream, tx: &Sender<Server>, lock: &Arc<Mutex<Server>>, user: &mut irc::User) -> usize {
//     // set buffer to size of 1024 and read from TcpStream 
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();
// 
// 
//     //let server = Arc::clone(&lock);
//     let server = lock.lock().unwrap();
// 
//     // search for first null character in array
//     let len = buffer.iter().position(|&p| p == 0).unwrap();
// 
//     // slice index only to the length of the string
//     let client_in = String::from_utf8_lossy(&buffer[0..len]);
//     println!("client in: {}", client_in);
//     // decode client in 
//     let msgs = irc::commandf::message_decode(client_in.to_string());
//      
//     let mut response: String = String::from("");
// 
//     
//     println!("Response: \n{}", response);
//     
//     // need to match the wrte() to see if the error connection is still
//     // alive, not sure why we don't need to do it on the read (we probs should)
//     match stream.write(response.as_bytes()) {
//         Ok(_) => {
//             stream.flush().unwrap();
//             return len;
//         }
//         // We should probably be checking what the error is and handling 
//         // instead of assuming the connection is dead.
//         Err(_e) => {
//             println!("Connection Closed");
//             return 0;
//         }
//     }
//     
// }
// 
