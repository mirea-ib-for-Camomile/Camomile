use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream, SocketAddr, IpAddr},
};
pub struct Server{
    pub addr: IpAddr,
    pub port: u16,
}
impl Server{
    pub fn start_server(&self){
        let socket = SocketAddr::new(self.addr,self.port );


        let listener = TcpListener::bind(socket).unwrap();
        println!("server start on host {} and port {}", self.addr, self.port);

        for stream in listener.incoming(){
            let stream = stream.unwrap();
            Self::handle_connection(stream);
            println!("new connection!");
        }
    }
    
    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let request_line = buf_reader.lines().next().unwrap().unwrap();

        let (status_line, filename) = if request_line == "GET / HTTP/1.1"{
            ("HTTP/1.1 200 OK", "www/views/index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "www/views/404.html")
        };

        let contents = fs::read_to_string(filename).unwrap();
        let lenght = contents.len();

        let response = 
            format!("{status_line}\r\nContent-Lenght: {lenght}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    }
}