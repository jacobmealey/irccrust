// IRCrust: main.rs
// Author: Jacob Mealey <jacob.mealey@maine.edu>
// Main code for IRCrust (pronounced I-R-crust)
// a "simple" IRC server written in rust because
// I don't like being happy?

use std::io::prelude::*;
use std::thread;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use tokio::{io::{AsyncWriteExt, BufReader, AsyncBufReadExt}, net::TcpListener, sync::broadcast};
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

#[tokio::main]
async fn main() {
    // bind to address ADDR 
    let listener = match TcpListener::bind(ADDR).await {
        Ok(listener) => listener,
        Err(e) => {panic!("Error binding to TCP socket: {}", e);}
    };


    let server_lock = Arc::new(Mutex::new(Server::new(String::from("localhost")))); 

    
    let (tx, _rx) = broadcast::channel(10);

    // currently we are only listening to a single connection at 
    // a time, we /should/ open a new thread everytime we get a 
    // connection
    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        let mut line = String::new();
        
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }

                        tx.send((line.clone(), addr)).unwrap();
                    } result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();
                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
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
