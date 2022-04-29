use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Self {
            socket_addr,
        }
    }

    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);

        for stream in connection_listener.incoming() {
            let mut stream = stream.expect("Failed to connect");
            println!("Connection established");
            let mut read_buffer = [0; 2048];
            stream.read(&mut read_buffer).expect("Fail to read buffer.");
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).expect("Fail to read request").into();
            Router::route(req, &mut stream);
        }
    }
}