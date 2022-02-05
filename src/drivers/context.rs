pub struct Settings {
    pub host: String,
    pub port: String,
}

pub struct Context {
    pub settings: Settings,
}

pub fn get_context(settings: Settings) -> Context {
    Context {
        settings
    }
}