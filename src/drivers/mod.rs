use self::server::Listen;
pub mod context;
mod server;

pub fn listen(settings: context::Settings) {
    let context = context::get_context(settings);
    let server = server::Server { context: context };
    server.listen();
}