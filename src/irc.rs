pub struct User {
    pub name: String,
    //pub host: String,
    //pub localname: String,
    //pub server: String,
    //pub user: bool, // Defines between user and service
}


// Channel struct as defined by RFC 2811
pub mod channel {
    use crate::irc::User;
    use std::collections::HashMap;
    use std::collections::hash_map::IntoIter;
    pub struct Channel <'a>{
        pub users: HashMap<String, &'a User>,
        pub priv_users: HashMap<String, &'a User>,
        pub flag: Flags,
        pub name: String,
        pub topic: String,
        pub key: String, // probs should be hashed but *shrug*
    }
    
    pub struct Flags {
        pub anonymous: bool,
        pub invite_only: bool,
        pub moderated: bool,
        pub no_msg_from_client: bool,
        pub quiet: bool,
        pub secret: bool,
        pub private: bool,
        pub server_reop: bool,
        pub topic_settable: bool,
        pub has_channel_pass: bool,
        pub has_user_limit: bool,
        pub ban_mask: bool,
        pub has_exception_mask: bool,
        pub has_invitation_mask: bool
    }
    
    impl Flags{
        pub fn new() -> Flags {
            return Flags {
                anonymous: false,
                invite_only: false,
                moderated: false,
                no_msg_from_client: false,
                quiet: false,
                secret: false,
                private: false,
                server_reop: false,
                topic_settable: false,
                has_channel_pass: false,
                has_user_limit: false,
                ban_mask: false,
                has_exception_mask: false,
                has_invitation_mask: false 
            }
        }
    }

    // Channel methods 
    impl<'a> Channel<'a> {
        // add a user to this channel
        pub fn add_user(&mut self, user: &'a User) {
            // only add the user if they aren't in the list
            if self.users.get(&user.name).is_none() {
                self.users.insert(user.name.clone(), user);
            }
        }

        pub fn delete_user(&mut self, user: &'a User) {
            self.users.remove(&user.name);
        }

        // get_users takes no arguments, returns iterator where
        // each element of the iterator is a tuple with the order
        // (username, user_instance)
        pub fn get_users(&mut self) -> IntoIter<String, &'a User> {
            // create a clone of the users map, convert to iterator
            return self.users.clone().into_iter();
        }

        pub fn set_topic(&mut self, topic: String) {
            self.topic = topic;
        }

    }
}

