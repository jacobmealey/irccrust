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

#[repr(u32)]
// if the enum value isn't used and 
#[allow(dead_code)]
pub enum Response {
    RplWelcome=001,
    RplYourhosT=002,
    RplCreated=003,
    RplMyinfo=004,
    RplBounce=005,
    RplUserhosT=302,
    RplIson=303,
    RplAway=301,
    RplUnaway=305,
    RplNowaway=306,
    RplWhoisusER=311,
    RplWhoisoperator=313,
    RplWhoiseidle=317,
    RplEndofwhois=318,
    RplWhoischannels=319,
    RplWhowasuser=314,
    RplEndofwhowas=369,
    RplListstart=321,
    RplList=322,
    RplListend=323,
    RplUniqopis=325,
    RplChannelmodeis=324,
    RplNotopic=331,
    RplTopic=332,
    RplInviting=341,
    RplSummoning=342,
    RplInvitelist=346,
    RplEndofinviteliST=347,
    RplExcptlist=348,
    RplEndofexceptliST=349,
    RplVersion=351,
    RplWhoreply=352,
    RplEndofwho=315,
    RplNamreply=353,
    RplEndofnames=366,
    RplLinks=364,
    RplEndoflinks=365,
    RplBanlist=367,
    RplEndofbanlist=368,
    RplInfo=371,
    RplEndofinfo=374,
    RplMotdstart=375,
    RplMotd=372,
    RplEndofmotd=376,
    RplYoureoper=381,
    RplRehashing=382,
    RplYoureservice=383,
    RplTime=391,
    RplUsersstart=392,
    RplUsers=393,
    RplEndofusers=394,
    RplNousers=395,
    RplTracelink=200,
    RplTraceconnecting=201,
    RplTracehandshake=202,
    RplTraceunknown=203,
    RplTraceoperator=204,
    RplTraceuser=205,
    RplTraceserver=206,
    RplTraceservice=207,
    RplTracenewtype=208,
    RplTraceclass=209,
    RplTracereconnect=210,
    RplTracelog=261,
    RplTraceend=262,
    RplStatslinkinfo=211,
    RplStatscommands=212,
    RplEndofstats=219,
    RplStatsuptime=242,
    RplStatsoline=243,
    RplUmodeis=221,
    RplServlist=234,
    RplServlistend=235,
    RplLuserclient=251,
    RplLuserop=252,
    RplLuserunknown=253,
    RplLuserchannels=254,
    RplUserme=255,
    RplAdinme=256,
    RplAdminloc1=257,
    RplAdminloc2=258,
    RplAdminemail=259,
    RplTryagain=263,
}

pub mod commandf {
    // registration -- generates the string to send when a new connection
    // is made.
    use crate::irc::Response;
    pub fn server_client(host: &String, numeric: Response, username: &String, message: &String) -> String {
        return format!(":{} {:0>3} {} :{}!!!\n", host, numeric as u32, username, message);
    }
}

