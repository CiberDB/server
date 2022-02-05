mod drivers;

fn main() {
    let settings = drivers::context::Settings {
        host: String::from("0.0.0.0"),
        port: String::from("7890"),
    };
    drivers::listen(settings);
}