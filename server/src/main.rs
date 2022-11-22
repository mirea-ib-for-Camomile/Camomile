mod server;
use server::{HttpRequest, HttpResponse};

use crate::server::Server;
use std::{
    net::TcpListener,
    sync::{mpsc, Arc, Mutex}, thread, io::{Read, Write}, collections::HashMap, path::Path, fs
};

fn index(request: HttpRequest) -> HttpResponse {
    println!("{:?}", request);
    HttpResponse::ok().body("Hello from the Index!")}
fn sleep(request: HttpRequest) -> HttpResponse {
    println!("{:?}", request);
    thread::sleep(std::time::Duration::from_secs(5));
    HttpResponse::ok().body("Hello from the sleep!")}

fn main() {
    let mut function_dictionary: HashMap<&str, Box<dyn Fn(HttpRequest) -> HttpResponse + Send + Sync>> = HashMap::new();    //function_dictionary.insert("/index", Box::new(index));

    let function_dictionary = Arc::new(function_dictionary);
    let ip_address = "0.0.0.0";
    let port = "7878";
    let socket = TcpListener::bind(format!("{}:{}", ip_address, port)).unwrap();
    println!("server run at {}:{}", ip_address, port);

    let number_of_thread = 10;
    let mut pool = Vec::new();

    let (sender, receiver) = mpsc::channel::<Box<dyn FnOnce() + Send>>();
    let receiver = Arc::new(Mutex::new(receiver));

    for i in 0..number_of_thread {
        let receiver = Arc::clone(&receiver);

        pool.push(thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Executing...{}", i);
            job();
        }));
    }

    for session in socket.incoming() {
        let function_dictionary = Arc::clone(&function_dictionary);
        let job = Box::new(|| {
            let mut session = session.unwrap();
            let mut buffer = vec![0;2048];
            session.read(&mut buffer).unwrap();

            let response = HttpRequest::handle(buffer, function_dictionary);

            session.write(response.data().as_bytes()).unwrap();
            session.flush().unwrap();
        });
        sender.send(job).unwrap();
    }
}
