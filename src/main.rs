// IRCrust: main.rs
// Author: Jacob Mealey <jacob.mealey@maine.edu>
// Main code for IRCrust (pronounced I-R-crust)
// a crusty IRC server written in rust because

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::{io::{AsyncWriteExt, BufReader, AsyncBufReadExt}, net::TcpListener, sync::broadcast};
mod irc;

const ADDR: &str = "localhost:3030";

// The server structure has is passed around between different
// threads. it has 3 members, a hashmap of the channels where 
// the channel name is the key of the value is the channel object, 
// a hash map of the users where the users realname is the key
// and the user object is the value. and domain is the name of 
// the server.
#[derive(Clone)]
struct Server {
    pub channels: Box<HashMap::<String, irc::channel::Channel>>,
    pub users: Box<HashMap::<String, irc::User>>,
    pub domain: String,
}

impl Server {
    // create a new server instance
    pub fn new(host: String) -> Server {
        return Server {
            channels: Box::new(HashMap::<String, irc::channel::Channel>::new()),
            users: Box::new(HashMap::<String, irc::User>::new()),
            domain: host,
        }
    }
    
    // add a new empty channel 
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


    // thread safe locking of the server data
    let server_lock = Arc::new(Mutex::new(Server::new(String::from("localhost")))); 
    
    // creates channels to send data between threads 
    let (tx, _rx) = broadcast::channel(10);

