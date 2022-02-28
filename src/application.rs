use crate::context::Settings;

use super::context::Context;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::vec::Vec;

pub fn main(context: Context) -> impl Fn() -> () {
    static mut CONNECTIONS: Vec<TcpStream> = Vec::new();
    static mut IS_LISTENING: bool = true;
    let settings: Settings = context.settings;

    fn handle_request(request: String) -> String {
        println!("{}", request);
        return String::from("olá mundo");
    }

    fn get_requests() {
        unsafe {
            while IS_LISTENING {
                let iter = CONNECTIONS.iter_mut();
                for connection in iter {
                    let mut buffer = String::new();
                    let _ = connection.read_to_string(&mut buffer);
                    let response = handle_request(buffer);
                    let _ = connection.write(response.as_bytes());
                }
            }
        }
    }

    fn add_connection(stream: TcpStream) {
        unsafe {
            CONNECTIONS.push(stream);
            let iter = CONNECTIONS.iter_mut();
            for connection in iter {
                let mut data = [0];
                let _ = connection.read(&mut data);
            }
        }
    }

    return move || {
        let host: String = format!("{}:{}", settings.host, settings.port);
        let listener = TcpListener::bind(&host).unwrap();
        thread::spawn(get_requests);
        println!("Listening on: {}", host);
        for stream in listener.incoming() {
            unsafe {
                if !IS_LISTENING {
                    break;
                }
            }
            match stream {
                Ok(stream) => {
                    add_connection(stream);
                }
                Err(error) => {
                    println!("Error to establish new connection - {:?}", error.kind())
                }
            }
        }
    };
}
