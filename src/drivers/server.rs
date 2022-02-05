use std::{net::{TcpListener, TcpStream}, io::Error};
use super::context;

pub trait Listen {
    fn listen(&self);
    fn on_request(&self, stream: &Result<TcpStream, Error>);
}

pub struct Server {
    pub context: context::Context,
}

impl Listen for Server {

    fn on_request(&self, stream: &Result<TcpStream, Error>) {
        let mut _stream = stream.as_ref().unwrap();
        println!("Connection established!");
    }

    fn listen(&self) {
        let settings = &self.context.settings;
        let host = format!("{}:{}", &settings.host, &settings.port);
        let listener = TcpListener::bind(&host).unwrap();
        println!("Listening on: {}", &host);
        for stream in listener.incoming() {
            self.on_request(&stream);
        }
    }
}