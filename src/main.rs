mod application;
mod context;
mod drivers;
mod settings;

pub fn listen(settings: context::Settings) {
    let ctx = drivers::get_context(settings);
    let server = application::main(ctx);
    server();
}

fn main() {
    let settings = settings::get_settings();
    listen(settings);
}
