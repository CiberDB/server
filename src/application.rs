use crate::context::Settings;

use super::context::Context;
use ctrlc::set_handler;
use openssl::pkey::Private;
use openssl::rsa::{Padding, Rsa};
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::vec::Vec;

struct Connection {
    stream: TcpStream,
    cert: Vec<u8>,
}

struct State {
    connections: Vec<Connection>,
    is_listening: bool,
}

fn handle_request(request: String) -> String {
    println!("{}", request);
    return String::from("olá mundo");
}

fn decrypt_request(mut stream: &TcpStream, _rsa: &Rsa<Private>) -> Result<String, String> {
    let mut buffer: Vec<u8> = vec![];
    let _ = stream.read(&mut buffer);
    if buffer.len() == 0 {
        return Err("Empty request".to_string());
    }
    // let mut request: Vec<u8> = vec![];
    // let padding = Padding::PKCS1;
    // rsa.public_decrypt(&buffer, &mut request, padding).unwrap();
    return Ok(String::from_utf8(buffer).unwrap());
}

fn encrypt_response(response: String, cert: &Vec<u8>) -> Vec<u8> {
    let mut response_bytes: Vec<u8> = vec![];
    let rsa = Rsa::public_key_from_pem(cert).unwrap();
    let padding = Padding::PKCS1;
    rsa.public_encrypt(response.as_bytes(), &mut response_bytes, padding)
        .expect("Failed to encrypt response");
    return response_bytes;
}

fn add_connection<'a>(
    mut stream: TcpStream,
    state: &'a mut State,
    cert: &'a Vec<u8>,
) -> Result<&'a mut State, String> {
    let mut client_cert: Vec<u8> = Vec::new();
    stream.read(&mut client_cert).unwrap();
    if client_cert.len() == 0 {
        stream.write("Invalid request".as_bytes()).err();
        return Err("Invalid certificate".to_string());
    }
    stream.write(&cert).unwrap();
    let connection = Connection {
        stream,
        cert: client_cert,
    };
    state.connections.push(connection);
    return Ok(state);
}

fn get_requests(state: &mut State, rsa: Rsa<Private>) {
    while state.is_listening {
        let iter = state.connections.iter();
        for connection in iter {
            let mut stream = &connection.stream;
            let request = decrypt_request(stream, &rsa);
            match request {
                Ok(request) => {
                    let response = handle_request(request);
                    let response_bytes = encrypt_response(response, &connection.cert);
                    let _ = stream.write(&response_bytes);
                }
                Err(_) => {
                    continue;
                }
            }
        }
    }
}

pub fn run(context: Context) {
    let settings: Settings = context.settings;
    let cert = settings.rsa.public_key_to_pem().unwrap();
    let host: String = format!("{}:{}", settings.host, settings.port);
    let listener = TcpListener::bind(&host).unwrap();
    static mut STATE: State = State {
        is_listening: true,
        connections: Vec::new(),
    };
    thread::spawn(move || unsafe {
        get_requests(&mut STATE, settings.rsa);
    });
    println!("Listening on: {}", host);
    for stream in listener.incoming() {
        unsafe {
            if !STATE.is_listening {
                break;
            }
        }
        match stream {
            Ok(stream) => unsafe {
                add_connection(stream, &mut STATE, &cert).err();
            },
            Err(error) => {
                println!("Error to establish new connection - {:?}", error.kind())
            }
        }
    }
}