    // This loops creates a new thread and keeps them alive as long as their is a 
    // connection everytime a new connection is made it spawns a new thread
    loop {
        // get socket and adress from the listener
        let (mut socket, _) = listener.accept().await.unwrap();

        // create clones of channels 
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        // data being read from socket will be written into this.
        let mut line = String::new();

        let server = Arc::clone(&server_lock);
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut user: irc::User = irc::User::new();
            loop {
                tokio::select! {

                    // This first select is for incoming data from the clients, it ingests it and
                    // makes modifications to the state of the server
                    _result = reader.read_line(&mut line) => {
                        let msg_type: irc::commandf::IRCMessageType;
                        let response: String;

                        println!("Incoming: {}", &line);

                        // Entering the locked section of the thread, this is where the server
                        // state will be mutated and worked on.
                        (response, msg_type) = handle_ingest(Arc::clone(&server), &line, &mut user);                         
                        // release server lock
                        tx.send((msg_type, response.clone(), user.realname.clone())).unwrap();
                    // this select is for outgoing messages from the server to the clients, this
                    // only holds the lock for a brief time to make a copy of the server state,
                    // this is then used for outgoing messages to the clients.
                    } result = rx.recv() => {
                        // this part should NEVER mutate the server -- this is for 
                        // updating all clients with current state of this biddy
                        let server_ : Server; 
                        {
                            server_ = server.lock().unwrap().clone();
                        }

                        let (mtype, msg, name) = result.unwrap();
                        let messages = irc::commandf::message_decode(msg.clone());
                        println!("Outgoing: {}", msg);
                        match mtype {
                            irc::commandf::IRCMessageType::USER => {
                                if name == user.realname.clone() {
                                    writer.write_all(&msg.as_bytes()).await.unwrap();
                                } 
                            }
                            irc::commandf::IRCMessageType::NICK => {
                                writer.write_all(&msg.as_bytes()).await.unwrap();
                            }
                            irc::commandf::IRCMessageType::JOIN => {
                                // get the channel name from the message
                                let message = &messages[0];
                                let channel = &message.component.last().unwrap();
                                
                                println!("{} Attempting to join: {}", user.nickname.clone(), channel.clone());
                                let mut channel = server_.channels.get(channel.clone()).unwrap().clone();
                                // We need to do different things if we are, or are not the user
                                // sending the message, if we are the user sending the message we
                                // need to send back more data to show who's in the server and what
                                // not. if we aren't the user sending we just need to forward the
                                // message.
                                if name == user.realname.clone() {
                                    let names = channel.get_users().join(" ");
                                    let response = irc::commandf::client_join(&user.nickname, &names, &channel.name.clone(), &server_.domain.clone());
                                    writer.write_all(&response.as_bytes()).await.unwrap();
                                } else if channel.get_users().contains(&user.nickname) {
                                    let response = irc::commandf::join_announce(&name, &channel.name.clone(), &server_.domain.clone());
                                    writer.write_all(&response.as_bytes()).await.unwrap();
                                } 
                            }
                            irc::commandf::IRCMessageType::PRIVMSG => {
                                println!("{}", msg.clone());
                                // We probbably /shouldn't/ be sending to all but whatevs. FIXME
                                if name != user.realname.clone() {
                                    writer.write_all(msg.as_bytes()).await.unwrap();
                                }
                            }
                            irc::commandf::IRCMessageType::TOPIC => {
                                writer.write_all(msg.as_bytes()).await.unwrap();
                            }
                            irc::commandf::IRCMessageType::QUIT => {
                                println!("Quiting...");
                                break;
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


// Handle ingest is a used to manipulate data coming in to the server 
// from different clients, it returns the response as a string and the
// message type. 
fn handle_ingest(server: Arc<Mutex<Server>>, line: &String, user: &mut irc::User) -> (String, irc::commandf::IRCMessageType) { 
    let mut server = server.lock().unwrap();
    let messages = irc::commandf::message_decode(line.clone());
    let mut response = String::from("");
    let mut mesgtype: irc::commandf::IRCMessageType = irc::commandf::IRCMessageType::UNKNOWN; 
    // loop through the messages and decode them, update state accordingly
    // pass the decoded messages to the transmit section
    for msg in messages {
        mesgtype = msg.msg_type;
        match mesgtype {
            // this section should only match the message types that
            // directly modify the state? maybe? idk, just food for though.
            irc::commandf::IRCMessageType::USER => {
                let realname= msg.component[0].clone();
                if server.users.contains_key(&realname.clone()) {
                    println!("USER already exists!");
                    response = irc::commandf::server_client(&server.domain,
                                                            irc::Response::RplErrAlreadyReg, &"".to_string(), 
                                                            &"User already registered".to_string());
                } else {
                    user.realname = realname.clone();
                    server.users.insert(user.realname.clone(), user.clone());
                    response = irc::commandf::server_client(&server.domain, 
                                                            irc::Response::RplWelcome, &user.nickname, 
                                                            &"Weclome to IRCrust!".to_string());
                }
            }
            irc::commandf::IRCMessageType::NICK => {
                let old_nick = user.nickname.clone();
                user.nickname = msg.component[0].clone();
                if old_nick.is_empty() {
                    response = format!(":NICK {}\n", &user.nickname);
                } else {
                    response = format!(":{} NICK {}\n", &old_nick, &user.nickname);
                }
            }
            irc::commandf::IRCMessageType::JOIN => {
                let channel = match server.channels.get_mut(&msg.component[0].clone()) {
                    Some(channel) => channel,
                    None => {
                        // add the channel
                        server.add_channel(msg.component[0].clone());
                        // we can be sure we added it now?
                        server.channels.get_mut(&msg.component[0].clone()).unwrap()
                    }

                };
                channel.add_user(user.nickname.clone());
                let names = channel.get_users().join(" ");
                response = irc::commandf::client_join(&user.nickname, &names, &msg.component[0], &server.domain.clone());
            }
            irc::commandf::IRCMessageType::PRIVMSG  => {
                response = line.clone();
                let (channel_name, message) = irc::commandf::privmsg_decode(&response).unwrap();
                // This is gauranteed because can't send message if not in
                // channel?
                response = format!(":{} PRIVMSG {} {}", user.nickname.clone(), channel_name.clone(), message.clone());
                println!("{}", line);
            }                            
            irc::commandf::IRCMessageType::TOPIC => {
                println!("{}", &msg.component[0]);
                let topic_split: Vec<&str> = line.split(":").collect();
                let topic = topic_split[1];
                let channel = msg.component[0].clone();
                response = format!(":{} TOPIC {} :{}", &user.nickname, &channel, &topic);
            }
            irc::commandf::IRCMessageType::QUIT => {
                server.users.remove(&user.realname);
            }

            _ => {}
        }
    }
    return (response, mesgtype);
}
