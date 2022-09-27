mod server;
use crate::server::Server;
use std::net::{IpAddr, Ipv4Addr};
fn main() {
    //FIXME: отредактивровать создание сервера
    let (a, b, c, d) = (127, 0, 0, 1);
    let server = Server {
        addr: IpAddr::V4(Ipv4Addr::new(a, b, c, d)),
        port: 7878,
    };
    Server::start_server(&server);
}
