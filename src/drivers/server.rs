use std::{net::TcpListener};

pub trait Listen {
    fn listen(&self);
}

pub struct Server ();

impl Listen for Server {
    fn listen(&self) {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        for stream in listener.incoming() {
            let mut _stream = stream.unwrap();
            println!("Connection established!");
        }
    }
}