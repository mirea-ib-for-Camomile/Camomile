mod server;
use std::net::{IpAddr, Ipv4Addr};
use crate::server::Server;
fn main() {
    let (a,b,c,d) = (127,0,0,1);
    let server = Server{
        addr: IpAddr::V4(Ipv4Addr::new(a,b,c,d)),
        port: 7878,
    };
    Server::start_server(&server);
}

