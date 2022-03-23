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
pub enum Responses {
    RPL_WELCOME=001,
    RPL_YOURHOST=002,
    RPL_CREATED=003,
    RPL_MYINFO=004,
    RPL_BOUNCE=005,
    RPL_USERHOST=302,
    RPL_ISON=303,
    RPL_AWAY=301,
    RPL_UNAWAY=305,
    RPL_NOWAWAY=306,
    RPL_WHOISUSER=311,
    RPL_WHOISOPERATOR=313,
    RPL_WHOISEIDLE=317,
    RPL_ENDOFWHOIS=318,
    RPL_WHOISCHANNELS=319,
    RPL_WHOWASUSER=314,
    RPL_ENDOFWHOWAS=369,
    RPL_LISTSTART=321,
    RPL_LIST=322,
    RPL_LISTEND=323,
    RPL_UNIQOPIS=325,
    RPL_CHANNELMODEIS=324,
    RPL_NOTOPIC=331,
    RPL_TOPIC=332,
    RPL_INVITING=341,
    RPL_SUMMONING=342,
    RPL_INVITELIST=346,
    RPL_ENDOFINVITELIST=347,
    RPL_EXCPTLIST=348,
    RPL_ENDOFEXCEPTLIST=349,
    RPL_VERSION=351,
    RPL_WHOREPLY=352,
    RPL_ENDOFWHO=315,
    RPL_NAMREPLY=353,
    RPL_ENDOFNAMES=366,
    RPL_LINKS=364,
    RPL_ENDOFLINKS=365,
    RPL_BANLIST=367,
    RPL_ENDOFBANLIST=368,
    RPL_INFO=371,
    RPL_ENDOFINFO=374,
    RPL_MOTDSTART=375,
    RPL_MOTD=372,
    RPL_ENDOFMOTD=376,
    RPL_YOUREOPER=381,
    RPL_REHASHING=382,
    RPL_YOURESERVICE=383,
    RPL_TIME=391,
    RPL_USERSSTART=392,
    RPL_USERS=393,
    RPL_ENDOFUSERS=394,
    RPL_NOUSERS=395,
    RPL_TRACELINK=200,
    RPL_TRACECONNECTING=201,
    RPL_TRACEHANDSHAKE=202,
    RPL_TRACEUNKNOWN=203,
    RPL_TRACEOPERATOR=204,
    RPL_TRACEUSER=205,
    RPL_TRACESERVER=206,
    RPL_TRACESERVICE=207,
    RPL_TRACENEWTYPE=208,
    RPL_TRACECLASS=209,
    RPL_TRACERECONNECT=210,
    RPL_TRACELOG=261,
    RPL_TRACEEND=262,
    RPL_STATSLINKINFO=211,
    RPL_STATSCOMMANDS=212,
    RPL_ENDOFSTATS=219,
    RPL_STATSUPTIME=242,
    RPL_STATSOLINE=243,
    RPL_UMODEIS=221,
    RPL_SERVLIST=234,
    RPL_SERVLISTEND=235,
    RPL_LUSERCLIENT=251,
    RPL_LUSEROP=252,
    RPL_LUSERUNKNOWN=253,
    RPL_LUSERCHANNELS=254,
    RPL_USERME=255,
    RPL_ADINME=256,
    RPL_ADMINLOC1=257,
    RPL_ADMINLOC2=258,
    RPL_ADMINEMAIL=259,
    RPL_TRYAGAIN=263,
}

pub mod commands {
    // registration -- generates the string to send when a new connection
    // is made.
    pub fn registration(host: &String, username: &String, message: &String) -> String {
        return format!(":{} 001 {} :{}!!!\n", host, username, message);
    }
}

