extern crate openssl;
extern crate ctrlc;

mod application;
mod context;
mod drivers;
mod settings;

fn main() {
    let settings = settings::get_settings();
    let ctx = drivers::get_context(settings);
    application::run(ctx);
}
