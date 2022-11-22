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
            println!("new connection!\n{:?}\n\n", listener);
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let request_line = buf_reader.lines().next().unwrap().unwrap();
        println!("req_line:\n{:?}\n",request_line);
        let (status_line, filename) = Self::routing(request_line);

        let contents = fs::read_to_string(filename).unwrap();
        let lenght = contents.len();

        let response = format!("{status_line}\r\nContent-Lenght: {lenght}\r\n\r\n{contents}");
        println!("res:\n{:?}\n",response);
        stream.write_all(response.as_bytes()).unwrap();
    }

    fn routing(req_line: String) -> (&'static str, &'static str) {
        let index = String::from("GET / HTTP/1.1");
        let log_panel = String::from("GET /logs HTTP/1.1");
        let obsor_panel = String::from("GET /obsor HTTP/1.1");
        let scanners_panel = String::from("GET /scanners HTTP/1.1");
        let style = String::from("GET /styles/{cssfile} HTTP/1.1");


        //FIXME: через match было бы удобнее
        if req_line == index {
            ("HTTP/1.1 200 OK", "www/views/index.html")
        } else if req_line == log_panel {
            ("HTTP/1.1 200 OK", "www/views/logs.html")
        } else if req_line == obsor_panel {
            ("HTTP/1.1 200 OK", "www/views/obsor.html")
        } else if  req_line == scanners_panel {
            ("HTTP/1.1 200 OK", "www/views/scanners.html")
        }else if  req_line==style{
            ("HTTP/1.1 200 OK", "www/style/{cssfile}")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "www/views/404.html")
        }
    }
}
