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
                            
                            println!("{}", line.clone());

                            for msg in messages {
                                msg_type = msg.msg_type;
                                match msg_type {
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
                                let message = &messages[0];
                                let channel = &message.component.last().unwrap();
                                
                                println!("{} Attempting to join: {}", user.name.clone(), channel.clone());
                                let mut channel = server_.channels.get(channel.clone()).unwrap().clone();
                                if name == user.name.clone() {
                                    writer.write_all(&msg.as_bytes()).await.unwrap();
                                }
                                if channel.get_users().contains(&user.name) {
                                    let response = irc::commandf::join_announce(&name.clone(), &channel.name.clone(), &server_.domain.clone());
                                    writer.write_all(&response.as_bytes()).await.unwrap();
                                } 
                            }
                            irc::commandf::IRCMessageType::PRIVMSG => {
                                println!("{}", msg.clone());

                                if name != user.name.clone() {
                                    writer.write_all(msg.as_bytes()).await.unwrap();
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        });
    }
        
}

