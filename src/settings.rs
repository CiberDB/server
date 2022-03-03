use super::drivers::Settings;
use openssl::rsa::Rsa;

pub fn get_settings() -> Settings {
    let settings: Settings = Settings {
        host: String::from("0.0.0.0"),
        port: String::from("7890"),
        rsa: Rsa::generate(2048).unwrap(),
    };
    return settings;
}
