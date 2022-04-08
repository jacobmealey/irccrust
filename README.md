# irccrust
IRCrust (I-R-crust) 

A dead simple IRC server written _for academia_ currently it isn't even compatible with the RFCs, but its a work in progress. 
Though the plan is to be compatible with [RFC 2810](https://datatracker.ietf.org/doc/html/rfc2810), the main objective is to 
get server-client relattionship working as outlined in [RFC 2812](https://datatracker.ietf.org/doc/html/rfc2812), and then go 
on to implement the server-server communication as outlined in [RFC 2813](https://datatracker.ietf.org/doc/html/rfc2813) at a 
later time. 

### why (??)
You may be asking yourself why someone would write a server for a protocol written 30 some-odd years ago, and is quickly being
replaced by things like Matrix... well Matrix is complicated and this is for a class so it needs to be remotely working by the
end of April 2022, so here we are. Also, I've never written a 'large' networked program like this. This project is an 
exploration of three topics:
  1. Writing network-y code (it's for a networking class)
  2. Exploring Rust, which is painful but the bird app says to learn it and I have no spine. 
  3. Multi-threading - most of my courses thus far have barely touched on threading and writing threaded programs so here are. 
