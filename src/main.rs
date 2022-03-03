extern crate ctrlc;
extern crate openssl;

mod application;
mod settings;
mod drivers;
mod controllers;

fn main() {
    let settings: drivers::Settings = settings::get_settings();
    let ctx: drivers::Context = drivers::Context::new(settings);
    application::run(&ctx);
}
