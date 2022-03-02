use ctrlc::set_handler;

extern crate ctrlc;
extern crate openssl;

mod application;
mod context;
mod drivers;
mod settings;

fn main() {
    let settings = settings::get_settings();
    let ctx = drivers::get_context(settings);
    set_handler(application::stop).expect("Error to set stop handler");
    application::run(ctx);
}
