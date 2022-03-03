use crate::drivers::Settings;
use std::{
    io::Error,
    net::{TcpListener, TcpStream},
};

static mut IS_LISTENING: bool = true;

pub struct TCP {
    listener: TcpListener,
}

type Handler = dyn Fn(TcpStream) -> ();
type HandlerError = dyn Fn(Error) -> ();

pub trait Listen {
    fn listen(&self, handler: &Handler);
}

impl Listen for TCP {
    fn listen(&self, handler: &Handler) {
        println!(
            "Sever listening on: {}",
            self.listener.local_addr().expect("Failed to get host")
        );
        self.listener
            .set_nonblocking(true)
            .expect("Failed to set non blocking");
        self.event_loop(&|| {
            let request = self.listener.accept();
            match request {
                Ok((stream, _)) => handler(stream),
                Err(_) => (),
            }
        });
    }
}

impl TCP {
    pub fn new(settings: &Settings) -> TCP {
        let host: String = format!("{}:{}", &settings.host, &settings.port);
        let listener: TcpListener = TcpListener::bind(host).unwrap();
        return TCP { listener };
    }

    pub fn stop() {
        println!("Server stopping");
        unsafe {
            IS_LISTENING = false;
        }
    }

    fn event_loop(&self, callback: &impl Fn() -> ()) {
        while TCP::get_state() {
            callback();
        }
    }

    fn get_state() -> bool {
        unsafe {
            return IS_LISTENING;
        }
    }
}
