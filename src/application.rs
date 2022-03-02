use crate::context::Settings;

use super::context::Context;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use std::vec::Vec;

struct Connection {
    stream: TcpStream,
    should_close: bool,
}

struct State {
    connections: Vec<Connection>,
    is_listening: bool,
}

static mut STATE: State = State {
    is_listening: true,
    connections: vec![],
};

async fn handle_request(query: String) -> String {
    println!("{}", query);
    return String::from("Olá mundo");
}

async fn manage_requests(state: &mut State) {
    while state.is_listening {
        for connection in state.connections.iter_mut() {
            if !connection.should_close {
                let mut stream = &connection.stream;
                let err = stream.write(&[0]).is_err();
                if err {
                    connection.should_close = true;
                    continue;
                }
                let mut buffer = vec![];
                stream.read_to_end(&mut buffer).err();
                if buffer.len() > 0 {
                    let body = String::from_utf8(buffer).unwrap();
                    let response = handle_request(body).await;
                    let has_error = stream.write(response.as_bytes()).is_err();
                    stream.flush().expect("Failed to flush");
                    if has_error {
                        connection.should_close = true;
                    }
                }
            }
        }
    }
}

async fn clear_queue(state: &mut State) {
    state.connections.retain(|request| {
        if request.should_close {
            request.stream.shutdown(Shutdown::Both).unwrap();
            println!("Connection closed {}", request.stream.peer_addr().unwrap());
        }
        return !request.should_close;
    });
}

fn manage_connections(context: Context, state: &mut State) {
    let settings: Settings = context.settings;
    let host: String = format!("{}:{}", settings.host, settings.port);
    let listener = TcpListener::bind(&host).unwrap();
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");
    println!("Listening on: {}", host);
    let clearing_queue = tokio::runtime::Runtime::new().unwrap();
    while state.is_listening {
        clearing_queue.block_on(clear_queue(state));
        let incoming = listener.accept();
        match incoming {
            Ok((stream, _)) => {
                stream
                    .set_nonblocking(true)
                    .expect("Failed to set non blocking");
                state.connections.push(Connection {
                    stream,
                    should_close: false,
                });
                println!("New connection");
            }
            Err(_error) => continue,
        }
    }
}

pub fn run(context: Context) {
    unsafe {
        thread::spawn(|| {
            let checking_requests =
                tokio::runtime::Runtime::new().expect("Could not start manage requests");
            checking_requests.block_on(manage_requests(&mut STATE));
        });
        manage_connections(context, &mut STATE);
    }
}

pub fn stop() {
    unsafe {
        print!("\nStopping server\n");
        STATE.is_listening = false;
    }
}
