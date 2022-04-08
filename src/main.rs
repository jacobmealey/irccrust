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
    
    pub fn add_channel(&mut self, name: String) {
        println!("Attempting to create channel: {}", name.clone());
        self.channels.insert(name.clone(), irc::channel::Channel::new(&name));
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

        // data being read from socket will be written into this.
        let mut line = String::new();

        let server = Arc::clone(&server_lock);
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut user: irc::User = irc::User{name: "".to_string()};
            loop {
                tokio::select! {

                    // This first select is for incoming data from the clients, it ingests it and
                    // makes modifications to the state of the server
                    result = reader.read_line(&mut line) => {
                        let mut msg_type: irc::commandf::IRCMessageType = irc::commandf::IRCMessageType::UNKNOWN;
                        let mut response = String::from("");
                        let messages = irc::commandf::message_decode(line.clone());

                        // Entering the locked section of the thread, this is where the server
                        // state will be mutated and worked on.
                        { 
                            let mut server = server.lock().unwrap();
                            if result.unwrap() == 0 {
                                break;
                            }
                            
                            println!("{}", line.clone());

                            // loop through the messages and decode them, update state accordingly
                            // pass the decoded messages to the transmit section
                            for msg in messages {
                                msg_type = msg.msg_type;
                                match msg_type {
                                    // this section should only match the message types that
                                    // directly modify the state? maybe? idk, just food for though.
                                    irc::commandf::IRCMessageType::NICK => {
                                        user.name= msg.component[0].clone();
                                        response = irc::commandf::server_client(&server.domain, irc::Response::RplWelcome, &user.name, &"Goodday!".to_string());
                                    }
                                    irc::commandf::IRCMessageType::JOIN => {
                                        let mut channel = match server.channels.get_mut(&msg.component[0].clone()) {
                                            Some(channel) => channel,
                                            None => {
                                                // add the channel
                                                server.add_channel(msg.component[0].clone());
                                                // we can be sure we added it now?
                                                server.channels.get_mut(&msg.component[0].clone()).unwrap()
                                            }

                                        };
                                        channel.add_user(user.name.clone());
                                        let names = channel.get_users().join(" ");
                                        response = irc::commandf::client_join(&names, &msg.component[0], &server.domain.clone());
                                    }
                                    irc::commandf::IRCMessageType::PRIVMSG  => {
                                        response = line.clone();
                                        let (channel_name, message) = irc::commandf::privmsg_decode(&response).unwrap();
                                        // This is gauranteed because can't send message if not in
                                        // channel?
                                        response = format!(":{} PRIVMSG {} {}", user.name.clone(), channel_name.clone(), message.clone());
                                        println!("{}", line);
                                    }                            
                                    _ => {
                                        response = "".to_string();
                                        println!("{}", line);
                                    }
                                }
                            }
                        } 
                        // release server lock

                        tx.send((msg_type, response.clone(), user.name.clone())).unwrap();
                    // this select is for outgoing messages from the server to the clients, this
                    // only holds the lock for a brief time to make a copy of the server state,
                    // this is then used for outgoing messages to the clients.
                    } result = rx.recv() => {
                        // this part should NEVER mutate the server -- this is for updating 
                        // updating all clients with current state of this biddy
                        let server_ : Server; 
                        {
                            server_ = server.lock().unwrap().clone();
                        }
                        let (mtype, msg, name) = result.unwrap();
                        let messages = irc::commandf::message_decode(msg.clone());
                        match mtype {
                            irc::commandf::IRCMessageType::NICK => {
                                if name == user.name.clone() {
                                    writer.write_all(&msg.as_bytes()).await.unwrap();
                                } 
                            }
                            irc::commandf::IRCMessageType::JOIN => {
                                // get the channel name from the message
                                let message = &messages[0];
                                let channel = &message.component.last().unwrap();
                                
                                println!("{} Attempting to join: {}", user.name.clone(), channel.clone());
                                let mut channel = server_.channels.get(channel.clone()).unwrap().clone();
                                // We need to do different things if we are, or are not the user
                                // sending the message, if we are the user sending the message we
                                // need to send back more data to show who's in the server and what
                                // not. if we aren't the user sending we just need to forward the
                                // message.
                                if name == user.name.clone() {
                                    let response = irc::commandf::client_join(&name.clone(), &channel.name.clone(), &server_.domain.clone());
                                    writer.write_all(&response.as_bytes()).await.unwrap();
                                } else if channel.get_users().contains(&user.name) {
                                    let response = irc::commandf::join_announce(&name.clone(), &channel.name.clone(), &server_.domain.clone());
                                    writer.write_all(&response.as_bytes()).await.unwrap();
                                } 
                            }
                            irc::commandf::IRCMessageType::PRIVMSG => {
                                println!("{}", msg.clone());
                                // We probbably /shouldn't/ be sending to all but whatevs. FIXME
                                if name != user.name.clone() {
                                    writer.write_all(msg.as_bytes()).await.unwrap();
                                }
                            }
                            // if we haven't implemented it do nothing :)
                            _ => {}
                        }
                    }
                }
            }
        });
    }
        
}

