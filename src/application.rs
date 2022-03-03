use std::io::{Read, Write};

use ctrlc::set_handler;

use crate::controllers::handler;
use crate::drivers::tcp::{Listen, TCP};
use crate::drivers::Context;

pub fn run(context: &Context) {
    set_handler(TCP::stop).expect("Error to set stop handler");
    let handler = handler::create(context);
    context.server.listen(&move |mut stream| {
        let mut buffer: Vec<u8> = vec![];
        stream.read(&mut buffer).expect("Failed to read request");
        let response = handler(buffer);
        stream.write(&response).expect("Failed do send response");
    });
}
