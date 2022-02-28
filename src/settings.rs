use super::context::Settings;

pub fn get_settings() -> Settings {
    let settings = Settings {
        host: String::from("0.0.0.0"),
        port: String::from("7890"),
    };
    return settings;
}
