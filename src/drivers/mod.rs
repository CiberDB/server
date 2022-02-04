use self::server::Listen;

mod server;

pub fn listen() {
    let server = server::Server();
    server.listen();
}