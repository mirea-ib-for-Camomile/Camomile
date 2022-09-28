use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{IpAddr, TcpListener, TcpStream},
};
pub struct Server {
    pub addr: IpAddr,
    pub port: u16,
}
impl Server {
    pub fn start_server() {
        let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
        println!("server started");

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            Self::handle_connection(stream);
            println!("new connection!");
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let request_line = buf_reader.lines().next().unwrap().unwrap();

        let (status_line, filename) = Self::routing(request_line);

        let contents = fs::read_to_string(filename).unwrap();
        let lenght = contents.len();

        let response = format!("{status_line}\r\nContent-Lenght: {lenght}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    }

    fn routing(req_line: String) -> (&'static str, &'static str) {
        let index = String::from("GET / HTTP/1.1");
        let admin_panel = String::from("GET /admin HTTP/1.1");
        let personal_cabinet = String::from("GET /personal HTTP/1.1");

        //FIXME: через match было бы удобнее

        if req_line == index {
            ("HTTP/1.1 200 OK", "www/views/index.html")
        } else if req_line == admin_panel {
            ("HTTP/1.1 200 OK", "www/views/admin.html")
        } else if req_line == personal_cabinet {
            ("HTTP/1.1 200 OK", "www/views/personal.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "www/views/404.html")
        }
    }
}
