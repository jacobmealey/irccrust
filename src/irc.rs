pub struct User {

}

// Channel struct as defined by RFC 2811
pub mod channel {
    use crate::irc::User;
    pub struct Channel {
        pub users: Vec<User>,
        pub priv_users: Vec<User>,
        pub flag: Vec<Flags>,
        pub name: String,
        pub topic: String,
        pub key: String, // probs should be hashed but *shrug*
    }
    
    pub struct Flags {
        anonymous: bool,
        invite_only: bool,
        moderated: bool,
        no_msg_from_client: bool,
        quiet: bool,
        secret: bool,
        private: bool,
        server_reop: bool,
        topic_settable: bool,
        has_channel_pass: bool,
        has_user_limit: bool,
        ban_mask: bool,
        has_exception_mask: bool,
        has_invitation_mask: bool
    }
}

